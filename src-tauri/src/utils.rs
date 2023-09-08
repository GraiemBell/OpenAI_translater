use tauri::Manager;
use parking_lot::Mutex;
use enigo::*;
use std::{thread, time::Duration};

use crate::APP_HANDLE;

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn up_control_keys(enigo: &mut Enigo) {
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_up(Key::Tab);
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn up_control_keys(enigo: &mut Enigo) {
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Meta);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_up(Key::Tab);
    enigo.key_up(Key::Option);
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn up_control_keys(enigo: &mut Enigo) {
    enigo.key_up(Key::Control);
    enigo.key_up(Key::Alt);
    enigo.key_up(Key::Shift);
    enigo.key_up(Key::Space);
    enigo.key_up(Key::Tab);
}

static COPY: Mutex<()> = Mutex::new(());

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn copy(enigo: &mut Enigo) {
    let _guard = COPY.lock();

    up_control_keys(enigo);

    enigo.key_down(Key::Control);
    thread::sleep(Duration::from_millis(50));
    enigo.key_click(Key::Layout('c'));
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::Control);
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn copy(enigo: &mut Enigo) {
    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .resolve_resource("resources/copy.applescript")
        .expect("failed to resolve copy.applescript");

    std::process::Command::new("osascript").arg(apple_script).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn copy(enigo: &mut Enigo) {
    let _guard = COPY.lock();

    up_control_keys(enigo);

    enigo.key_down(Key::Control);
    thread::sleep(Duration::from_millis(50));
    enigo.key_click(Key::Layout('c'));
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::Control);
}

static PASTE: Mutex<()> = Mutex::new(());

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn paste(enigo: &mut Enigo) {
    let __guard = PASTE.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn paste(enigo: &mut Enigo) {
    let __guard = PASTE.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Meta);
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn paste(enigo: &mut Enigo) {
    let __guard = PASTE.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

#[cfg(not(target_os = "macos"))]
pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();
    get_selected_text_by_clipboard(&mut enigo)
}

pub fn get_selected_text_by_clipboard(enigo: &mut Enigo) -> Result<String, Box<dyn std::error::Error>> {
    use arboard::Clipboard;

    let old_clipboard = (Clipboard::new()?.get_text(), Clipboard::new()?.get_image());

    let mut write_clipboard = Clipboard::new()?;

    let not_selected_placeholder = "";

    write_clipboard.set_text(not_selected_placeholder)?;

    thread::sleep(Duration::from_millis(50));

    copy(enigo);

    thread::sleep(Duration::from_millis(100));

    let new_text = Clipboard::new()?.get_text();

    match old_clipboard {
        (Ok(old_text), _) => {
            // Old Content is Text
            write_clipboard.set_text(old_text.clone())?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
        (_, Ok(image)) => {
            // Old Content is Image
            write_clipboard.set_image(image)?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
        _ => {
            // Old Content is Empty
            write_clipboard.clear()?;
            if let Ok(new) = new_text {
                if new.trim() == not_selected_placeholder.trim() {
                    Ok(String::new())
                } else {
                    Ok(new)
                }
            } else {
                Ok(String::new())
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    use crate::config::get_config;

    match get_selected_text_by_ax() {
        Ok(text) => Ok(text),
        Err(err) => {
            println!("get_selected_text_by_ax error: {}", err);
            match get_config() {
                Ok(config) => {
                    if config.allow_using_clipboard_when_selected_text_not_available.unwrap_or(false) {
                        get_selected_text_by_clipboard_using_applescript()
                    } else {
                        Ok(String::new())
                    }
                }
                Err(err) => {
                    println!("get_config error: {}", err);
                    Ok(String::new())
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_selected_text_by_ax() -> Result<String, Box<dyn std::error::Error>> {
    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .resolve_resource("resources/get-selected-text-by-ax.applescript")
        .expect("failed to resolve get-selected-text-by-ax.applescript");

    match std::process::Command::new("osascript")
        .arg(apple_script)
        .output()
    {
        Ok(output) => {
            // check exit code
            if output.status.success() {
                // get output content
                let content = String::from_utf8(output.stdout)
                    .expect("failed to parse get-selected-text-by-ax.applescript output");
                // trim content
                let content = content.trim();
                Ok(content.to_string())
            } else {
                let err = output
                    .stderr
                    .into_iter()
                    .map(|c| c as char)
                    .collect::<String>()
                    .into();
                Err(err)
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(target_os = "macos")]
pub fn get_selected_text_by_clipboard_using_applescript() -> Result<String, Box<dyn std::error::Error>> {
    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path_resolver()
        .resolve_resource("resources/get-selected-text.applescript")
        .expect("failed to resolve get-selected-text.applescript");

    match std::process::Command::new("osascript")
        .arg(apple_script)
        .output()
    {
        Ok(output) => {
            // check exit code
            if output.status.success() {
                // get output content
                let content = String::from_utf8(output.stdout)
                    .expect("failed to parse get-selected-text.applescript output");
                // trim content
                let content = content.trim();
                Ok(content.to_string())
            } else {
                let err = output
                    .stderr
                    .into_iter()
                    .map(|c| c as char)
                    .collect::<String>()
                    .into();
                Err(err)
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn send_text(text: String) {
    match APP_HANDLE.get() {
        Some(handle) => handle.emit_all("change-text", text).unwrap_or_default(),
        None => {}
    }
}

pub fn writing_text(text: String) {
    match APP_HANDLE.get() {
        Some(handle) => handle.emit_all("writing-text", text).unwrap_or_default(),
        None => {}
    }
}

pub fn show() {
    match APP_HANDLE.get() {
        Some(handle) => handle.emit_all("show", "").unwrap_or_default(),
        None => {}
    }
}
