const { app, Tray, nativeImage, Menu, ipcMain } = require('electron');
const path = require('path');
const SettingsController = require('./setting-controller');

const macOS = process.platform === 'darwin';
const appIconPath = path.join(__dirname, '../../build/icons/512x512.png');

class TrayController {
    constructor(mailController,config) {
        this.config = config;
        this.mailController = mailController;
        this.settingController = new SettingsController(config);
        this.init()
    }

    init() {
        this.tray = new Tray(this.createTrayIcon(''));
        this.tray.setIgnoreDoubleClickEvents(true);

        const context = Menu.buildFromTemplate([
            {label: 'Open', click: () => this.showWindow()},
            {label: 'Separator', type: 'separator'},
            // {label: 'Window Frame', type: 'checkbox', checked: this.config.get('showWindowFrame', true), click: () => this.toggleWindowFrame()},
            {label: 'Settings', click: () => this.openSettings()},
            {label: 'Quit', click: () => this.cleanupAndQuit()}
        ]);

        this.tray.setContextMenu(context);
        this.tray.on('click', () => this.fireClickEvent());
        // Special click handle for macOS as this event is only for macOS.
        // This will fix the unreliable click event in macOS.
        this.tray.on('mouse-down', () => this.fireMouseDownEvent());

        ipcMain.on('updateUnread', (event, value) => {
            this.tray.setImage(this.createTrayIcon(value))
        })
    }

    createTrayIcon(value) {

        let iconPath;
        if (macOS) {
            iconPath = value ? '../../assets/outlook_macOS_unread.png' : '../../assets/outlook_macOS.png';
            let trayIcon = nativeImage.createFromPath(path.join(__dirname, iconPath));
            trayIcon.setTemplateImage(true);
            return trayIcon
        } else {
            iconPath = value ? path.join(__dirname, '../../assets/outlook_linux_unread.png') : appIconPath;
            return nativeImage.createFromPath(iconPath)
        }
    }

    fireClickEvent() {
        if(!macOS) {
            this.mailController.show();
        }
    }

    fireMouseDownEvent() {
        if(macOS) {
            this.mailController.show();
        }
    }

    showWindow() {
        this.mailController.show();
    }

    toggleWindowFrame() {
        this.config.set('showWindowFrame', !this.config.get('showWindowFrame',false));
        this.mailController.win.destroy();
        this.mailController.init()
    }

    openSettings() {
        this.settingController.show()
    }

    cleanupAndQuit() {
        app.exit(0)
    }
}

module.exports = TrayController;
