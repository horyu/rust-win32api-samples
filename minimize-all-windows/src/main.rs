use windows::{
    Win32::{
        Foundation::{BOOL, FALSE, HWND, LPARAM, TRUE, WPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, FindWindowW, GetWindowTextW, SW_RESTORE, SendMessageW, ShowWindow,
            WM_COMMAND,
        },
    },
    core::w,
};

fn main() {
    if let Err(e) = trt_main() {
        dbg!(e);
    }
}

fn trt_main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // https://stackoverflow.com/questions/13942765/minimize-all-open-windows
        let hwnd = FindWindowW(w!("Shell_TrayWnd"), None)?;
        let _ = dbg!(SendMessageW(
            hwnd,
            WM_COMMAND,
            Some(WPARAM(419)),
            Some(LPARAM(0))
        ));

        // Using cfg for such functionality is not recommended.
        // Consider using runtime configuration or environment variables instead.
        if cfg!(feature = "reopen-cookie-clicker") {
            std::thread::sleep(std::time::Duration::from_millis(256));
            let mut cc_hwnd = HWND::default();
            if EnumWindows(
                Some(enum_window_proc),
                LPARAM(&mut cc_hwnd as *mut _ as isize),
            )
            .is_err()
            {
                assert_ne!(cc_hwnd, HWND::default());
                let _ = dbg!(ShowWindow(cc_hwnd, SW_RESTORE));
            }
        }
    }
    Ok(())
}

/// https://wisdom.sakura.ne.jp/system/winapi/win32/win142.html
extern "system" fn enum_window_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut buf = vec![0u16; 64];
    let len = unsafe { GetWindowTextW(hwnd, &mut buf) };
    if len == 0 {
        return TRUE;
    }
    let title = String::from_utf16_lossy(&buf[..(len as usize)]);
    if title.ends_with("- Cookie Clicker") {
        let cc_hwnd_ref = unsafe { &mut *(lparam.0 as *mut HWND) };
        *cc_hwnd_ref = hwnd;
        return FALSE;
    }
    TRUE
}
