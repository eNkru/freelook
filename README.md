<img src="build/icons/128x128.png" alt="logo" height="80" align="right" />

# Freelook
Freelook, an alternative Electron-based desktop app to help you manage Outlook and Office 365.

## Ask for help
Please raise an PR If you can help me to design an icon for this app (currently the icon is shown on the right top conor of this README)
If you'd like to help but don't know how to PR, please contact me directly. 

Many Thanks.

## Declamation
***This app helps some people like me who couldn't (or don't wish to) install an POP or SMPT mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource voilate your copyright or trademark.***

## Feature
* Receive your hotmail / outlook / office 365 online from the desktop app
* Close to minimise
* Dock tray support
* System notification
* Network connection detection
* Customized setting
    * Ads block as your control
    * Switch between outlook and office 365

## Download
The released application can be downloaded [here](https://github.com/eNkru/electron-outlook/releases).

## Troubleshot
`Some Linux distributions has the issue to display the reciptons in the To and CC fields.`

The workaround is set some invalid values in the Ads Blocker settings. The settings come with the default value, you need to input some random numbers to overwrite them.

`Desktop Integration not updated/created when using AppImage`

Since electron-builder 21 desktop integration is not a part of produced AppImage file anymore. Electron builder recommends [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) to install AppImages and create Desktop Integration or to create the desktop files manually.

## Build Pre-Request
* [GIT](https://git-scm.com/)
* [NPM](https://www.npmjs.com/)

## Build & Install
Clone the repository and run in development mode.
```
git clone https://github.com/eNkru/freelook.git
cd freelook
npm i
npm run start
```
Build the application 
```
npm run dist:linux
```
This will build a predefined AppImage & deb packages in the dist folder. AppImage can be run in most popular linux distributions with the support. Deb is only for debian & ubuntu distributions.

## Release
```
npm version (new release version)
git push origin master
git push origin --tags
npm publish
```

## License
This app helps some people like me who couldn't (or don't wish to) install an POP or SMPT mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource voilate your copyright or trademark.

[MIT](https://github.com/eNkru/electron-xiami/blob/master/LICENSE) @ [Howard Ju](https://enkru.github.io/)
