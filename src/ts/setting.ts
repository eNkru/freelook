// Settings page TypeScript - Tauri version (no jQuery, no Semantic UI)
export {};
const { invoke } = window.__TAURI__.core;

document.addEventListener("DOMContentLoaded", async () => {
    try {
        await loadSettings();
    } catch (err) {
        console.error("Failed to load settings:", err);
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
}

function setupListeners() {
    bindChange("zoomLevel", "zoomFactor");
    bindChange("verticalAdsClass", "verticalAdsClass");
    bindChange("smallAdsClass", "smallAdsClass");
    bindChange("premiumAdsClass", "premiumAdsClass");
    bindChange("unreadClass", "unreadMessageClass");
    bindChange("homepageUrl", "homepageUrl");

    document.getElementById("windowReset")!.addEventListener("click", async () => {
        try {
            await invoke("reset_window_frame");
        } catch (err) {
            console.error("Failed to reset window:", err);
        }
    });

    document.getElementById("saveRestart")!.addEventListener("click", async () => {
        try {
            await invoke("restart_app");
        } catch (err) {
            console.error("Failed to restart:", err);
        }
    });
}

function bindChange(elementId: string, configKey: string) {
    const el = document.getElementById(elementId) as HTMLInputElement | HTMLSelectElement;
    if (!el) return;
    el.addEventListener("change", async () => {
        try {
            await invoke("set_config", { key: configKey, value: el.value });
        } catch (err) {
            console.error(`Failed to save ${configKey}:`, err);
        }
    });
}
