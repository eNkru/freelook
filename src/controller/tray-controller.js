const { app, Tray, nativeImage, Menu, ipcMain } = require('electron');
const settings = require('electron-settings');
const path = require('path');
const SettingsController = require('./setting-controller');

const macOS = process.platform === 'darwin';

class TrayController {
    constructor(mailController) {
        this.mailController = mailController;
        this.settingController = new SettingsController();
        this.init()
    }

    init() {
        this.tray = new Tray(this.createTrayIcon(''));

        const context = Menu.buildFromTemplate([
            {label: 'Open', click: () => this.showWindow()},
            {label: 'Separator', type: 'separator'},
            // {label: 'Window Frame', type: 'checkbox', checked: settings.get('showWindowFrame', true), click: () => this.toggleWindowFrame()},
            {label: 'Settings', click: () => this.openSettings()},
            {label: 'Quit', click: () => this.cleanupAndQuit()}
        ]);

        this.tray.setContextMenu(context);

        this.tray.on('click', () => this.fireClickEvent());

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
            iconPath = value ? '../../assets/outlook_linux_unread.png' : '../../assets/outlook_linux_black.png';
            return nativeImage.createFromPath(path.join(__dirname, iconPath))
        }
    }

    fireClickEvent() {
        this.mailController.toggleWindow()
    }

    showWindow() {
        this.mailController.show();
    }

    toggleWindowFrame() {
        settings.set('showWindowFrame', !settings.get('showWindowFrame'));
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