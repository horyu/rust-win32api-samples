use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{LibraryLoader::GetModuleHandleA, Threading::GetCurrentThreadId},
        UI::WindowsAndMessaging::*,
    },
};

#[derive(Debug)]
struct HookManager {
    hook: HHOOK,
}

impl Drop for HookManager {
    fn drop(&mut self) {
        unsafe {
            let _ = UnhookWindowsHookEx(self.hook);
        }
    }
}

fn main() -> Result<()> {
    unsafe {
        let _wh_keyboard_ll_hook_manager = HookManager {
            hook: SetWindowsHookExA(
                WH_KEYBOARD_LL,
                Some(wh_keyboard_ll_callback),
                HINSTANCE::default(),
                0,
            )?,
        };

        let _wh_keyboard_hook_manager = HookManager {
            hook: SetWindowsHookExA(
                WH_KEYBOARD,
                Some(wh_keyboard_callback),
                HINSTANCE::default(),
                GetCurrentThreadId(),
            )?,
        };

        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");
        let wc = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            lpszClassName: window_class,
            ..WNDCLASSA::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("KEYBOAD LISTENER"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            200,
            150,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();
        let hwnd = HWND::default();
        while GetMessageA(&mut message, hwnd, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
    Ok(())
}

extern "system" fn wh_keyboard_ll_callback(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if ncode as u32 == HC_ACTION {
        match wparam.0 as u32 {
            WM_KEYDOWN => {
                log_wh_keyboard_ll("WM_KEYDOWN", lparam);
            }
            WM_KEYUP => {
                log_wh_keyboard_ll("WM_KEYUP", lparam);
            }
            WM_SYSKEYDOWN => {
                log_wh_keyboard_ll("WM_SYSKEYDOWN", lparam);
            }
            WM_SYSKEYUP => {
                log_wh_keyboard_ll("WM_SYSKEYUP", lparam);
            }
            _ => (),
        }
    }
    unsafe { CallNextHookEx(HHOOK::default(), ncode, wparam, lparam) }
}

fn log_wh_keyboard_ll(message_type: &'static str, kbd_ll_hook_struct_ptr: LPARAM) {
    let kbd_ll_hook_struct = unsafe { &*(kbd_ll_hook_struct_ptr.0 as *const KBDLLHOOKSTRUCT) };
    println!(
        "[WH_KEYBOARD_LL] {}: vkCode: {}({:#02x}), scanCode: {}({:#02x}), flags: {}({:#b}), time: {}, dwExtraInfo: {}",
        message_type,
        kbd_ll_hook_struct.vkCode,
        kbd_ll_hook_struct.vkCode,
        kbd_ll_hook_struct.scanCode,
        kbd_ll_hook_struct.scanCode,
        kbd_ll_hook_struct.flags.0,
        kbd_ll_hook_struct.flags.0,
        kbd_ll_hook_struct.time,
        kbd_ll_hook_struct.dwExtraInfo
    );
}

extern "system" fn wh_keyboard_callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if 0 <= code {
        log_wh_keyboard(code, wparam, lparam);
    }
    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}

fn log_wh_keyboard(code: i32, wparam: WPARAM, lparam: LPARAM) {
    println!(
        "[WH_KEYBOARD] code: {code}, wparam: {}({:#02x}), lparam: {}({:#032b})",
        wparam.0, wparam.0, lparam.0, lparam.0
    );
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                // eprintln!("WM_PAINT");
                return LRESULT(0);
            }
            WM_DESTROY => {
                eprintln!("WM_DESTROY");
                PostQuitMessage(0);
                return LRESULT(0);
            }
            WM_KEYDOWN => {
                log_wndproc("WM_KEYDOWN", wparam, lparam);
            }
            WM_KEYUP => {
                log_wndproc("WM_KEYUP", wparam, lparam);
            }
            WM_CHAR => {
                log_wndproc("WM_CHAR", wparam, lparam);
            }
            WM_SYSCHAR => {
                log_wndproc("WM_SYSCHAR", wparam, lparam);
            }
            _ => (),
        }
        DefWindowProcA(window, message, wparam, lparam)
    }
}

fn log_wndproc(message_type: &'static str, wparam: WPARAM, lparam: LPARAM) {
    println!(
        "[WNDPROC] {}: wparam: {}({:#x}), lparam: {}({:#032b})",
        message_type, wparam.0, wparam.0, lparam.0, lparam.0
    );
}
