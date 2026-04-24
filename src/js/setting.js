// Settings page - Tauri version (no jQuery, no Semantic UI)
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
    const zoomLevel = document.getElementById("zoomLevel");
    const verticalAds = document.getElementById("verticalAdsClass");
    const smallAds = document.getElementById("smallAdsClass");
    const premiumAds = document.getElementById("premiumAdsClass");
    const unreadClass = document.getElementById("unreadClass");
    const homepageUrl = document.getElementById("homepageUrl");

    zoomLevel.value = await invoke("get_config", { key: "zoomFactor", default: "1.0" });
    verticalAds.value = await invoke("get_config", { key: "verticalAdsClass", default: "pBKjV" });
    smallAds.value = await invoke("get_config", { key: "smallAdsClass", default: "X1Kvq" });
    premiumAds.value = await invoke("get_config", { key: "premiumAdsClass", default: "VPtFl" });
    unreadClass.value = await invoke("get_config", { key: "unreadMessageClass", default: "" });
    homepageUrl.value = await invoke("get_config", { key: "homepageUrl", default: "https://outlook.live.com/mail" });
}

function setupListeners() {
    // Persist on change
    bindChange("zoomLevel", "zoomFactor");
    bindChange("verticalAdsClass", "verticalAdsClass");
    bindChange("smallAdsClass", "smallAdsClass");
    bindChange("premiumAdsClass", "premiumAdsClass");
    bindChange("unreadClass", "unreadMessageClass");
    bindChange("homepageUrl", "homepageUrl");

    // Window reset
    document.getElementById("windowReset").addEventListener("click", async () => {
        try {
            await invoke("reset_window_frame");
        } catch (err) {
            console.error("Failed to reset window:", err);
        }
    });

    // Save & Restart
    document.getElementById("saveRestart").addEventListener("click", async () => {
        try {
            await invoke("restart_app");
        } catch (err) {
            console.error("Failed to restart:", err);
        }
    });
}

function bindChange(elementId, configKey) {
    const el = document.getElementById(elementId);
    if (!el) return;
    el.addEventListener("change", async () => {
        try {
            await invoke("set_config", { key: configKey, value: el.value });
        } catch (err) {
            console.error(`Failed to save ${configKey}:`, err);
        }
    });
}
