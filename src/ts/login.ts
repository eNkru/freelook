// Login form TypeScript - Tauri version
export {};
const { invoke } = window.__TAURI__.core;

const loginForm = document.getElementById('login-form') as HTMLFormElement;
const usernameInput = document.getElementById('username-input') as HTMLInputElement;
const passwordInput = document.getElementById('password-input') as HTMLInputElement;
const originSpan = document.getElementById('origin') as HTMLSpanElement;
const cancelButton = document.getElementById('cancel-form-button') as HTMLButtonElement;

// Read origin from URL query params
const params = new URLSearchParams(window.location.search);
const origin = params.get('origin') || '';
originSpan.textContent = origin;

loginForm.addEventListener('submit', async (event) => {
    event.preventDefault();

    const credentials = {
        username: usernameInput.value,
        password: passwordInput.value,
    };

    try {
        await invoke('submit_login', { credentials });
        // Close the login window after successful submission
        window.close();
    } catch (error) {
        console.error('Login failed:', error);
    }
});

cancelButton.addEventListener('click', () => {
    window.close();
});