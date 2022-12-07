process.once("loaded", () => {
    global.electron = require("electron")
    electron.webFrame.setZoomFactor(1.2)
})