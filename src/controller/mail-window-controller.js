const { BrowserWindow, shell, ipcMain, Menu, MenuItem, } = require("electron")
const CssInjector = require("../js/css-injector")
const path = require("path")
const isOnline = require("is-online")
const contextMenu = require('electron-context-menu')

contextMenu({
    showSaveImageAs: true
})

const deeplinkUrls = [
    "outlook.live.com/mail/deeplink",
    "outlook.office365.com/mail/deeplink",
    "outlook.office.com/mail/deeplink",
    "outlook.live.com/calendar/0/deeplink",
    "outlook.office365.com/calendar/0/deeplink",
    "outlook.office.com/calendar/0/deeplink",
]

class MailWindowController {
    constructor(config) {
        this.config = config
        this.initSplash()
        setTimeout(() => this.connectToMicrosoft(), 1000)
    }

    init() {
        // Get configurations.
        const showWindowFrame = this.config.get("showWindowFrame", true)
        const windowFrameX = this.config.get("windowFrameX", 100)
        const windowFrameY = this.config.get("windowFrameY", 100)
        const windowFrameWidth = this.config.get("windowFrameWidth", 1400)
        const windowFrameHeight = this.config.get("windowFrameHeight", 900)

        // Create the browser window.
        this.win = new BrowserWindow({
            x: windowFrameX,
            y: windowFrameY,
            width: windowFrameWidth,
            height: windowFrameHeight,
            frame: showWindowFrame,
            autoHideMenuBar: true,
            show: false,
            icon: path.join(__dirname, "../../assets/outlook_linux_black.png"),
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
                spellcheck: true,
                preload: path.join(__dirname, "../js/preload.js"),
            }
        })

        // and load the index.html of the app.
        this.win.loadURL(this.getHomepageUrl())

        // Open DevTools for debugging
        // this.win.webContents.openDevTools()

        // Show window handler
        ipcMain.on("show", () => {
            this.show()
        })

        // getConfig is handled by SettingsWindow controller

        this._pauseFrameSave = false;

        // Save the new position of window
        this.win.on("move", () => {
            if (this._pauseFrameSave) return;
            const position = this.win.getPosition()
            this.config.set("windowFrameX", position[0])
            this.config.set("windowFrameY", position[1])
        })

        // Save resized size of window
        this.win.on("resize", () => {
            if (this._pauseFrameSave) return;
            const size = this.win.getSize()
            this.config.set("windowFrameWidth", size[0])
            this.config.set("windowFrameHeight", size[1])
        })

        // Reset window position and size to defaults
        ipcMain.on("resetWindowFrame", () => {
            this._pauseFrameSave = true;
            this.win.setSize(1400, 900);
            this.win.center();
            this._pauseFrameSave = false;
        })

        this.win.webContents.on("did-navigate", (event, url, httpResponseCode, httpStatusText) => {
            if (httpResponseCode >= 400) {
                this.win.loadURL("data:text/htmlcharset=UTF-8," + encodeURIComponent(`
                    <!DOCTYPE html>
                    <html lang="">
                    <head>
                        <meta charset="UTF-8">
                        <title>Error</title>
                    </head>
                    <body>
                        <div style="text-align: center;">
                            <h1 id="error-message">${httpResponseCode} ${httpStatusText}</h1>
                            <a href="${this.getHomepageUrl()}">Return to home page</a>
                        </div>
                    </body>
                    </html>
                `))
            }
        })

        // insert styles
        this.win.webContents.on("dom-ready", () => {
            this.win.webContents.insertCSS(CssInjector.main(this.config))
            if (!showWindowFrame) this.win.webContents.insertCSS(CssInjector.noFrame)

            this.addUnreadNumberObserver()

            this.win.show()
        })

        // prevent the app quit, hide the window instead.
        this.win.on("close", (e) => {
            if (this.win.isVisible()) {
                e.preventDefault()
                this.win.hide()
            }
        })

        // Emitted when the window is closed.
        this.win.on("closed", () => {
            // Dereference the window object, usually you would store windows
            // in an array if your app supports multi windows, this is the time
            // when you should delete the corresponding element.
            this.win = null
        })

        // Open the new window in external browser
        this.win.webContents.setWindowOpenHandler(({ url }) => {
            return this.handleWindowOpen(url)
        })

        // Add context menu for build in spell checker
        this.win.webContents.on("context-menu", (event, params) => {
            if (params && params.dictionarySuggestions) {
                let show = false
                const menu = new Menu()
                // Add each spelling suggestion (if available)
                for (const suggestion of params.dictionarySuggestions) {
                    menu.append(new MenuItem({
                        label: suggestion,
                        click: () => this.win.webContents.replaceMisspelling(suggestion)
                    }))
                    show = true
                }

                // Allow users to add the misspelled word to the dictionary
                if (params.misspelledWord) {
                    menu.append(
                        new MenuItem({
                            label: "Add to dictionary",
                            click: () => this.win.webContents.session.addWordToSpellCheckerDictionary(params.misspelledWord)
                        })
                    )
                    show = true
                }

                // Show the context menu if there a spelling suggestion or a misspelled word exist
                if (show) {
                    menu.popup()
                }
            }
        })
    }

    getHomepageUrl() {
        return this.config.get("homepageUrl","https://outlook.live.com/mail")
    }

    addUnreadNumberObserver() {
        this.config.get("unreadMessageClass") && this.win.webContents.executeJavaScript(`
            setTimeout(() => {
                let unreadSpan = document.querySelector(".${this.config.get("unreadMessageClass")}")
                require("electron").ipcRenderer.send("updateUnread", unreadSpan.hasChildNodes())

                let observer = new MutationObserver(mutations => {
                    mutations.forEach(mutation => {
                        // console.log("Observer Changed.")
                        require("electron").ipcRenderer.send("updateUnread", unreadSpan.hasChildNodes())

                        // Scrape messages and pop up a notification
                        var messages = document.querySelectorAll("div[role="listbox"][aria-label="Message list"]")
                        if (messages.length)
                        {
                            var unread = messages[0].querySelectorAll("div[aria-label^="Unread"]")
                            var body = ""
                            for (var i = 0 i < unread.length i++)
                            {
                                if (body.length)
                                {
                                    body += "\\n"
                                }
                                body += unread[i].getAttribute("aria-label").substring(7, 127)
                            }
                            if (unread.length)
                            {
                                var notification = new Notification("Microsoft Outlook - receiving " + unread.length + " NEW mails", {
                                    body: body,
                                    icon: "assets/outlook_linux_black.png"
                                })
                                notification.onclick = () => {
                                    require("electron").ipcRenderer.send("show")
                                }
                            }
                        }
                    })
                })
            
                observer.observe(unreadSpan, {childList: true})

                // If the div containing reminders gets taller we probably got a new
                // reminder, so force the window to the top.
                let reminders = document.getElementsByClassName("_1BWPyOkN5zNVyfbTDKK1gM")
                let height = 0
                let reminderObserver = new MutationObserver(mutations => {
                    mutations.forEach(mutation => {
                        if (reminders[0].clientHeight > height)
                        {
                            require("electron").ipcRenderer.send("show")
                        }
                        height = reminders[0].clientHeight
                    })
                })

                if (reminders.length) {
                    reminderObserver.observe(reminders[0], { childList: true })
                }

            }, 10000)
        `)
    }

    // toggleWindow() {
    //     if (this.win) {
    //         if (this.win.isFocused()) {
    //             this.win.hide()
    //         } else {
    //             this.show()
    //         }
    //     }
    // }

    handleWindowOpen(url) {
        console.log(url)
        if (new RegExp(deeplinkUrls.join("|")).test(url)) {
            // Allow deeplink URLs to open in a new Electron window
            return { action: 'allow' }
        } else if (url && url.startsWith("https://outlook.live.com/calendar/0/deeplink")) {
            // Disable the logic to load calendar in the Electron window.
            // Calendar has no link back to mail.
            return { action: 'deny' }
        } else {
            // Send everything else to the external browser
            shell.openExternal(url)
            return { action: 'deny' }
        }
    }

    show() {
        this.win.show()
        this.win.focus()
    }

    initSplash() {
        this.splashWin = new BrowserWindow({
            width: 300,
            height: 300,
            frame: false,
            autoHideMenuBar: true,
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
            }
        })
        this.splashWin.loadURL(`file://${path.join(__dirname, "../view/splash.html")}`)

        ipcMain.on("reconnect", () => {
            this.connectToMicrosoft()
        })
    }

    connectToMicrosoft() {
        (async () => await isOnline({ timeout: 15000 }))().then(result => {
            if (result) {
                this.init()
                this.splashWin.destroy()
            } else {
                this.splashWin.webContents.send("connect-timeout")
            }
        })
    }
}

module.exports = MailWindowController
