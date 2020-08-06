const { app } = require('electron');
const settings = require('electron-settings');
const fs = require('fs-extra');

class CssInjector {}

CssInjector.main = `
    /* hide the vertical ad bar */
    .${fs.existsSync(`${app.getPath('userData')}/Settings`) ? settings.get('verticalAdsClass', 'some-class-does-not-exist') : 'some-class-does-not-exist'} {
        display: none !important;
    }

    /* hide the small ad bar in other email page */
    .${fs.existsSync(`${app.getPath('userData')}/Settings`) ? settings.get('smallAdsClass', 'some-class-does-not-exist') : 'some-class-does-not-exist'} {
        display: none !important;
    }

    /* hide the upgrade premium ad bar */
    .${fs.existsSync(`${app.getPath('userData')}/Settings`) ? settings.get('premiumAdsClass', 'some-class-does-not-exist') : 'some-class-does-not-exist'} {
        display: none !important;
    }
`

CssInjector.noFrame = `
    /* make the header higher and dragable */
    ._1Kg3ffZABPxXxDqcmoxkBA {
        padding-top: 30px !important;
        -webkit-app-region: drag;
    }

    /* make the clickable component in header not dragable */
    .ms-FocusZone,
    ._3Nd2PGu67wifhuPZp2Sfj5 {
        -webkit-app-region: no-drag;
    }
`

module.exports = CssInjector