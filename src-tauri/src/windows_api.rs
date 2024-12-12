extern crate winapi;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    CallNextHookEx, SetWindowsHookExA, KBDLLHOOKSTRUCT, VK_LWIN, VK_RWIN, WH_KEYBOARD_LL,
};
use winapi::um::winuser::{DispatchMessageA, PeekMessageA, MSG, PM_REMOVE};

use std::ptr::null_mut;

// Hook procedure
unsafe extern "system" fn hook_proc(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let key_struct: &KBDLLHOOKSTRUCT = &*(l_param as *const KBDLLHOOKSTRUCT);
        println!("Key: {}", key_struct.vkCode);
        if key_struct.vkCode == VK_LWIN as u32 || key_struct.vkCode == VK_RWIN as u32 {
            // Intercept Windows key
            return 1; // Block the key
        }
    }
    CallNextHookEx(null_mut(), code, w_param, l_param)
}

pub(crate) fn block_windows_key() {
    unsafe {
        let h_instance = GetModuleHandleA(null_mut());
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_proc), h_instance, 0);

        if hook.is_null() {
            eprintln!("Failed to set hook!");
            return;
        }
        let mut msg: MSG = std::mem::zeroed();
        while PeekMessageA(&mut msg, null_mut(), 0, 0, PM_REMOVE) > 0 {
            DispatchMessageA(&msg);
        }
    }
}
