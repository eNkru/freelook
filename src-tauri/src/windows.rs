use std::sync::Mutex;
use std::time::Instant;
use tauri::{webview::PageLoadEvent, AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
    match url.host_str() {
        Some(host) => {
            let h = host.to_lowercase();
            h == "outlook.live.com"
                || h == "outlook.office365.com"
                || h == "outlook.office.com"
                || h == "login.microsoftonline.com"
                || h == "login.live.com"
                || h == "www.office.com"
                || h.ends_with(".outlook.com")
                || h.ends_with(".outlook.live.com")
                || h.ends_with(".office365.com")
                || h.ends_with(".office.com")
                || h.ends_with(".microsoft.com")
                || h.ends_with(".microsoftonline.com")
                || h.ends_with(".live.com")
                || h.ends_with(".sharepoint.com")
                || h.ends_with(".onenote.com")
        }
        None => false,
    }
}

/// Check if a URL is an ad/tracking URL that should be silently blocked
fn is_ad_url(url: &url::Url) -> bool {
    // Block about:blank navigations from ad iframes
    if url.scheme() == "about" {
        return true;
    }
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
    .on_navigation(move |url| {
        if is_microsoft_url(url) {
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
        if is_microsoft_url(&url) {
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
            try {
                var u = new URL(href);
                var h = u.hostname.toLowerCase();
                return h === 'outlook.live.com' || h === 'outlook.office365.com'
                    || h === 'outlook.office.com' || h === 'login.microsoftonline.com'
                    || h === 'login.live.com' || h === 'www.office.com'
                    || h.endsWith('.outlook.com') || h.endsWith('.outlook.live.com')
                    || h.endsWith('.office365.com') || h.endsWith('.office.com')
                    || h.endsWith('.microsoft.com') || h.endsWith('.microsoftonline.com')
                    || h.endsWith('.live.com') || h.endsWith('.sharepoint.com')
                    || h.endsWith('.onenote.com');
            } catch(e) { return true; }
        }
        
        function interceptClick(e) {
            var el = e.target;
            while (el && el.tagName !== 'A') el = el.parentElement;
            if (!el) return;
            var href = el.getAttribute('href');
            if (!isMsUrl(href)) {
                e.preventDefault();
                e.stopPropagation();
                window.__TAURI__.core.invoke('open_external_url', { url: href });
            }
        }
        
        document.addEventListener('click', interceptClick, true);
        
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
