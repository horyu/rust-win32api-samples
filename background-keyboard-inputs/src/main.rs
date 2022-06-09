use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

fn main() -> Result<()> {
    unsafe {
        let k_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(k_callback1), HINSTANCE::default(), 0);
        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            DispatchMessageA(&message);
        }
        if !k_hook.is_invalid() {
            UnhookWindowsHookEx(k_hook);
        }
        Ok(())
    }
}

static mut INPUTS_ARRAY: [bool; 256] = [false; 256];
unsafe fn set_and_show(vk_code: u32, tf: bool) {
    INPUTS_ARRAY[vk_code as usize] = tf;
    let mut s = String::with_capacity((b'Z' - b'A' + 1) as usize);
    for i in (b'A' as usize)..=(b'Z' as usize) {
        s.push(if INPUTS_ARRAY[i] { 'T' } else { 'F' });
    }
    println!("{s}");
}

extern "system" fn k_callback1(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if ncode as u32 == HC_ACTION {
            match wparam.0 as u32 {
                WM_KEYDOWN => {
                    let kb_hook = std::mem::transmute::<isize, &KBDLLHOOKSTRUCT>(lparam.0);
                    set_and_show(kb_hook.vkCode, true);
                },
                WM_KEYUP => {
                    let kb_hook = std::mem::transmute::<isize, &KBDLLHOOKSTRUCT>(lparam.0);
                    set_and_show(kb_hook.vkCode, false);
                },
                _ => (),
            }
        }
        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}
