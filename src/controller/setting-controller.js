const path = require('path');
const { BrowserWindow,ipcMain } = require('electron');

class SettingsWindow {

    constructor(config) {
        this.config = config;
        this.init();
    }

    init() {
        this.window = new BrowserWindow({
            width: 500,
            height: 750,
            autoHideMenuBar: true,
            show: false,
            webPreferences: {
                nodeIntegration: true,
                enableRemoteModule: true
            }
        });
        this.window.loadURL(`file://${path.join(__dirname, '../view/setting.html')}`);
        this.window.on('close', (e) => {
            if (this.window.isVisible()) {
                e.preventDefault();
                this.window.hide();
            }
        });

        ipcMain.on("getConfig", (event, key, defaultValue) => {
            event.returnValue = this.config.get(key, defaultValue);
        });

        ipcMain.on("getConfigs", (event) => {
            event.returnValue = this.config.store;
        });

        ipcMain.on("setConfig", (event, key, value) => {
            this.config.set(key, value);
        });

        ipcMain.on("deleteConfig", (event, key) => {
            this.config.delete(key);
        });
    }

    show() {
        this.window.show();
    }
}

module.exports = SettingsWindow;