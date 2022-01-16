const { ipcRenderer } = require('electron');

const form = document.getElementById('login-form');
form.addEventListener('submit', function (event) {
  event.preventDefault();
  const username = document.getElementById('username-input').value;
  const password = document.getElementById('password-input').value;
  ipcRenderer.send('login', { username, password });
});

const params = new URLSearchParams(window.location.search);
document.getElementById('origin').textContent = params.get('origin');
