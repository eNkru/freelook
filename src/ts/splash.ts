// Splash screen TypeScript - Tauri version
export {};
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const splashIcon = document.getElementById('splashIcon') as HTMLElement;
const splashBtn = document.getElementById('splashBtn') as HTMLButtonElement;

async function checkConnection() {
    splashIcon.className = 'loading microsoft icon massive';
    splashBtn.textContent = 'Connecting to microsoft network...';
    splashBtn.disabled = true;

    try {
        const online = await invoke<boolean>('check_network');
        if (online) {
            splashIcon.className = 'checkmark microsoft icon massive';
            splashBtn.textContent = 'Connected! Loading...';
            // Main window will be shown by Rust backend
        } else {
            showDisconnected();
        }
    } catch (error) {
        showDisconnected();
    }
}

function showDisconnected() {
    splashIcon.className = 'wifi disabled icon massive';
    splashBtn.textContent = 'Reconnect to microsoft network...';
    splashBtn.disabled = false;
}

splashBtn.addEventListener('click', async () => {
    splashIcon.className = 'loading microsoft icon massive';
    splashBtn.textContent = 'Connecting to microsoft network...';
    splashBtn.disabled = true;

    try {
        const online = await invoke<boolean>('reconnect_and_launch');
        if (online) {
            splashIcon.className = 'checkmark microsoft icon massive';
            splashBtn.textContent = 'Connected! Loading...';
            // Main window is created and splash is closed by the Rust command
        } else {
            showDisconnected();
        }
    } catch (error) {
        showDisconnected();
    }
});

// Listen for connect-timeout event from Rust backend
listen('connect-timeout', () => {
    showDisconnected();
});

// Initial check on page load
checkConnection();