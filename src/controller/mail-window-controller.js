const { app, BrowserWindow, shell, ipcMain, Notification } = require('electron');
const settings = require('electron-settings');
const CssInjector = require('../js/css-injector');
const path = require('path');
const fs = require('fs-extra');
const isOnline = require('is-online');

const settingsExist = fs.existsSync(`${app.getPath('userData')}/Settings`);
const homepageUrl = settingsExist ? settings.get('homepageUrl', 'https://outlook.live.com/mail') : 'https://outlook.live.com/mail';
const deeplinkUrls = ['outlook.live.com/mail/deeplink', 'outlook.office365.com/mail/deeplink', 'outlook.office.com/mail/deeplink'];
const outlookUrls = ['outlook.live.com', 'outlook.office365.com', 'outlook.office.com'];

class MailWindowController {

    // Current displayed notifications.
    notifications = [];
    // Notification handle
    notificationHandle = undefined;

    constructor() {
        this.initSplash();
        setTimeout(() => this.connectToMicrosoft(), 1000);
    }

    init() {
        // Get configurations.
        const showWindowFrame = settings.get('showWindowFrame', true);

        // Create the browser window.
        this.win = new BrowserWindow({
            x: 100,
            y: 100,
            width: 1400,
            height: 900,
            frame: showWindowFrame,
            autoHideMenuBar: true,
            show: false,
            icon: path.join(__dirname, '../../assets/outlook_linux_black.png'),
            webPreferences: {
                nodeIntegration: true,
                preload: path.join(__dirname, '../js/preload.js')
            }
        });

        this.win.webContents.setUserAgent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3831.6 Safari/537.36");

        // and load the index.html of the app.
        this.win.loadURL(homepageUrl);

        // Show window handler
        ipcMain.on('show', () => {
            this.show()
        });

        // Show notification handler
        ipcMain.on('showNotification', (event, data) => {
            this.showNotifications(data);
        });

        // insert styles
        this.win.webContents.on('dom-ready', (event) => {
            this.win.webContents.insertCSS(CssInjector.main);
            if (!showWindowFrame) this.win.webContents.insertCSS(CssInjector.noFrame);

            event.sender.send('registerCalloutObserver');

            this.win.show()
        });

        // prevent the app quit, hide the window instead.
        this.win.on('close', (e) => {
            if (this.win.isVisible()) {
                e.preventDefault();
                this.win.hide()
            }
        });

        // Emitted when the window is closed.
        this.win.on('closed', () => {
            if (this.notificationHandle) {
                this.notificationHandle.close();
            }
            // Dereference the window object, usually you would store windows
            // in an array if your app supports multi windows, this is the time
            // when you should delete the corresponding element.
            this.win = null
        });

        // Open the new window in external browser
        this.win.webContents.on('new-window', this.openInBrowser)
    }

    showNotifications(data) {

        if (data) {
            this.notifications.push(data);

            if (this.notificationHandle) {
                this.notificationHandle.close();
            }

            let emails = 0;
            let reminder = 0;
            for (const n of this.notifications) {
                if (n.type == "reminder") {
                    reminder++;
                } else {
                    emails++;
                }
            }

            let title = ""
            if (emails > 1) {
                title = emails + " new mails"
            } else if (emails === 1) {
                title = emails + " new mail"
            }

            if (title !== "") {
                title = title + ", "
            }
            if (reminder > 0) {
                title = title + reminder + " new reminder"
            }

            this.notificationHandle = new Notification({
                title,
                body: this.notifications.map((n, i) => {
                    if (n.type === "email") {
                        return "Email from " + n.data.address + ": " + n.data.subject;
                    } else if (n.type === "reminder") {
                        return "Reminder: " + n.data.text + " (" + n.data.time + ")";
                    }
                }).join("\n"),
                timeoutType: settings.get('notificationTimeout', 'default'),
                icon: "assets/outlook_linux_black.png",
                urgency: "normal",

            });


            this.notificationHandle.on("click", () => {
                this.notifications = [];
                this.show();
            });

            this.notificationHandle.on("close", () => {
                this.notifications = [];
                this.notificationHandle = undefined;
            });
            this.notificationHandle.show();
        }
    }

    toggleWindow() {
        if (this.win) {
            if (this.win.isFocused()) {
                this.win.hide()
            } else {
                this.show()
            }
        }
    }

    openInBrowser(e, url) {
        // console.log(url);
        if (new RegExp(deeplinkUrls.join('|')).test(url)) {
            // Default action - if the user wants to open mail in a new window - let them.
        }

        // Disable the logic to load calendar contact and tasks in the election window.
        // Calendar has no link to back to mail. Once switch the window to calendar no way to back to mail unless close the app.

        // else if (new RegExp(outlookUrls.join('|')).test(url)) {
        //     // Open calendar, contacts and tasks in the same window
        //     e.preventDefault();
        //     this.loadURL(url)
        // }
        else {
            // Send everything else to the browser
            e.preventDefault();
            shell.openExternal(url)
        }
    }

    show() {
        this.win.show();
        this.win.focus()
    }

    initSplash() {
        this.splashWin = new BrowserWindow({
            width: 300,
            height: 300,
            frame: false,
            autoHideMenuBar: true,
            webPreferences: {
                nodeIntegration: true
            }
        });
        this.splashWin.loadURL(`file://${path.join(__dirname, '../view/splash.html')}`);

        ipcMain.on('reconnect', () => {
            this.connectToMicrosoft();
        });
    }

    connectToMicrosoft() {
        (async () => await isOnline({ timeout: 15000 }))().then(result => {
            if (result) {
                this.init();
                this.splashWin.destroy();
            } else {
                this.splashWin.webContents.send('connect-timeout');
            }
        });
    }
}

module.exports = MailWindowController;
