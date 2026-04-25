export {};

const { invoke } = window.__TAURI__.core;
const STATUS_CLASSES = ["status-idle", "status-saving", "status-success", "status-error"];
let statusResetTimer: number | undefined;

document.addEventListener("DOMContentLoaded", async () => {
    try {
        await loadSettings();
        setStatus("Ready", "Changes are saved automatically. Restart to apply everything cleanly.", "idle");
    } catch (err) {
        console.error("Failed to load settings:", err);
        setStatus("Load issue", "Some values could not be loaded. You can still review and update them manually.", "error");
    }

    setupListeners();
});

async function loadSettings() {
    const zoomLevel = document.getElementById("zoomLevel") as HTMLInputElement;
    const verticalAds = document.getElementById("verticalAdsClass") as HTMLInputElement;
    const smallAds = document.getElementById("smallAdsClass") as HTMLInputElement;
    const premiumAds = document.getElementById("premiumAdsClass") as HTMLInputElement;
    const unreadClass = document.getElementById("unreadClass") as HTMLInputElement;
    const homepageUrl = document.getElementById("homepageUrl") as HTMLSelectElement;

    zoomLevel.value = await invoke<string>("get_config", { key: "zoomFactor", default: "1.0" });
    verticalAds.value = await invoke<string>("get_config", { key: "verticalAdsClass", default: "pBKjV" });
    smallAds.value = await invoke<string>("get_config", { key: "smallAdsClass", default: "X1Kvq" });
    premiumAds.value = await invoke<string>("get_config", { key: "premiumAdsClass", default: "VPtFl" });
    unreadClass.value = await invoke<string>("get_config", { key: "unreadMessageClass", default: "" });
    homepageUrl.value = await invoke<string>("get_config", { key: "homepageUrl", default: "https://outlook.live.com/mail" });

    syncZoomPreview(zoomLevel.value);
}

function setupListeners() {
    bindChange("zoomLevel", "zoomFactor", value => syncZoomPreview(value));
    bindChange("verticalAdsClass", "verticalAdsClass");
    bindChange("smallAdsClass", "smallAdsClass");
    bindChange("premiumAdsClass", "premiumAdsClass");
    bindChange("unreadClass", "unreadMessageClass");
    bindChange("homepageUrl", "homepageUrl");

    const zoomLevel = document.getElementById("zoomLevel") as HTMLInputElement;
    zoomLevel.addEventListener("input", () => syncZoomPreview(zoomLevel.value));

    document.getElementById("windowReset")!.addEventListener("click", async () => {
        const button = document.getElementById("windowReset") as HTMLButtonElement;
        setBusy(button, true, "Resetting...");
        setStatus("Working", "Resetting window size and position...", "saving", false);

        try {
            await invoke("reset_window_frame");
            setStatus("Window reset", "Window size and position have been restored.", "success");
        } catch (err) {
            console.error("Failed to reset window:", err);
            setStatus("Reset failed", "The window frame could not be reset. Please try again.", "error");
        } finally {
            setBusy(button, false, "Reset window");
        }
    });

    document.getElementById("saveRestart")!.addEventListener("click", async () => {
        const button = document.getElementById("saveRestart") as HTMLButtonElement;
        setBusy(button, true, "Restarting...");
        setStatus("Restarting", "Applying your latest settings and restarting the app...", "saving", false);

        try {
            await invoke("restart_app");
        } catch (err) {
            console.error("Failed to restart:", err);
            setBusy(button, false, "Save & Restart");
            setStatus("Restart failed", "The app could not restart automatically. Your saved values are still kept.", "error");
        }
    });
}

function bindChange(
    elementId: string,
    configKey: string,
    afterSave?: (value: string) => void,
) {
    const el = document.getElementById(elementId) as HTMLInputElement | HTMLSelectElement | null;
    if (!el) return;

    el.addEventListener("change", async () => {
        setStatus("Saving", `Updating ${formatConfigLabel(configKey)}...`, "saving", false);

        try {
            await invoke("set_config", { key: configKey, value: el.value });
            afterSave?.(el.value);
            setStatus("Saved", `${formatConfigLabel(configKey)} updated successfully.`, "success");
        } catch (err) {
            console.error(`Failed to save ${configKey}:`, err);
            setStatus("Save failed", `Could not update ${formatConfigLabel(configKey)}.`, "error");
        }
    });
}

function syncZoomPreview(value: string) {
    const zoomPreview = document.getElementById("zoomPreview");
    if (!zoomPreview) return;

    const parsedValue = Number.parseFloat(value);
    const percentage = Number.isFinite(parsedValue) ? Math.round(parsedValue * 100) : 100;
    zoomPreview.textContent = `${percentage}%`;
}

function setStatus(title: string, message: string, tone: "idle" | "saving" | "success" | "error", autoReset = true) {
    const statusBadge = document.getElementById("statusBadge");
    const statusMessage = document.getElementById("statusMessage");
    if (!statusBadge || !statusMessage) return;

    statusBadge.textContent = title;
    statusBadge.classList.remove(...STATUS_CLASSES);
    statusBadge.classList.add(`status-${tone}`);
    statusMessage.textContent = message;

    if (statusResetTimer) {
        window.clearTimeout(statusResetTimer);
        statusResetTimer = undefined;
    }

    if (autoReset && tone !== "idle") {
        statusResetTimer = window.setTimeout(() => {
            setStatus("Ready", "Changes are saved automatically. Restart to apply everything cleanly.", "idle", false);
        }, 2600);
    }
}

function setBusy(button: HTMLButtonElement, busy: boolean, label: string) {
    button.disabled = busy;
    button.textContent = label;
}

function formatConfigLabel(configKey: string) {
    switch (configKey) {
        case "zoomFactor":
            return "zoom level";
        case "verticalAdsClass":
            return "vertical ads class";
        case "smallAdsClass":
            return "small ads class";
        case "premiumAdsClass":
            return "premium upgrade class";
        case "unreadMessageClass":
            return "unread message marker";
        case "homepageUrl":
            return "home URL";
        default:
            return "setting";
    }
}
