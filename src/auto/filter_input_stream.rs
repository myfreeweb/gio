// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use InputStream;
use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FilterInputStream(Object<gio_sys::GFilterInputStream, gio_sys::GFilterInputStreamClass, FilterInputStreamClass>) @extends InputStream;

    match fn {
        get_type => || gio_sys::g_filter_input_stream_get_type(),
    }
}

pub const NONE_FILTER_INPUT_STREAM: Option<&FilterInputStream> = None;

pub trait FilterInputStreamExt: 'static {
    fn get_base_stream(&self) -> Option<InputStream>;

    fn get_close_base_stream(&self) -> bool;

    fn set_close_base_stream(&self, close_base: bool);

    fn connect_property_close_base_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilterInputStream>> FilterInputStreamExt for O {
    fn get_base_stream(&self) -> Option<InputStream> {
        unsafe {
            from_glib_none(gio_sys::g_filter_input_stream_get_base_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn get_close_base_stream(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_filter_input_stream_get_close_base_stream(self.as_ref().to_glib_none().0))
        }
    }

    fn set_close_base_stream(&self, close_base: bool) {
        unsafe {
            gio_sys::g_filter_input_stream_set_close_base_stream(self.as_ref().to_glib_none().0, close_base.to_glib());
        }
    }

    fn connect_property_close_base_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_close_base_stream_trampoline<P, F: Fn(&P) + 'static>(this: *mut gio_sys::GFilterInputStream, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FilterInputStream>
        {
            let f: &F = &*(f as *const F);
            f(&FilterInputStream::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::close-base-stream\0".as_ptr() as *const _,
                Some(transmute(notify_close_base_stream_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FilterInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterInputStream")
    }
}
