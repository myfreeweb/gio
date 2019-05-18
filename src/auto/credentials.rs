// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use gio_sys;
use glib::GString;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct Credentials(Object<gio_sys::GCredentials, gio_sys::GCredentialsClass, CredentialsClass>);

    match fn {
        get_type => || gio_sys::g_credentials_get_type(),
    }
}

impl Credentials {
    pub fn new() -> Credentials {
        unsafe {
            from_glib_full(gio_sys::g_credentials_new())
        }
    }

    //pub fn get_native(&self, native_type: CredentialsType) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gio_sys:g_credentials_get_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    pub fn get_unix_pid(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_credentials_get_unix_pid(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    pub fn get_unix_user(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_credentials_get_unix_user(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn is_same_user(&self, other_credentials: &Credentials) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_credentials_is_same_user(self.to_glib_none().0, other_credentials.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn set_native(&self, native_type: CredentialsType, native: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call gio_sys:g_credentials_set_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    pub fn set_unix_user(&self, uid: u32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_credentials_set_unix_user(self.to_glib_none().0, uid, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gio_sys::g_credentials_to_string(self.to_glib_none().0))
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Credentials")
    }
}
