use windows::{
    core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Direct2D::*, Win32::Graphics::Gdi::*, Win32::System::Com::*,
    Win32::System::LibraryLoader::*, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?;
    }
    let mut window = Window::new()?;
    unsafe { window.run() }
}

pub struct Window {
    handle: HWND,
    factory: ID2D1Factory1,
    style: ID2D1StrokeStyle,
    target: Option<ID2D1HwndRenderTarget>,
    brush1: Option<ID2D1SolidColorBrush>,
    brush2: Option<ID2D1SolidColorBrush>,
}

impl Window {
    fn new() -> Result<Self> {
        let factory = create_factory()?;
        let style = create_style(&factory)?;
        Ok(Self {
            handle: HWND(0),
            factory,
            style,
            target: None,
            brush1: None,
            brush2: None,
        })
    }

    fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let hwnd = self.handle;
            let mut rect = RECT::default();

            unsafe {
                GetClientRect(self.handle, &mut rect);
            }

            let d2d_rect = D2D_SIZE_U {
                width: (rect.right - rect.left) as u32,
                height: (rect.bottom - rect.top) as u32,
            };

            let render_properties = D2D1_RENDER_TARGET_PROPERTIES::default();

            let hwnd_render_properties = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd,
                pixelSize: d2d_rect,
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };

            let gray = D2D1_COLOR_F {
                r: 0.345,
                g: 0.423,
                b: 0.463,
                a: 1.0,
            };
            let red = D2D1_COLOR_F {
                r: 0.941,
                g: 0.353,
                b: 0.392,
                a: 1.0,
            };

            let properties = D2D1_BRUSH_PROPERTIES {
                opacity: 0.8,
                transform: Matrix3x2::identity(),
            };

            let target = unsafe {
                self.factory
                    .CreateHwndRenderTarget(&render_properties, &hwnd_render_properties)?
            };
            let brush1 = unsafe { target.CreateSolidColorBrush(&gray, &properties)? };
            let brush2 = unsafe { target.CreateSolidColorBrush(&red, &properties)? };

            self.target = Some(target);
            self.brush1 = Some(brush1);
            self.brush2 = Some(brush2);
        }

        let target = self.target.as_ref().unwrap();
        unsafe {
            target.BeginDraw();
            self.draw(target)?;
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        };

        Ok(())
    }

    unsafe fn draw(&self, target: &ID2D1HwndRenderTarget) -> Result<()> {
        target.Clear(&D2D1_COLOR_F {
            r: 255.0,
            g: 255.0,
            b: 255.0,
            a: 1.0,
        });

        let render_size = target.GetSize();
        let brush1 = self.brush1.as_ref().unwrap();
        let brush2 = self.brush2.as_ref().unwrap();

        let mut count: f32 = 0.0;
        while count < render_size.width {
            target.DrawLine(
                D2D_POINT_2F { x: count, y: 0.0 },
                D2D_POINT_2F {
                    x: count,
                    y: render_size.height,
                },
                brush1,
                0.5,
                &self.style,
            );
            count += 10.0;
        }

        count = 0.0;
        while count < render_size.height {
            target.DrawLine(
                D2D_POINT_2F { x: 0.0, y: count },
                D2D_POINT_2F {
                    x: render_size.width,
                    y: count,
                },
                brush1,
                0.5,
                &self.style,
            );
            count += 10.0;
        }

        // Draw two rectangles.
        let rx = render_size.width / 2.0;
        let ry = render_size.height / 2.0;

        let rect1 = D2D_RECT_F {
            left: rx - 50.0,
            right: rx + 50.0,
            top: ry - 50.0,
            bottom: ry + 50.0,
        };
        let rect2 = D2D_RECT_F {
            left: rx - 100.0,
            right: rx + 100.0,
            top: ry - 100.0,
            bottom: ry + 100.0,
        };

        target.FillRectangle(&rect1, brush1);
        target.DrawRectangle(&rect2, brush2, 3.0, &self.style);

        Ok(())
    }

    unsafe fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                BeginPaint(self.handle, &mut ps);
                self.render().unwrap();
                EndPaint(self.handle, &ps);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(self.handle, message, wparam, lparam),
        }
    }

    unsafe fn run(&mut self) -> Result<()> {
        let instance = GetModuleHandleA(None);
        debug_assert!(instance.0 != 0);

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_HAND)?,
            hInstance: instance,
            lpszClassName: PCSTR(b"window\0".as_ptr()),

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let handle = CreateWindowExA(
            Default::default(),
            PCSTR(b"window\0".as_ptr()),
            PCSTR(b"Sample Window\0".as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            self as *mut _ as _,
        );

        debug_assert!(handle.0 != 0);
        debug_assert!(handle == self.handle);
        let mut message = MSG::default();

        loop {
            self.render()?;

            while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                if message.message == WM_QUIT {
                    return Ok(());
                }
                DispatchMessageA(&message);
            }
        }
    }

    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLongPtrA(window, GWLP_USERDATA, this as isize);
            } else {
                let this = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    let mut result = None;

    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            std::mem::transmute(&mut result),
        )
        .map(|()| result.unwrap())
    }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES {
        startCap: D2D1_CAP_STYLE_ROUND,
        endCap: D2D1_CAP_STYLE_TRIANGLE,
        ..Default::default()
    };

    unsafe { factory.CreateStrokeStyle(&props, &[]) }
}
