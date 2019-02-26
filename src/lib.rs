//! A library for capturing screenshots on X11.

#![warn(missing_docs)]

#[cfg(feature = "image-interop")]
extern crate image;
extern crate x11;
#[cfg(feature = "image-interop")]
use self::image::RgbImage;
use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
    slice,
};
use x11::xlib;

/// A handle to an X11 screen.
pub struct Screen {
    display: *mut xlib::Display,
    screen: *mut xlib::Screen,
    window: xlib::Window,
}

/// A BGR pixel.
///
/// `X11Image`'s pixels are in this format.
#[derive(Debug)]
pub struct Bgr {
    /// Blue
    pub b: u8,
    /// Green
    pub g: u8,
    /// Red
    pub r: u8,
    _pad: u8,
}

/// A captured X11 image.
pub struct X11Image<'a> {
    ximage: NonNull<xlib::XImage>,
    screen: PhantomData<&'a Screen>,
}

impl<'a> X11Image<'a> {
    /// Returns a BGR pixel at the provided coordinates.
    pub fn pixel_at(&self, x: i32, y: i32) -> &Bgr {
        let image = unsafe { self.ximage.as_ref() };
        let sl = self.data_as_slice();
        &sl[y as usize * image.width as usize + x as usize]
    }
    fn data_as_slice(&self) -> &[Bgr] {
        let image = unsafe { self.ximage.as_ref() };
        unsafe {
            slice::from_raw_parts(
                image.data as *const _,
                image.width as usize * image.height as usize,
            )
        }
    }
}

/// Turn this `X11Image` into an `ImageBuffer` from the `image` crate.
///
/// Using `ImageBuffer`, you have access to features such as saving the screenshot, using the
/// `save` method.
#[cfg(feature = "image-interop")]
impl<'a> Into<RgbImage> for X11Image<'a> {
    fn into(self) -> RgbImage {
        let image = unsafe { self.ximage.as_ref() };
        let sl = self.data_as_slice();

        let mut bgr_iter = sl.iter();
        let mut image_buffer = RgbImage::new(image.width as u32, image.height as u32);

        for pix in image_buffer.pixels_mut() {
            let bgr = bgr_iter.next().unwrap();
            pix.data = [bgr.r, bgr.g, bgr.b];
        }

        image_buffer
    }
}

impl<'a> Drop for X11Image<'a> {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyImage(self.ximage.as_ptr());
        }
    }
}

impl Screen {
    /// Tries to open the X11 display, then returns a handle to the default screen.
    ///
    /// Returns `None` if the display could not be opened.
    pub fn open() -> Option<Screen> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return None;
            }
            let screen = xlib::XDefaultScreenOfDisplay(display);
            let root = xlib::XRootWindowOfScreen(screen);
            Some(Screen {
                display,
                screen,
                window: root,
            })
        }
    }
    /// Tries to capture a screenshot of the entire screen.
    ///
    /// Returns an `X11Image` on success, `None` on failure.
    pub fn capture(&self) -> Option<X11Image> {
        let screen: &mut xlib::Screen = &mut unsafe { *self.screen };
        self.capture_area(screen.width as u32, screen.height as u32, 0, 0)
    }
    /// Tries to capture a screenshot of the provided area.
    ///
    /// Returns an `X11Image` on success, `None` on failure.
    pub fn capture_area(&self, w: u32, h: u32, x: i32, y: i32) -> Option<X11Image> {
        let img = unsafe {
            xlib::XGetImage(
                self.display,
                self.window,
                x,
                y,
                w,
                h,
                !1,
                xlib::ZPixmap,
            )
        };

        NonNull::new(img).map(|ximage| X11Image {
            ximage,
            screen: PhantomData,
        })
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
