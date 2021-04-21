const { ipcRenderer } = require('electron');

$(() => {
    loadSettings();
});

loadSettings = () => {
    // load ads blocker setting
    const verticalClass = ipcRenderer.sendSync("getConfig",'verticalAdsClass');
    const $verticalInput = $('#ads-blocker-vertical-class input');
    $verticalInput.val(verticalClass);
    $verticalInput.change(() => ipcRenderer.send("setConfig",'verticalAdsClass', $verticalInput.val()));

    const smallClass = ipcRenderer.sendSync("getConfig",'smallAdsClass');
    const $smallInput = $('#ads-blocker-small-class input');
    $smallInput.val(smallClass);
    $smallInput.change(() => ipcRenderer.send("setConfig",'smallAdsClass', $smallInput.val()));

    const premiumClass = ipcRenderer.sendSync("getConfig",'premiumAdsClass');
    const $premiumInput = $('#ads-blocker-premium-class input');
    $premiumInput.val(premiumClass);
    $premiumInput.change(() => ipcRenderer.send("setConfig",'premiumAdsClass', $premiumInput.val()));

    // load unread message setting
    const unreadMsgClass = ipcRenderer.sendSync("getConfig",'unreadMessageClass');
    const unreadMsgInput = $('#unread-message-class input');
    unreadMsgInput.val(unreadMsgClass);
    unreadMsgInput.change(() => ipcRenderer.send("setConfig",'unreadMessageClass', unreadMsgInput.val()));

    // load home url setting
    const homepageUrl = ipcRenderer.sendSync("getConfig",'homepageUrl','https://outlook.live.com/mail');
    let $homepageUrl = $('#homepageUrl');
    $homepageUrl.dropdown('set selected', homepageUrl);
    $homepageUrl.dropdown({
        onChange: (value) => {
            ipcRenderer.send("setConfig",'homepageUrl', value);
        }
    });

    // Window position and size handling
    let $windowReset = $('#windowReset');
    $windowReset.click(() => {
        ipcRenderer.send("deleteConfig", 'windowFrameX');
        ipcRenderer.send("deleteConfig", 'windowFrameY');
        ipcRenderer.send("deleteConfig", 'windowFrameWidth');
        ipcRenderer.send("deleteConfig", 'windowFrameHeight');
    });
};