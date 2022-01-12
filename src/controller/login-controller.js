const path = require('path');
const { BrowserWindow, ipcMain } = require('electron');

class LoginController {

    constructor() {
        this.init();
    }

    init() {
        this.window = new BrowserWindow({
            width: 300,
            height: 150,
            autoHideMenuBar: true,
            show: false,
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
            }
        });
        this.window.loadURL(`file://${path.join(__dirname, '../view/login.html')}`);
        this.window.on('close', (e) => {
            if (this.window.isVisible()) {
                e.preventDefault();
                this.window.hide();
                this.resolve({});
            }
        });
        this.resolve = () => {};

        ipcMain.on('login', (event, credentials) => {
            this.window.hide();
            this.resolve(credentials);
        });
    }

    login() {
        this.window.show();
        return new Promise((resolve) => { this.resolve = resolve });
    }
}

module.exports = LoginController;
