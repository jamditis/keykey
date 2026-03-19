use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub work_x: i32,
    pub work_y: i32,
    pub work_width: i32,
    pub work_height: i32,
    pub dpi: u32,
    pub scale_factor: f64,
    pub is_primary: bool,
}

#[cfg(target_os = "windows")]
pub mod platform {
    use super::MonitorInfo;
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM, POINT, RECT, TRUE};
    use windows::Win32::Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, MonitorFromPoint, MonitorFromWindow,
        HDC, HMONITOR, MONITORINFOEXW, MONITOR_DEFAULTTONEAREST,
    };
    use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
    use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetForegroundWindow};

    struct MonitorCollector {
        monitors: Vec<MonitorInfo>,
    }

    unsafe extern "system" fn enum_monitor_callback(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let collector = &mut *(lparam.0 as *mut MonitorCollector);

        let mut info = MONITORINFOEXW::default();
        info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        if GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _).as_bool() {
            let rc = info.monitorInfo.rcMonitor;
            let work = info.monitorInfo.rcWork;
            let is_primary = (info.monitorInfo.dwFlags & 1) != 0;

            let name = String::from_utf16_lossy(
                &info.szDevice[..info
                    .szDevice
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(32)],
            );

            let mut dpi_x: u32 = 96;
            let mut dpi_y: u32 = 96;
            let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);

            let scale_factor = dpi_x as f64 / 96.0;

            collector.monitors.push(MonitorInfo {
                name,
                x: rc.left,
                y: rc.top,
                width: rc.right - rc.left,
                height: rc.bottom - rc.top,
                work_x: work.left,
                work_y: work.top,
                work_width: work.right - work.left,
                work_height: work.bottom - work.top,
                dpi: dpi_x,
                scale_factor,
                is_primary,
            });
        }

        TRUE
    }

    pub fn enumerate_monitors() -> Vec<MonitorInfo> {
        let mut collector = MonitorCollector {
            monitors: Vec::new(),
        };

        unsafe {
            let _ = EnumDisplayMonitors(
                HDC::default(),
                None,
                Some(enum_monitor_callback),
                LPARAM(&mut collector as *mut MonitorCollector as isize),
            );
        }

        collector.monitors
    }

    fn monitor_info_from_hmonitor(hmonitor: HMONITOR) -> Option<MonitorInfo> {
        if hmonitor == HMONITOR::default() {
            return None;
        }

        unsafe {
            let mut info = MONITORINFOEXW::default();
            info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

            if !GetMonitorInfoW(hmonitor, &mut info.monitorInfo as *mut _ as *mut _).as_bool() {
                return None;
            }

            let rc = info.monitorInfo.rcMonitor;
            let work = info.monitorInfo.rcWork;
            let is_primary = (info.monitorInfo.dwFlags & 1) != 0;

            let name = String::from_utf16_lossy(
                &info.szDevice[..info
                    .szDevice
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(32)],
            );

            let mut dpi_x: u32 = 96;
            let mut dpi_y: u32 = 96;
            let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);

            let scale_factor = dpi_x as f64 / 96.0;

            Some(MonitorInfo {
                name,
                x: rc.left,
                y: rc.top,
                width: rc.right - rc.left,
                height: rc.bottom - rc.top,
                work_x: work.left,
                work_y: work.top,
                work_width: work.right - work.left,
                work_height: work.bottom - work.top,
                dpi: dpi_x,
                scale_factor,
                is_primary,
            })
        }
    }

    pub fn get_active_monitor() -> Option<MonitorInfo> {
        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            let hmonitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            monitor_info_from_hmonitor(hmonitor)
        }
    }

    pub fn get_cursor_monitor() -> Option<MonitorInfo> {
        unsafe {
            let mut pt = POINT { x: 0, y: 0 };
            if GetCursorPos(&mut pt).is_err() {
                return None;
            }
            let hmonitor = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
            monitor_info_from_hmonitor(hmonitor)
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub mod platform {
    use super::MonitorInfo;

    pub fn enumerate_monitors() -> Vec<MonitorInfo> {
        vec![]
    }

    pub fn get_active_monitor() -> Option<MonitorInfo> {
        None
    }

    pub fn get_cursor_monitor() -> Option<MonitorInfo> {
        None
    }
}
