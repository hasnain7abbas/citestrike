use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_CONTROL, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, IsWindowVisible, SetForegroundWindow,
};
use std::sync::Mutex;

static FOUND_HWND: Mutex<Option<isize>> = Mutex::new(None);

fn find_window_by_title(title_part: &str) -> Option<HWND> {
    let search = title_part.to_lowercase();
    *FOUND_HWND.lock().unwrap() = None;

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let search_ptr = lparam.0 as *const String;
        let search = &*search_ptr;

        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len > 0 {
            let title = String::from_utf16_lossy(&buf[..len as usize]).to_lowercase();
            if title.contains(search.as_str()) {
                *FOUND_HWND.lock().unwrap() = Some(hwnd.0 as isize);
                return BOOL(0);
            }
        }
        BOOL(1)
    }

    let search_box = Box::new(search);
    let search_ptr = &*search_box as *const String as isize;

    unsafe {
        let _ = EnumWindows(Some(enum_callback), LPARAM(search_ptr));
    }

    FOUND_HWND
        .lock()
        .unwrap()
        .map(|h| HWND(h as *mut _))
}

fn simulate_paste() {
    let mut inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: Default::default(),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn insert_into_word() -> Result<String, String> {
    let hwnd = find_window_by_title("- word")
        .or_else(|| find_window_by_title("- microsoft word"))
        .or_else(|| find_window_by_title(".docx"))
        .or_else(|| find_window_by_title(".doc"))
        .ok_or("No Microsoft Word window found. Please open a document in Word first.")?;

    let mut buf = [0u16; 512];
    let title = unsafe {
        let len = GetWindowTextW(hwnd, &mut buf);
        String::from_utf16_lossy(&buf[..len as usize])
    };

    unsafe { let _ = SetForegroundWindow(hwnd); }
    std::thread::sleep(std::time::Duration::from_millis(200));
    simulate_paste();

    Ok(title)
}

pub fn insert_into_powerpoint() -> Result<String, String> {
    let hwnd = find_window_by_title("- powerpoint")
        .or_else(|| find_window_by_title("- microsoft powerpoint"))
        .or_else(|| find_window_by_title(".pptx"))
        .or_else(|| find_window_by_title(".ppt"))
        .ok_or("No Microsoft PowerPoint window found. Please open a presentation in PowerPoint first.")?;

    let mut buf = [0u16; 512];
    let title = unsafe {
        let len = GetWindowTextW(hwnd, &mut buf);
        String::from_utf16_lossy(&buf[..len as usize])
    };

    unsafe { let _ = SetForegroundWindow(hwnd); }
    std::thread::sleep(std::time::Duration::from_millis(200));
    simulate_paste();

    Ok(title)
}
