const {ipcRenderer} = require('electron');

ipcRenderer.on('connect-timeout', () => {
    const splashIcon = document.getElementById('splashIcon');
    splashIcon.classList.remove('loading', 'microsoft');
    splashIcon.classList.add('wifi', 'disabled');

    const splashBtn = document.getElementById('splashBtn');
    splashBtn.innerText = `Network connection is not available.
Click here to reconnect`;
});

document.querySelector('#splashBtn').addEventListener('click', () => {
    const splashIcon = document.getElementById('splashIcon');
    splashIcon.classList.remove('wifi', 'disabled');
    splashIcon.classList.add('loading', 'microsoft');

    const splashBtn = document.getElementById('splashBtn');
    splashBtn.innerText = 'Connect to microsoft network...';

    ipcRenderer.send('reconnect');
});