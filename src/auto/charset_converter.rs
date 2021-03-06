// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Converter;
use Error;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CharsetConverter(Object<ffi::GCharsetConverter, ffi::GCharsetConverterClass>): Converter;

    match fn {
        get_type => || ffi::g_charset_converter_get_type(),
    }
}

impl CharsetConverter {
    pub fn new(to_charset: &str, from_charset: &str) -> Result<CharsetConverter, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_charset_converter_new(to_charset.to_glib_none().0, from_charset.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait CharsetConverterExt {
    fn get_num_fallbacks(&self) -> u32;

    fn get_use_fallback(&self) -> bool;

    fn set_use_fallback(&self, use_fallback: bool);

    fn get_property_from_charset(&self) -> Option<String>;

    fn get_property_to_charset(&self) -> Option<String>;

    fn connect_property_from_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_to_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CharsetConverter> + IsA<glib::object::Object>> CharsetConverterExt for O {
    fn get_num_fallbacks(&self) -> u32 {
        unsafe {
            ffi::g_charset_converter_get_num_fallbacks(self.to_glib_none().0)
        }
    }

    fn get_use_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_charset_converter_get_use_fallback(self.to_glib_none().0))
        }
    }

    fn set_use_fallback(&self, use_fallback: bool) {
        unsafe {
            ffi::g_charset_converter_set_use_fallback(self.to_glib_none().0, use_fallback.to_glib());
        }
    }

    fn get_property_from_charset(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "from-charset".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_to_charset(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "to-charset".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_from_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::from-charset",
                transmute(notify_from_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_to_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::to-charset",
                transmute(notify_to_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-fallback",
                transmute(notify_use_fallback_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_from_charset_trampoline<P>(this: *mut ffi::GCharsetConverter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CharsetConverter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CharsetConverter::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_to_charset_trampoline<P>(this: *mut ffi::GCharsetConverter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CharsetConverter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CharsetConverter::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_fallback_trampoline<P>(this: *mut ffi::GCharsetConverter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CharsetConverter> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CharsetConverter::from_glib_borrow(this).downcast_unchecked())
}
