use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tauri::{
    webview::{DownloadEvent, PageLoadEvent},
    AppHandle, Manager, WebviewUrl, WebviewWindowBuilder,
};

#[allow(dead_code)]
const DEEPLINK_URLS: &[&str] = &[
    "outlook.live.com/mail/deeplink",
    "outlook.office365.com/mail/deeplink",
    "outlook.office.com/mail/deeplink",
    "outlook.live.com/calendar/0/deeplink",
    "outlook.office365.com/calendar/0/deeplink",
    "outlook.office.com/calendar/0/deeplink",
];

/// Check if a URL belongs to a Microsoft domain that should stay in-app
fn is_microsoft_url(url: &url::Url) -> bool {
    match url.scheme() {
        "about" | "data" => return true,
        "blob" => {
            return url
                .path()
                .parse::<url::Url>()
                .map(|inner_url| is_microsoft_url(&inner_url))
                .unwrap_or(true);
        }
        _ => {}
    }

    match url.host_str() {
        Some(host) => {
            let h = host.to_lowercase();
            h == "outlook.live.com"
                || h == "outlook.office365.com"
                || h == "outlook.office.com"
                || h == "login.microsoftonline.com"
                || h == "login.live.com"
                || h == "outlook.live.net"
                || h == "www.office.com"
                || h.ends_with(".outlook.com")
                || h.ends_with(".outlook.live.com")
                || h.ends_with(".outlook.live.net")
                || h.ends_with(".office.net")
                || h.ends_with(".office365.com")
                || h.ends_with(".office365.net")
                || h.ends_with(".office.com")
                || h.ends_with(".officeppe.net")
                || h.ends_with(".officeapps.live.com")
                || h.ends_with(".microsoft.com")
                || h.ends_with(".microsoftusercontent.com")
                || h.ends_with(".microsoftonline.com")
                || h.ends_with(".msauth.net")
                || h.ends_with(".msocdn.com")
                || h.ends_with(".msftauth.net")
                || h.ends_with(".live.com")
                || h.ends_with(".1drv.ms")
                || h.ends_with(".sharepoint.com")
                || h.ends_with(".sharepointonline.com")
                || h.ends_with(".onenote.com")
                || h.ends_with(".onedrive.com")
                || h.ends_with(".svc.ms")
        }
        None => false,
    }
}

fn is_attachment_or_preview_url(url: &url::Url) -> bool {
    if !is_microsoft_url(url) {
        return false;
    }

    let value = url.as_str().to_lowercase();
    value.contains("attachment")
        || value.contains("download")
        || value.contains("pdf")
        || value.contains("mspdfkit")
        || value.contains("fluidpreview")
        || value.contains("officepreview")
        || value.contains("wopi")
        || value.contains("owa/service.svc")
}

/// Check if a URL is an ad/tracking URL that should be silently blocked
fn is_ad_url(url: &url::Url) -> bool {
    match url.host_str() {
        Some(host) => {
            let h = host.to_lowercase();
            // SafeFrame ad SDK
            h.contains("adsdkprod")
                || h.contains("adsdk")
                // AppNexus / Xandr ad tracking
                || h.contains("adnxs.com")
                || h.contains("adnxs.net")
                // Google ad services
                || h.contains("doubleclick.net")
                || h.contains("googlesyndication.com")
                || h.contains("googleadservices.com")
                || h.contains("googletagmanager.com")
                // Common ad domains
                || h.starts_with("ads.")
                || h.starts_with("ad.")
                || h.contains("adsystem.com")
                || h.contains("advertising.com")
                || h.contains("adtechus.com")
                || h.contains("adcolony.com")
                || h.contains("adsafeprotected.com")
                || h.contains("moatads.com")
                || h.contains("serving-sys.com")
                || h.contains("sizmek.com")
                || h.contains("rubiconproject.com")
                || h.contains("pubmatic.com")
                || h.contains("openx.net")
                || h.contains("casalemedia.com")
                || h.contains("indexww.com")
                || h.contains("turn.com")
                || h.contains("mathtag.com")
                || h.contains("bidswitch.net")
                || h.contains("contextweb.com")
                || h.contains("sharethrough.com")
                || h.contains("spotxchange.com")
                || h.contains("tidaltv.com")
                || h.contains("tremorhub.com")
                || h.contains("videologygroup.com")
                || h.contains("yieldmo.com")
                || h.contains("smartadserver.com")
        }
        None => false,
    }
}

/// Get homepage URL from config, with fallback
fn get_homepage_url(app: &AppHandle) -> String {
    let store = tauri_plugin_store::StoreBuilder::new(app, "Settings")
        .build()
        .ok();
    if let Some(store) = store {
        if let Some(val) = store.get("homepageUrl") {
            if let Some(s) = val.as_str() {
                return s.to_string();
            }
        }
    }
    "https://outlook.live.com/mail".to_string()
}

/// Get config value as string with default
fn get_config_string(app: &AppHandle, key: &str, default: &str) -> String {
    let store = tauri_plugin_store::StoreBuilder::new(app, "Settings")
        .build()
        .ok();
    if let Some(store) = store {
        if let Some(val) = store.get(key) {
            if let Some(s) = val.as_str() {
                return s.to_string();
            }
        }
    }
    default.to_string()
}

/// Set config value
pub fn set_config_value(app: &AppHandle, key: &str, value: &str) {
    if let Ok(store) = tauri_plugin_store::StoreBuilder::new(app, "Settings").build() {
        store.set(
            key.to_string(),
            serde_json::Value::String(value.to_string()),
        );
        let _ = store.save();
    }
}

static LAST_FRAME_SAVE: Mutex<Option<Instant>> = Mutex::new(None);
static NEXT_DOWNLOAD_NAME: Mutex<Option<String>> = Mutex::new(None);

pub fn set_pending_download_name(name: String) {
    let name = sanitize_file_name(&name);
    if name.is_empty() {
        return;
    }

    eprintln!("[freelook] pending download name: {}", name);
    if let Ok(mut pending) = NEXT_DOWNLOAD_NAME.lock() {
        *pending = Some(name);
    }
}

fn take_pending_download_name() -> Option<String> {
    NEXT_DOWNLOAD_NAME
        .lock()
        .ok()
        .and_then(|mut pending| pending.take())
}

fn sanitize_file_name(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            ch if ch.is_control() => '_',
            ch => ch,
        })
        .collect::<String>();

    sanitized
        .trim()
        .trim_matches('.')
        .trim()
        .chars()
        .take(180)
        .collect()
}

fn unique_download_path(download_dir: &Path, file_name: &str) -> PathBuf {
    let candidate = download_dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }

    let path = Path::new(file_name);
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("download");
    let extension = path.extension().and_then(|value| value.to_str());

    for index in 1..1000 {
        let name = match extension {
            Some(extension) if !extension.is_empty() => format!("{stem} ({index}).{extension}"),
            _ => format!("{stem} ({index})"),
        };
        let candidate = download_dir.join(name);
        if !candidate.exists() {
            return candidate;
        }
    }

    candidate
}

fn download_name(url: &url::Url, destination: &std::path::Path) -> std::ffi::OsString {
    if let Some(name) = take_pending_download_name() {
        return std::ffi::OsString::from(name);
    }

    if let Some(name) = destination.file_name().filter(|name| !name.is_empty()) {
        return name.to_os_string();
    }

    if url.scheme() == "blob" {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default();
        return std::ffi::OsString::from(format!("freelook-attachment-{}.pdf", timestamp));
    }

    url.path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|name| !name.is_empty())
        .map(std::ffi::OsString::from)
        .unwrap_or_else(|| std::ffi::OsString::from("download"))
}

/// Save the main window's current position and size to config (debounced to 500ms).
pub fn save_window_frame(app: &AppHandle) {
    let now = Instant::now();
    if let Ok(mut last) = LAST_FRAME_SAVE.lock() {
        if let Some(prev) = *last {
            if now.duration_since(prev).as_millis() < 500 {
                return;
            }
        }
        *last = Some(now);
    }

    if let Some(window) = app.get_webview_window("main") {
        let scale = window.scale_factor().unwrap_or(1.0);
        if let Ok(pos) = window.outer_position() {
            let x = pos.x as f64 / scale;
            let y = pos.y as f64 / scale;
            set_config_value(app, "windowFrameX", &x.to_string());
            set_config_value(app, "windowFrameY", &y.to_string());
        }
        if let Ok(size) = window.outer_size() {
            let w = size.width as f64 / scale;
            let h = size.height as f64 / scale;
            set_config_value(app, "windowFrameWidth", &w.to_string());
            set_config_value(app, "windowFrameHeight", &h.to_string());
        }
    }
}

/// Force-save the window frame (bypasses debounce, for use on close).
pub fn save_window_frame_now(app: &AppHandle) {
    if let Ok(mut last) = LAST_FRAME_SAVE.lock() {
        *last = None;
    }
    save_window_frame(app);
}

/// Create and show the main (mail) window
pub fn create_main_window(app: &AppHandle) -> Result<(), String> {
    // Read config values
    let x: f64 = get_config_string(app, "windowFrameX", "100")
        .parse()
        .unwrap_or(100.0);
    let y: f64 = get_config_string(app, "windowFrameY", "100")
        .parse()
        .unwrap_or(100.0);
    let width: f64 = get_config_string(app, "windowFrameWidth", "1400")
        .parse()
        .unwrap_or(1400.0);
    let height: f64 = get_config_string(app, "windowFrameHeight", "900")
        .parse()
        .unwrap_or(900.0);
    let homepage_url = get_homepage_url(app);
    let app_handle = app.clone();

    let _window = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External(homepage_url.parse().unwrap()),
    )
    .title("Freelook")
    .inner_size(width, height)
    .position(x, y)
    .visible(false)
    .disable_drag_drop_handler()
    .on_download(|webview, event| {
        match event {
            DownloadEvent::Requested { url, destination } => {
                eprintln!("[freelook] download requested: {}", url);
                if let Ok(download_dir) = webview.app_handle().path().download_dir() {
                    let suggested_name = download_name(&url, destination);
                    let suggested_name = suggested_name.to_string_lossy();
                    *destination = unique_download_path(&download_dir, suggested_name.as_ref());
                    eprintln!("[freelook] download destination: {:?}", destination);
                }
            }
            DownloadEvent::Finished { url, path, success } => {
                eprintln!(
                    "[freelook] download finished: {} path={:?} success={}",
                    url, path, success
                );
            }
            _ => {}
        }
        true
    })
    .on_navigation(move |url| {
        if is_attachment_or_preview_url(url) {
            eprintln!("[freelook] opening attachment externally: {}", url);
            let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            false
        } else if is_microsoft_url(url) {
            true
        } else if is_ad_url(url) {
            // Silently block ad/tracking navigations
            false
        } else {
            let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            false
        }
    })
    .on_new_window(|url, _features| {
        if is_attachment_or_preview_url(&url) {
            eprintln!(
                "[freelook] opening attachment new-window externally: {}",
                url
            );
            let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            tauri::webview::NewWindowResponse::Deny
        } else if is_microsoft_url(&url) {
            tauri::webview::NewWindowResponse::Allow
        } else if is_ad_url(&url) {
            // Silently deny ad/tracking popups
            tauri::webview::NewWindowResponse::Deny
        } else {
            let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            tauri::webview::NewWindowResponse::Deny
        }
    })
    .on_page_load(move |_window, payload| {
        if payload.event() == PageLoadEvent::Finished {
            let _ = apply_main_settings(&app_handle);
        }
    })
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn apply_main_settings(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let zoom = get_config_string(app, "zoomFactor", "1.0")
            .parse::<f64>()
            .unwrap_or(1.0);
        window.set_zoom(zoom).map_err(|e| e.to_string())?;

        inject_main_css(&window, &get_main_css(app))?;

        // Inject link click interceptor
        let _ = window.eval(get_link_interceptor_js());

        // Inject floating refresh button
        let _ = window.eval(get_refresh_button_js());
    }
    Ok(())
}

fn inject_main_css(window: &tauri::WebviewWindow, css: &str) -> Result<(), String> {
    let escaped_css = css
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('$', "\\$");
    let js = format!(
        r#"
        (() => {{
            const id = "freelook-main-css";
            let style = document.getElementById(id);
            if (!style) {{
                style = document.createElement("style");
                style.id = id;
                document.head.appendChild(style);
            }}
            style.textContent = `{}`;
        }})();
        "#,
        escaped_css
    );
    window.eval(&js).map_err(|e| e.to_string())
}

/// Show the main window
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Reset window frame to defaults
pub fn reset_window_frame(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_size(tauri::LogicalSize::new(1400.0, 900.0))
            .map_err(|e| e.to_string())?;
        window.center().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Open settings window (create if needed, show if exists)
pub fn open_settings(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let window = WebviewWindowBuilder::new(
        &app,
        "settings",
        WebviewUrl::App("view/setting.html".into()),
    )
    .title("Settings")
    .inner_size(960.0, 740.0)
    .min_inner_size(800.0, 500.0)
    .visible(true)
    .build()
    .map_err(|e| e.to_string())?;

    let _ = window.set_focus();

    Ok(())
}

/// Restart the application
pub fn restart_app(app: AppHandle) -> Result<(), String> {
    app.restart();
}

/// Refresh (reload) the main webview page
pub fn refresh_page(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .eval("location.reload()")
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Inject CSS into a webview
pub fn css_inject(app: AppHandle, webview_label: String, css: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&webview_label) {
        let escaped_css = css
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace('$', "\\$");
        let js = format!(
            "document.head.insertAdjacentHTML('beforeend', `<style>{}</style>`)",
            escaped_css
        );
        window.eval(&js).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Check if a URL is a deeplink
#[allow(dead_code)]
pub fn is_deeplink(url: &str) -> bool {
    DEEPLINK_URLS.iter().any(|dl| url.contains(dl))
}

/// Check if a URL is a calendar deeplink (which we deny)
#[allow(dead_code)]
pub fn is_calendar_deeplink(url: &str) -> bool {
    url.contains("outlook.live.com/calendar/0/deeplink")
        || url.contains("outlook.office365.com/calendar/0/deeplink")
        || url.contains("outlook.office.com/calendar/0/deeplink")
}

/// Get the CSS for ad blocking
pub fn get_main_css(app: &AppHandle) -> String {
    let vertical = get_config_string(app, "verticalAdsClass", "pBKjV");
    let small = get_config_string(app, "smallAdsClass", "X1Kvq");
    let premium = get_config_string(app, "premiumAdsClass", "VPtFl");

    format!(
        r#"
        .{vertical} {{ display: none !important; }}
        .{small} {{ display: none !important; }}
        .{premium} {{ display: none !important; }}
        "#
    )
}

/// Get the JavaScript that injects a floating refresh button
fn get_refresh_button_js() -> &'static str {
    r#"
    (function() {
        if (document.getElementById('freelook-refresh-btn')) return;

        const btn = document.createElement('div');
        btn.id = 'freelook-refresh-btn';
        btn.title = 'Refresh page';
        btn.innerHTML = '<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"></polyline><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path></svg>';
        btn.style.cssText = 'position:fixed;bottom:24px;right:24px;z-index:2147483647;width:44px;height:44px;border-radius:50%;background:rgba(0,120,212,0.85);display:flex;align-items:center;justify-content:center;cursor:pointer;box-shadow:0 2px 8px rgba(0,0,0,0.3);transition:background 0.2s,transform 0.15s;backdrop-filter:blur(4px);';

        btn.addEventListener('mouseenter', function() {
            btn.style.background = 'rgba(0,120,212,1)';
            btn.style.transform = 'scale(1.1)';
        });
        btn.addEventListener('mouseleave', function() {
            btn.style.background = 'rgba(0,120,212,0.85)';
            btn.style.transform = 'scale(1)';
        });
        btn.addEventListener('click', function(e) {
            e.preventDefault();
            e.stopPropagation();
            location.reload();
        });

        document.body.appendChild(btn);
    })();
    "#
}

/// Get the no-frame CSS
#[allow(dead_code)]
pub fn get_no_frame_css() -> String {
    r#"
    ._1Kg3ffZABPxXxDqcmoxkBA {
        padding-top: 30px !important;
        -webkit-app-region: drag;
    }
    .ms-FocusZone,
    ._3Nd2PGu67wifhuPZp2Sfj5 {
        -webkit-app-region: no-drag;
    }
    "#
    .to_string()
}

/// JavaScript that intercepts non-Microsoft link clicks and opens them externally
fn get_link_interceptor_js() -> &'static str {
    r#"
    (function() {
        if (window.__freelook_link_interceptor__) return;
        window.__freelook_link_interceptor__ = true;
        
        function isMsUrl(href) {
            if (!href || href === '#' || href.startsWith('javascript:')) return true;
            if (href.startsWith('/')) return true;
            if (href.startsWith('mailto:') || href.startsWith('tel:')) return true;
            if (href.startsWith('about:') || href.startsWith('data:')) return true;
            try {
                var u = new URL(href, location.href);
                if (u.protocol === 'blob:') {
                    try {
                        u = new URL(u.pathname);
                    } catch(e) {
                        return true;
                    }
                }
                var h = u.hostname.toLowerCase();
                return h === 'outlook.live.com' || h === 'outlook.office365.com'
                    || h === 'outlook.office.com' || h === 'login.microsoftonline.com'
                    || h === 'login.live.com' || h === 'www.office.com'
                    || h.endsWith('.outlook.com') || h.endsWith('.outlook.live.com')
                    || h.endsWith('.outlook.live.net') || h.endsWith('.office.net')
                    || h.endsWith('.office365.com') || h.endsWith('.office365.net')
                    || h.endsWith('.office.com') || h.endsWith('.officeppe.net')
                    || h.endsWith('.officeapps.live.com')
                    || h.endsWith('.microsoft.com') || h.endsWith('.microsoftonline.com')
                    || h.endsWith('.microsoftusercontent.com')
                    || h.endsWith('.msauth.net') || h.endsWith('.msftauth.net')
                    || h.endsWith('.msocdn.com')
                    || h.endsWith('.live.com') || h.endsWith('.1drv.ms')
                    || h.endsWith('.sharepoint.com') || h.endsWith('.sharepointonline.com')
                    || h.endsWith('.onenote.com') || h.endsWith('.onedrive.com')
                    || h.endsWith('.svc.ms');
            } catch(e) { return true; }
        }

        function isAttachmentOrPreviewUrl(href) {
            if (!href || href === '#' || href.startsWith('javascript:')) return false;
            try {
                var u = new URL(href, location.href);
                if (u.protocol === 'blob:') {
                    try {
                        u = new URL(u.pathname);
                    } catch(e) {
                        return false;
                    }
                }
                if (!isMsUrl(u.href)) return false;
                var value = u.href.toLowerCase();
                return value.includes('attachment')
                    || value.includes('download')
                    || value.includes('pdf')
                    || value.includes('mspdfkit')
                    || value.includes('fluidpreview')
                    || value.includes('officepreview')
                    || value.includes('wopi')
                    || value.includes('owa/service.svc');
            } catch(e) {
                return false;
            }
        }

        const openedPreviewUrls = new Set();

        function externalizePreviewUrl(href) {
            if (!isAttachmentOrPreviewUrl(href)) return;
            var absoluteUrl = new URL(href, location.href).href;
            if (absoluteUrl.startsWith('blob:')) return;
            if (openedPreviewUrls.has(absoluteUrl)) return;
            openedPreviewUrls.add(absoluteUrl);
            window.__TAURI__.core.invoke('open_external_url', { url: absoluteUrl });
        }

        function scanPreviewFrames(root) {
            (root || document).querySelectorAll('iframe[src], embed[src], object[data]').forEach(function(node) {
                externalizePreviewUrl(node.getAttribute('src') || node.getAttribute('data'));
            });
        }

        function filenameFromText(text) {
            if (!text) return null;
            var normalized = text.replace(/\s+/g, ' ').trim();
            var match = normalized.match(/[^\\/:*?"<>|\r\n]+?\.(?:pdf|docx?|xlsx?|pptx?|csv|txt|rtf|zip|7z|rar|png|jpe?g|gif|heic|webp|eml|msg)\b/i);
            if (match) return match[0].replace(/^(download|open|preview|attachment)\s+/i, '').trim();

            match = normalized.match(/\b[A-Za-z0-9][A-Za-z0-9_-]{5,}(?:\s+[A-Za-z0-9][A-Za-z0-9_-]{2,}){0,5}\b/);
            if (!match) return null;
            var candidate = match[0].replace(/^(download|open|preview|attachment)\s+/i, '').trim();
            if (!/[._-]/.test(candidate)) return null;
            return candidate + '.pdf';
        }

        function extractAttachmentName(target) {
            var node = target && target.nodeType === 1 ? target : target && target.parentElement;
            if (!node) return null;

            var candidates = [];
            var current = node;
            for (var depth = 0; current && depth < 6; depth++, current = current.parentElement) {
                ['aria-label', 'title', 'download', 'data-filename', 'data-file-name', 'data-name'].forEach(function(attr) {
                    var value = current.getAttribute && current.getAttribute(attr);
                    if (value) candidates.push(value);
                });
                if (current.innerText) candidates.push(current.innerText);
            }

            var attachmentNode = node.closest && node.closest('[aria-label*="Attachment"], [aria-label*="attachment"], [title*="Attachment"], [title*="attachment"], [role="listitem"], [role="button"]');
            if (attachmentNode) {
                ['aria-label', 'title'].forEach(function(attr) {
                    var value = attachmentNode.getAttribute && attachmentNode.getAttribute(attr);
                    if (value) candidates.push(value);
                });
                if (attachmentNode.innerText) candidates.push(attachmentNode.innerText);
            }

            document.querySelectorAll('[aria-label], [title], [role="button"], [role="listitem"]').forEach(function(item) {
                var text = '';
                ['aria-label', 'title'].forEach(function(attr) {
                    var value = item.getAttribute && item.getAttribute(attr);
                    if (value) text += ' ' + value;
                });
                if (item.innerText) text += ' ' + item.innerText;
                if (/\.(pdf|docx?|xlsx?|pptx?)\b/i.test(text) || /\b[A-Za-z0-9]+[_-][A-Za-z0-9_-]+\b/.test(text)) {
                    candidates.push(text);
                }
            });

            for (var i = 0; i < candidates.length; i++) {
                var name = filenameFromText(candidates[i]);
                if (name) return name;
            }
            return null;
        }

        function rememberDownloadName(e) {
            var name = extractAttachmentName(e.target);
            if (!name) return;
            window.__TAURI__.core.invoke('set_pending_download_name', { name: name });
        }

        function interceptClick(e) {
            var el = e.target;
            while (el && el.tagName !== 'A') el = el.parentElement;
            if (!el) return;
            var href = el.getAttribute('href');
            if (isAttachmentOrPreviewUrl(href)) {
                e.preventDefault();
                e.stopPropagation();
                externalizePreviewUrl(href);
            } else if (!isMsUrl(href)) {
                e.preventDefault();
                e.stopPropagation();
                window.__TAURI__.core.invoke('open_external_url', {
                    url: new URL(href, location.href).href
                });
            }
        }
        
        document.addEventListener('pointerdown', rememberDownloadName, true);
        document.addEventListener('mousedown', rememberDownloadName, true);
        document.addEventListener('click', rememberDownloadName, true);
        document.addEventListener('click', interceptClick, true);
        scanPreviewFrames(document);

        new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType !== 1) return;
                    if (node.matches && node.matches('iframe[src], embed[src], object[data]')) {
                        externalizePreviewUrl(node.getAttribute('src') || node.getAttribute('data'));
                    }
                    if (node.querySelectorAll) scanPreviewFrames(node);
                });
                if (mutation.type === 'attributes') {
                    externalizePreviewUrl(mutation.target.getAttribute('src') || mutation.target.getAttribute('data'));
                }
            });
        }).observe(document.documentElement, {
            childList: true,
            subtree: true,
            attributes: true,
            attributeFilter: ['src', 'data']
        });
        
        // Inject into same-origin iframes only (skip ads)
        function attachToMailIframes() {
            document.querySelectorAll('iframe').forEach(function(iframe) {
                var src = iframe.src || '';
                // Skip ad/tracking iframes
                if (src.includes('adnxs') || src.includes('adsdk') || src.includes('doubleclick') || src.includes('ads.')) return;
                try {
                    var doc = iframe.contentDocument;
                    if (doc && !doc.__freelook_interceptor__) {
                        doc.__freelook_interceptor__ = true;
                        doc.addEventListener('click', interceptClick, true);
                    }
                } catch(e) {} // Cross-origin, skip
            });
        }
        
        window.addEventListener('load', function() {
            attachToMailIframes();
            setTimeout(attachToMailIframes, 1000);
            setTimeout(attachToMailIframes, 3000);
        });
    })();
    "#
}

/// Get the unread observer JavaScript
pub fn get_unread_js(app: &AppHandle) -> String {
    let unread_class = get_config_string(app, "unreadMessageClass", "");
    if unread_class.is_empty() {
        return String::new();
    }

    format!(
        r#"
        setTimeout(() => {{
            let unreadSpan = document.querySelector(".{unread_class}");
            if (!unreadSpan) return;

            window.__TAURI__.core.invoke('update_unread', {{ hasUnread: unreadSpan.hasChildNodes() }});

            let observer = new MutationObserver(mutations => {{
                window.__TAURI__.core.invoke('update_unread', {{ hasUnread: unreadSpan.hasChildNodes() }});

                var messages = document.querySelectorAll('div[role="listbox"][aria-label="Message list"]');
                if (messages.length) {{
                    var unread = messages[0].querySelectorAll('div[aria-label^="Unread"]');
                    var body = "";
                    for (var i = 0; i < unread.length; i++) {{
                        if (body.length) body += "\\n";
                        body += unread[i].getAttribute("aria-label").substring(7, 127);
                    }}
                    if (unread.length) {{
                        window.__TAURI__.notification.sendNotification({{
                            title: "Freelook - receiving " + unread.length + " NEW mails",
                            body: body,
                        }});
                    }}
                }}
            }});

            observer.observe(unreadSpan, {{ childList: true }});

            // Reminder observer
            let reminders = document.getElementsByClassName("_1BWPyOkN5zNVyfbTDKK1gM");
            let height = 0;
            let reminderObserver = new MutationObserver(mutations => {{
                if (reminders[0].clientHeight > height) {{
                    window.__TAURI__.core.invoke('show_main_window');
                }}
                height = reminders[0].clientHeight;
            }});

            if (reminders.length) {{
                reminderObserver.observe(reminders[0], {{ childList: true }});
            }}
        }}, 10000);
        "#
    )
}
