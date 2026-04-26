<img src="src-tauri/icons/128x128.png" alt="Freelook logo" height="80" align="right" />

# Freelook
[![Build](https://github.com/eNkru/freelook/actions/workflows/node.js.yml/badge.svg)](https://github.com/eNkru/freelook/actions/workflows/node.js.yml)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)

Freelook is a desktop wrapper for Outlook and Microsoft 365 Mail, built with [Tauri 2](https://tauri.app/). It keeps the app lightweight while still supporting tray behavior, unread indicators, notifications, and persistent window settings.

## Declaration
***This app helps some people like me who couldn't (or don't wish to) install a POP or SMTP mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource violate your copyright or trademark.***

## Features
* Access Outlook, Hotmail, and Microsoft 365 mail in a dedicated desktop app
* Close to tray instead of quitting the app
* Tray and dock unread indicators
* Native system notifications for unread mail
* Network connection detection
* Persistent app settings
* Optional ad blocking controls
* Switch between Outlook and Microsoft 365
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

This starts the Tauri development app. Frontend changes in `src/` are reloaded automatically, and Rust changes in `src-tauri/` trigger a rebuild.

### 3. Build for production

```bash
npm run build
```

This builds the production app and generates platform-specific bundles:

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
| `npm run release:patch` | — | Bump patch version, tag, and push to trigger a release |
| `npm run release:minor` | — | Bump minor version, tag, and push to trigger a release |
| `npm run release:major` | — | Bump major version, tag, and push to trigger a release |
| `npm run release` | — | Tag and push the current version without bumping |
| `npm run release:check` | — | Verify `package.json` and `tauri.conf.json` versions match |
| `npm run release:sync` | — | Copy version from `package.json` into `tauri.conf.json` |

## Releasing

Releases are automated via GitHub Actions. Pushing a `v*` tag triggers the [release workflow](.github/workflows/release.yml), which builds the app for macOS (ARM64 & x86_64), Linux (x86_64), and Windows (x86_64), then publishes the artifacts to a GitHub Release.

### Quick release

```bash
# Patch release (e.g. 2.0.0 → 2.0.1)
npm run release:patch

# Minor release (e.g. 2.0.0 → 2.1.0)
npm run release:minor

# Major release (e.g. 2.0.0 → 3.0.0)
npm run release:major
```

Each command will:
1. Bump the version in [`package.json`](package.json)
2. Sync the version to [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json)
3. Commit the changes with a `release: vX.Y.Z` message
4. Create a `vX.Y.Z` git tag
5. Push the commit and tag to GitHub

Once the tag is pushed, the GitHub Actions workflow takes over and builds platform-specific installers. The release starts as a **draft** and is automatically published once all builds complete successfully.

### Manual release

If you want to release the current version without bumping:

```bash
npm run release
```

This verifies the versions are in sync, creates the tag, and pushes it.

### macOS code signing

To enable macOS code signing and notarization, configure these repository secrets:

| Secret | Description |
|--------|-------------|
| `APPLE_CERTIFICATE` | Base64-encoded `.p12` certificate |
| `APPLE_CERTIFICATE_PASSWORD` | Password for the `.p12` certificate |
| `APPLE_SIGNING_IDENTITY` | Signing identity (e.g. `Developer ID Application: ...`) |
| `APPLE_ID` | Apple ID email used for notarization |
| `APPLE_PASSWORD` | App-specific password for the Apple ID |
| `APPLE_TEAM_ID` | Apple Developer Team ID |

Without these secrets, macOS builds will still succeed but the app will not be signed or notarized.

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
├── assets/                 # Tray and unread-state assets
├── src-tauri/icons/        # Tauri app icons used for bundling
├── package.json
└── tsconfig.json
```

## Packaging Notes

Freelook now uses the Tauri icon set in `src-tauri/icons/`. Release bundles are generated by `tauri build`, not by the old Electron packaging flow.

## Troubleshooting

**Some Linux distributions have issues displaying recipients in the To and CC fields.**

The workaround is to set some invalid values in the Ads Blocker settings. The settings come with default values; you need to input some random numbers to overwrite them.

**AppImage desktop integration not updated/created.**

Use [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) to install AppImages and create Desktop Integration, or create the desktop files manually.

## License
This app helps some people like me who couldn't (or don't wish to) install a POP or SMTP mail client to manage their outlook & hotmail emails. Please raise any concern to me if any of the code or resource violate your copyright or trademark.

[MIT](https://github.com/eNkru/freelook/blob/master/LICENSE) @ [Howard Ju](https://enkru.github.io/)
