const { app, BrowserWindow, shell, ipcMain, Menu, MenuItem, Notification } = require('electron');
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
                spellcheck: true,
                preload: path.join(__dirname, '../js/preload.js')
            }
        });

        this.win.webContents.openDevTools()

        // and load the index.html of the app.
        this.win.loadURL(homepageUrl, {userAgent: 'Chrome'});

        // Show window handler
        ipcMain.on('show', () => {
            this.show()
        });

        let mailNotifications = [];
        let notification = undefined;
        // Show notification handler
        ipcMain.on('showNotification', (event, data) => {

            if (data) {
                mailNotifications.push(data);

                if (notification) {
                    notification.close();
                }

                notification = new Notification({
                    title: "Received " + mailNotifications.length + (mailNotifications.length === 1 ? " email" : " emails"),
                    body: mailNotifications.map((n, i) => n.address + ": " + n.subject).join("\n"),

                    timeoutType: "never",
                    icon: "assets/outlook_linux_black.png",
                    urgency: "normal",

                });


                notification.on("click", () => {
                    mailNotifications = [];
                    this.show();
                });

                notification.on("close", () => {
                    mailNotifications = [];
                    notification = undefined;
                });
                notification.show();
            }
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
            // Dereference the window object, usually you would store windows
            // in an array if your app supports multi windows, this is the time
            // when you should delete the corresponding element.
            this.win = null
        });

        // Open the new window in external browser
        this.win.webContents.on('new-window', this.openInBrowser)

        // Add context menu for build in spell checker
        this.win.webContents.on('context-menu', (event, params) => {

            if (params && params.dictionarySuggestions) {
                const menu = new Menu()
                // Add each spelling suggestion
                for (const suggestion of params.dictionarySuggestions) {
                    menu.append(new MenuItem({
                        label: suggestion,
                        click: () => this.win.webContents.replaceMisspelling(suggestion)
                    }))
                }

                // Allow users to add the misspelled word to the dictionary
                if (params.misspelledWord) {
                    menu.append(
                        new MenuItem({
                            label: 'Add to dictionary',
                            click: () => this.win.webContents.session.addWordToSpellCheckerDictionary(params.misspelledWord)
                        })
                    )
                }

                menu.popup()
            }
        });
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
