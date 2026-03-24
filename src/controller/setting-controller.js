const path = require('path');
const { BrowserWindow,ipcMain } = require('electron');

class SettingsWindow {

    constructor(config) {
        this.config = config;
        this.init();
    }

    init() {
        this.window = new BrowserWindow({
            width: 600,
            height: 800,
            autoHideMenuBar: true,
            show: false,
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
            }
        });
        this.window.loadURL(`file://${path.join(__dirname, '../view/setting.html')}`);
        this.window.on('close', (e) => {
            if (this.window.isVisible()) {
                e.preventDefault();
                this.window.hide();
            }
        });

        if (!ipcMain.listenerCount("getConfig")) {
            ipcMain.on("getConfig", (event, key, defaultValue) => {
                event.returnValue = this.config.get(key, defaultValue);
            });
        }

        if (!ipcMain.listenerCount("getConfigs")) {
            ipcMain.on("getConfigs", (event) => {
                event.returnValue = this.config.store;
            });
        }

        if (!ipcMain.listenerCount("setConfig")) {
            ipcMain.on("setConfig", (event, key, value) => {
                this.config.set(key, value);
            });
        }

        if (!ipcMain.listenerCount("deleteConfig")) {
            ipcMain.on("deleteConfig", (event, key) => {
                this.config.delete(key);
            });
        }

        if (!ipcMain.listenerCount("restartApp")) {
            ipcMain.on("restartApp", () => {
                const { app } = require('electron');
                app.relaunch();
                app.exit(0);
            });
        }
    }

    show() {
        this.window.show();
    }
}

module.exports = SettingsWindow;