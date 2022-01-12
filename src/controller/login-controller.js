const path = require('path');
const { BrowserWindow, ipcMain } = require('electron');

class LoginController {

    constructor() {
        this.init();
    }

    init() {
        const width = 300;
        const height = 150;
        this.window = new BrowserWindow({
            width,
            height,
            maxWidth: width,
            maxHeight: height,
            modal: true,
            alwaysOnTop: true,
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

    async login(parent) {
        this.window.setParentWindow(parent);
        setWindowCenterPosition(this.window, ...getWindowCenterPosition(parent));
        this.window.show();
        parent.setEnabled(false);
        try {
            return await new Promise((resolve) => { this.resolve = resolve });
        } finally {
            parent.setEnabled(true);
        }
    }
}

function getWindowCenterPosition(win) {
    const [x, y] = win.getPosition();
    const [width, height] = win.getSize();
    return [x + width / 2, y + height / 2];
}

function setWindowCenterPosition(win, x, y) {
    const [width, height] = win.getSize();
    win.setPosition(Math.trunc(x - width / 2), Math.trunc(y - height / 2));
}

module.exports = LoginController;
