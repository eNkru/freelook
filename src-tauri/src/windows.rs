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
#[allow(dead_code)]
fn set_config_value(app: &AppHandle, key: &str, value: &str) {
    if let Ok(store) = tauri_plugin_store::StoreBuilder::new(app, "Settings").build() {
        store.set(
            key.to_string(),
            serde_json::Value::String(value.to_string()),
        );
        let _ = store.save();
    }
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
    .inner_size(600.0, 800.0)
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
                            title: "Microsoft Outlook - receiving " + unread.length + " NEW mails",
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
