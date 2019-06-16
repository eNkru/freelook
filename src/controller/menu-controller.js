const { Menu } = require('electron')
const macOS = process.platform === 'darwin' ? true : false

class MenuController {
    constructor() {
        this.init()
    }

    init() {
        const appMenuTemplate = [{
            label: "Edit",
            submenu: [
                { label: "Undo", accelerator: "CmdOrCtrl+Z", selector: "undo:" },
                { label: "Redo", accelerator: "Shift+CmdOrCtrl+Z", selector: "redo:" },
                { type: "separator" },
                { label: "Cut", accelerator: "CmdOrCtrl+X", selector: "cut:" },
                { label: "Copy", accelerator: "CmdOrCtrl+C", selector: "copy:" },
                { label: "Paste", accelerator: "CmdOrCtrl+V", selector: "paste:" },
                { label: "Select All", accelerator: "CmdOrCtrl+A", selector: "selectAll:" }
            ]}
        ];

        if(macOS){
            Menu.setApplicationMenu(Menu.buildFromTemplate(appMenuTemplate));
        }
    }
}

module.exports = MenuController
