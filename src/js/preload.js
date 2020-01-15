const {SpellCheckHandler, ContextMenuListener, ContextMenuBuilder} = require('electron-spellchecker');

setTimeout(() => {
    window.spellCheckHandler = new SpellCheckHandler();
    window.spellCheckHandler.attachToInput();
    window.spellCheckHandler.switchLanguage('en-US');

    const contextMenuBuilder = new ContextMenuBuilder(window.spellCheckHandler);
    const contextMenuListener = new ContextMenuListener((info) => {
        contextMenuBuilder.showPopupMenu(info);
    });
}, 5000);