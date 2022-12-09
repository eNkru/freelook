const { ipcRenderer } = require("electron")

process.once("loaded", async () => {
    const zoomFactor = await ipcRenderer.sendSync("getConfig", "zoomFactor", "1.0")
    global.electron = require("electron")
    electron.webFrame.setZoomFactor(Number.parseFloat(zoomFactor))
})
