const { app, BrowserWindow, shell, ipcMain, Menu, MenuItem, } = require('electron');
const CssInjector = require('../js/css-injector');
const path = require('path');
const isOnline = require('is-online');

const deeplinkUrls = ['outlook.live.com/mail/deeplink', 'outlook.office365.com/mail/deeplink', 'outlook.office.com/mail/deeplink'];
const outlookUrls = ['outlook.live.com', 'outlook.office365.com', 'outlook.office.com'];

class MailWindowController {
    constructor(config) {
        this.config = config;
        this.initSplash();
        setTimeout(() => this.connectToMicrosoft(), 1000);
    }

    init() {
        // Get configurations.
        const showWindowFrame = this.config.get('showWindowFrame', true);
        const windowFrameX = this.config.get('windowFrameX', 100);
        const windowFrameY = this.config.get('windowFrameY', 100);
        const windowFrameWidth = this.config.get('windowFrameWidth', 1400);
        const windowFrameHeight = this.config.get('windowFrameHeight', 900);

        // Create the browser window.
        this.win = new BrowserWindow({
            x: windowFrameX,
            y: windowFrameY,
            width: windowFrameWidth,
            height: windowFrameHeight,
            frame: showWindowFrame,
            autoHideMenuBar: true,
            show: false,
            icon: path.join(__dirname, '../../assets/outlook_linux_black.png'),
            webPreferences: {
                enableRemoteModule: true,
                nodeIntegration: true,
                spellcheck: true,
                preload: path.join(__dirname, '../js/preload.js')
            }
        });

        // and load the index.html of the app.
        this.win.loadURL(this.config.get("homepageUrl",'https://outlook.live.com/mail'));

        // Show window handler
        ipcMain.on('show', () => {
            this.show()
        });

        // Save the new position of window
        this.win.on('move', (e) => {
            var position = this.win.getPosition();
            this.config.set('windowFrameX', position[0]);
            this.config.set('windowFrameY', position[1]);
        });

        // Save resized size of window
        this.win.on('resize', (e) => {
            var size = this.win.getSize();
            this.config.set('windowFrameWidth', size[0]);
            this.config.set('windowFrameHeight', size[1]);
        });

        // insert styles
        this.win.webContents.on('dom-ready', () => {
            this.win.webContents.insertCSS(CssInjector.main(this.config));
            if (!showWindowFrame) this.win.webContents.insertCSS(CssInjector.noFrame);

            this.addUnreadNumberObserver();

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
        this.win.webContents.on('new-window', this.openInBrowser);

        // Add context menu for build in spell checker
        this.win.webContents.on('context-menu', (event, params) => {
            if (params && params.dictionarySuggestions) {
                let show = false;
                const menu = new Menu()
                // Add each spelling suggestion (if available)
                for (const suggestion of params.dictionarySuggestions) {
                    menu.append(new MenuItem({
                        label: suggestion,
                        click: () => this.win.webContents.replaceMisspelling(suggestion)
                    }));
                    show = true;
                }

                // Allow users to add the misspelled word to the dictionary
                if (params.misspelledWord) {
                    menu.append(
                        new MenuItem({
                            label: 'Add to dictionary',
                            click: () => this.win.webContents.session.addWordToSpellCheckerDictionary(params.misspelledWord)
                        })
                    );
                    show = true;
                }

                // Show the context menu if there a spelling suggestion or a misspelled word exist
                if (show) {
                    menu.popup();
                }
            }
        });
    }

    addUnreadNumberObserver() {
        this.config.get('unreadMessageClass') && this.win.webContents.executeJavaScript(`
            setTimeout(() => {
                let unreadSpan = document.querySelector(".${this.config.get('unreadMessageClass')}");
                require('electron').ipcRenderer.send('updateUnread', unreadSpan.hasChildNodes());

                let observer = new MutationObserver(mutations => {
                    mutations.forEach(mutation => {
                        // console.log('Observer Changed.');
                        require('electron').ipcRenderer.send('updateUnread', unreadSpan.hasChildNodes());

                        // Scrape messages and pop up a notification
                        var messages = document.querySelectorAll('div[role="listbox"][aria-label="Message list"]');
                        if (messages.length)
                        {
                            var unread = messages[0].querySelectorAll('div[aria-label^="Unread"]');
                            var body = "";
                            for (var i = 0; i < unread.length; i++)
                            {
                                if (body.length)
                                {
                                    body += "\\n";
                                }
                                body += unread[i].getAttribute("aria-label").substring(7, 127);
                            }
                            if (unread.length)
                            {
                                var notification = new Notification("Microsoft Outlook - receiving " + unread.length + " NEW mails", {
                                    body: body,
                                    icon: "assets/outlook_linux_black.png"
                                });
                                notification.onclick = () => {
                                    require('electron').ipcRenderer.send('show');
                                };
                            }
                        }
                    });
                });
            
                observer.observe(unreadSpan, {childList: true});

                // If the div containing reminders gets taller we probably got a new
                // reminder, so force the window to the top.
                let reminders = document.getElementsByClassName("_1BWPyOkN5zNVyfbTDKK1gM");
                let height = 0;
                let reminderObserver = new MutationObserver(mutations => {
                    mutations.forEach(mutation => {
                        if (reminders[0].clientHeight > height)
                        {
                            require('electron').ipcRenderer.send('show');
                        }
                        height = reminders[0].clientHeight;
                    });
                });

                if (reminders.length) {
                    reminderObserver.observe(reminders[0], { childList: true });
                }

            }, 10000);
        `)
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
