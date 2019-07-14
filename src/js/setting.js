const settings = require('electron-settings');

$(() => {
    loadSettings();
});

loadSettings = () => {
    // load ads blocker setting
    const verticalClass = settings.get('verticalAdsClass');
    const $verticalInput = $('#ads-blocker-vertical-class input');
    $verticalInput.val(verticalClass);
    $verticalInput.change(() => settings.set('verticalAdsClass', $verticalInput.val()));

    const smallClass = settings.get('smallAdsClass');
    const $smallInput = $('#ads-blocker-small-class input');
    $smallInput.val(smallClass);
    $smallInput.change(() => settings.set('smallAdsClass', $smallInput.val()));

    const premiumClass = settings.get('premiumAdsClass');
    const $premiumInput = $('#ads-blocker-premium-class input');
    $premiumInput.val(premiumClass);
    $premiumInput.change(() => settings.set('premiumAdsClass', $premiumInput.val()));

    // load home url setting
    const homepageUrl = settings.get('homepageUrl', 'https://outlook.live.com/mail');
    let $homepageUrl = $('#homepageUrl');
    $homepageUrl.dropdown('set selected', homepageUrl);

    $homepageUrl.dropdown({
        onChange: (value) => {
            settings.set('homepageUrl', value);
        }
    });
};