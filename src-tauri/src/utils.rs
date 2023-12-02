use parking_lot::Mutex;
use enigo::*;
use std::{thread, time::Duration};
use tauri::path::BaseDirectory;
use tauri::Manager;

use crate::APP_HANDLE;

static SELECT_ALL: Mutex<()> = Mutex::new(());

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn select_all(enigo: &mut Enigo) {
    let _guard = SELECT_ALL.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('a'));
    enigo.key_up(Key::Control);
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn select_all(enigo: &mut Enigo) {
    let _guard = SELECT_ALL.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/select-all.applescript", BaseDirectory::Resource)
        .expect("failed to resolve select-all.applescript");

    std::process::Command::new("osascript").arg(apple_script).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn select_all(enigo: &mut Enigo) {
    let _guard = SELECT_ALL.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('a'));
    enigo.key_up(Key::Control);
}

pub static INPUT_LOCK: Mutex<()> = Mutex::new(());

#[cfg(not(target_os = "macos"))]
pub fn left_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    for _ in 0..n {
        enigo.key_click(Key::LeftArrow);
    }
}

#[cfg(target_os = "macos")]
pub fn left_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/left.applescript", BaseDirectory::Resource)
        .expect("failed to resolve left.applescript");

    std::process::Command::new("osascript").arg(apple_script).arg(n.to_string()).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[cfg(not(target_os = "macos"))]
pub fn right_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    for _ in 0..n {
        enigo.key_click(Key::RightArrow);
    }
}

#[cfg(target_os = "macos")]
pub fn right_arrow_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/right.applescript", BaseDirectory::Resource)
        .expect("failed to resolve right.applescript");

    std::process::Command::new("osascript").arg(apple_script).arg(n.to_string()).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[cfg(not(target_os = "macos"))]
pub fn backspace_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    for _ in 0..n {
        enigo.key_click(Key::Backspace);
    }
}

#[cfg(target_os = "macos")]
pub fn backspace_click(enigo: &mut Enigo, n: usize) {
    let _guard = INPUT_LOCK.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/backspace.applescript", BaseDirectory::Resource)
        .expect("failed to resolve backspace.applescript");

    std::process::Command::new("osascript").arg(apple_script).arg(n.to_string()).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

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

static COPY_PASTE: Mutex<()> = Mutex::new(());

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn copy(enigo: &mut Enigo) {
    let _guard = COPY_PASTE.lock();

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
    let _guard = COPY_PASTE.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/copy.applescript", BaseDirectory::Resource)
        .expect("failed to resolve copy.applescript");

    std::process::Command::new("osascript").arg(apple_script).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn copy(enigo: &mut Enigo) {
    let _guard = COPY_PASTE.lock();

    up_control_keys(enigo);

    enigo.key_down(Key::Control);
    thread::sleep(Duration::from_millis(50));
    enigo.key_click(Key::Layout('c'));
    thread::sleep(Duration::from_millis(50));
    enigo.key_up(Key::Control);
}

#[allow(dead_code)]
#[cfg(target_os = "windows")]
pub fn paste(enigo: &mut Enigo) {
    let _guard = COPY_PASTE.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

#[allow(dead_code)]
#[cfg(target_os = "macos")]
pub fn paste(enigo: &mut Enigo) {
    let _guard = COPY_PASTE.lock();

    let apple_script = APP_HANDLE
        .get()
        .unwrap()
        .path()
        .resolve("resources/paste.applescript", BaseDirectory::Resource)
        .expect("failed to resolve paste.applescript");

    std::process::Command::new("osascript").arg(apple_script).spawn().expect("failed to run applescript").wait().expect("failed to wait");
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
pub fn paste(enigo: &mut Enigo) {
    let _guard = COPY_PASTE.lock();

    crate::utils::up_control_keys(enigo);

    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
}

#[cfg(not(target_os = "macos"))]
pub fn get_selected_text() -> Result<String, Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();
    get_selected_text_by_clipboard(&mut enigo, false)
}

pub fn get_selected_text_by_clipboard(enigo: &mut Enigo, cancel_select: bool) -> Result<String, Box<dyn std::error::Error>> {
    use arboard::Clipboard;

    let old_clipboard = (Clipboard::new()?.get_text(), Clipboard::new()?.get_image());

    let mut write_clipboard = Clipboard::new()?;

    let not_selected_placeholder = "";

    write_clipboard.set_text(not_selected_placeholder)?;

    thread::sleep(Duration::from_millis(50));

    copy(enigo);

    if cancel_select {
        right_arrow_click(enigo, 1);
    }

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
        .path()
        .resolve("resources/get-selected-text-by-ax.applescript", BaseDirectory::Resource)
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
        .path()
        .resolve("resources/get-selected-text.applescript", BaseDirectory::Resource)
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
        Some(handle) => handle.emit("change-text", text).unwrap_or_default(),
        None => {}
    }
}

pub fn writing_text(text: String) {
    match APP_HANDLE.get() {
        Some(handle) => handle.emit("writing-text", text).unwrap_or_default(),
        None => {}
    }
}

pub fn show() {
    match APP_HANDLE.get() {
        Some(handle) => handle.emit("show", "").unwrap_or_default(),
        None => {}
    }
}
