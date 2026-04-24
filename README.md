<img src="build/icons/128x128.png" alt="logo" height="80" align="right" />

# Freelook
[![Build](https://github.com/eNkru/freelook/actions/workflows/node.js.yml/badge.svg)](https://github.com/eNkru/freelook/actions/workflows/node.js.yml)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)

Freelook, an alternative desktop app to help you manage Outlook and Office 365. Powered by [Tauri](https://tauri.app/).

## Declaration
***This app helps some people like me who couldn't (or don't wish to) install a POP or SMTP mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource violate your copyright or trademark.***

## Features
* Receive your hotmail / outlook / office 365 online from the desktop app
* Close to minimise (close-to-tray)
* Dock tray support with unread mail indicator
* System notification for new unread emails
* Network connection detection
* Customized settings
    * Ads block as your control
    * Switch between outlook and office 365
    * Zoom level adjustment
    * Window position and size persistence

## Prerequisites

### All Platforms
* [Node.js](https://nodejs.org/) >= 24
* [Rust](https://www.rust-lang.org/tools/install) toolchain (rustup, cargo)

### macOS
* Xcode Command Line Tools: `xcode-select --install`

### Linux (Ubuntu/Debian)
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### Windows
* [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)
* Visual Studio Build Tools

## Download
The released application can be downloaded [here](https://github.com/eNkru/freelook/releases).

## Getting Started

### 1. Clone and install dependencies

```bash
git clone https://github.com/eNkru/freelook.git
cd freelook
npm install
```

### 2. Run in development mode

```bash
npm run dev
```

This compiles the Rust backend and launches the app with hot-reload. Frontend changes (HTML/CSS/TS in `src/`) are picked up automatically. Rust code changes trigger a recompile.

### 3. Build for production

```bash
npm run build
```

This compiles the Rust backend in release mode and bundles the app into platform-specific installers:

| Platform | Output |
|----------|--------|
| macOS    | `.dmg` |
| Linux    | `.AppImage`, `.deb`, `.rpm` |
| Windows  | `.msi` |

Build artifacts are written to `src-tauri/target/release/bundle/`.

### Available npm scripts

| Script | Command | Description |
|--------|---------|-------------|
| `npm run dev` | `tauri dev` | Launch app in development mode with hot-reload |
| `npm run build` | `tauri build` | Build production installers for your platform |
| `npm run build:ts` | `tsc` | Compile TypeScript files in `src/ts/` only |
| `npm run tauri` | `tauri` | Direct access to the Tauri CLI |

## Project Structure

```
freelook/
├── src/                    # Frontend source files
│   ├── ts/                 # TypeScript source
│   │   ├── splash.ts       # Splash screen logic
│   │   ├── login.ts        # Login form logic
│   │   ├── setting.ts      # Settings page logic
│   │   ├── css-injector.ts # CSS injection for ad blocking
│   │   └── tauri.d.ts      # Tauri type declarations
│   ├── view/               # HTML views
│   │   ├── splash.html
│   │   ├── login.html
│   │   └── setting.html
│   └── css/                # Stylesheets
│       ├── splash.css
│       └── setting.css
├── src-tauri/              # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── lib.rs          # App builder & plugin registration
│   │   ├── commands.rs     # Tauri commands exposed to frontend
│   │   ├── config.rs       # Persistent configuration store
│   │   ├── tray.rs         # System tray management
│   │   ├── menu.rs         # Native menu (macOS)
│   │   ├── windows.rs      # Multi-window management
│   │   └── network.rs      # Network connectivity detection
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── capabilities/       # Permission definitions
├── assets/                 # Tray icon assets
├── build/                  # Application icons
├── package.json
└── tsconfig.json
```

## Troubleshooting

**Some Linux distributions have issues displaying recipients in the To and CC fields.**

The workaround is to set some invalid values in the Ads Blocker settings. The settings come with default values; you need to input some random numbers to overwrite them.

**AppImage desktop integration not updated/created.**

Use [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) to install AppImages and create Desktop Integration, or create the desktop files manually.

## License
This app helps some people like me who couldn't (or don't wish to) install a POP or SMTP mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource violate your copyright or trademark.

[MIT](https://github.com/eNkru/freelook/blob/master/LICENSE) @ [Howard Ju](https://enkru.github.io/)