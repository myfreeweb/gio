// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Converter;
use Error;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CharsetConverter(Object<ffi::GCharsetConverter, ffi::GCharsetConverterClass, CharsetConverterClass>) @implements Converter;

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

pub const NONE_CHARSET_CONVERTER: Option<&CharsetConverter> = None;

pub trait CharsetConverterExt: 'static {
    fn get_num_fallbacks(&self) -> u32;

    fn get_use_fallback(&self) -> bool;

    fn set_use_fallback(&self, use_fallback: bool);

    fn get_property_from_charset(&self) -> Option<GString>;

    fn get_property_to_charset(&self) -> Option<GString>;

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CharsetConverter>> CharsetConverterExt for O {
    fn get_num_fallbacks(&self) -> u32 {
        unsafe {
            ffi::g_charset_converter_get_num_fallbacks(self.as_ref().to_glib_none().0)
        }
    }

    fn get_use_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_charset_converter_get_use_fallback(self.as_ref().to_glib_none().0))
        }
    }

    fn set_use_fallback(&self, use_fallback: bool) {
        unsafe {
            ffi::g_charset_converter_set_use_fallback(self.as_ref().to_glib_none().0, use_fallback.to_glib());
        }
    }

    fn get_property_from_charset(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"from-charset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_to_charset(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"to-charset\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::use-fallback\0".as_ptr() as *const _,
                transmute(notify_use_fallback_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_use_fallback_trampoline<P>(this: *mut ffi::GCharsetConverter, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CharsetConverter> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CharsetConverter::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CharsetConverter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CharsetConverter")
    }
}
