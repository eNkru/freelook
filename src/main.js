const { app } = require('electron')
const MailWindowController = require('./controller/mail-window-controller')
const TrayController = require('./controller/tray-controller')
const MenuController = require('./controller/menu-controller')
const LoginController = require('./controller/login-controller')
const Store = require('electron-store')

app.commandLine.appendSwitch("auth-server-whitelist", "*");
app.commandLine.appendSwitch("enable-ntlm-v2", "true");


class ElectronOutlook {
  constructor() {
    this.mailController = null;
    this.trayController = null;
    this.menuController = null;
    this.loginController = null;
    this.config = new Store({
      name: "Settings",
      fileExtension: "",
    });
  }

  // init method, the entry point of the app
  init() {
    const lock = app.requestSingleInstanceLock()
    if (!lock) {
      app.quit()
    } else {
      app.on('second-instance', (event, commandLine, workingDirectory) => {
        if (this.mailController) this.mailController.show()
      })

      this.initApp()
    }
  }

  // init the main app
  initApp() {
    // This method will be called when Electron has finished
    // initialization and is ready to create browser windows.
    // Some APIs can only be used after this event occurs.
    app.on('ready', () => {
      this.createControllers()
    })

    // Quit when all windows are closed.
    app.on('window-all-closed', () => {
      // On macOS it is common for applications and their menu bar
      // to stay active until the user quits explicitly with Cmd + Q
      if (process.platform !== 'darwin' && !this.mailController) {
        app.quit()
      }
    })

    app.on('activate', () => {
      // On macOS it's common to re-create a window in the app when the
      // dock icon is clicked and there are no other windows open.
      if (this.mailController === null) {
        this.createControllers()
      } else {
        this.mailController.show()
      }
    })

    app.on('login', async (event, webContents, request, authInfo, callback) => {
      event.preventDefault()
      const origin = new URL(request.url).origin
      const { username, password } = await this.loginController.login(this.mailController.win, origin)
      callback(username, password)
    })
  }

  createControllers() {
    this.mailController = new MailWindowController(this.config)
    this.trayController = new TrayController(this.mailController,this.config)
    this.menuController = new MenuController()
    this.loginController = new LoginController()
  }
}

new ElectronOutlook().init()
