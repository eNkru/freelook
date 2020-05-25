<img src="build/icons/128x128.png" alt="logo" height="80" align="right" />

# Freelook

Freelook, an Electron-based desktop app for Microsoft Outlook and Office 365.

![screenshot_linux](https://user-images.githubusercontent.com/13460738/35953459-a0875872-0ce9-11e8-9bca-880564b9beee.png)

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
[MIT](https://github.com/eNkru/electron-xiami/blob/master/LICENSE) @ [Howard Ju](https://enkru.github.io/)