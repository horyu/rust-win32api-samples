use windows::{core::*, Win32::Graphics::Gdi};

fn main() -> Result<()> {
    unsafe {
        let mut device_names = vec![];
        for i in 0.. {
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesa
            // > [out] lpDisplayDevice
            // > Before calling EnumDisplayDevices, you must initialize the cb member of DISPLAY_DEVICE to the size, in bytes, of DISPLAY_DEVICE.
            let mut device = Gdi::DISPLAY_DEVICEA::default();
            device.cb = std::mem::size_of::<Gdi::DISPLAY_DEVICEA>() as u32;
            if !Gdi::EnumDisplayDevicesA(None, i, &mut device, 0).as_bool() {
                break;
            }
            // https://stackoverflow.com/questions/26957186/checking-enumdisplaydevices-dwflags
            if device.StateFlags & Gdi::DISPLAY_DEVICE_ACTIVE != 0 {
                let p = device.DeviceName.as_ptr() as *const i8;
                let s = std::ffi::CStr::from_ptr(p).to_str().unwrap().to_owned();
                device_names.push(s);
            }
        }

        for device_name in &device_names {
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsa
            let mut mode = Gdi::DEVMODEA::default();
            if Gdi::EnumDisplaySettingsA(
                device_name.as_str(),
                Gdi::ENUM_CURRENT_SETTINGS,
                &mut mode,
            )
            .as_bool()
            {
                println!(
                    "{}: {}x{} @ {}Hz",
                    device_name, mode.dmPelsWidth, mode.dmPelsHeight, mode.dmDisplayFrequency
                );
            }
        }

        Ok(())
    }
}
