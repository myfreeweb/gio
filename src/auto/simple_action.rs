// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Action;
use ffi;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SimpleAction(Object<ffi::GSimpleAction, SimpleActionClass>) @implements Action;

    match fn {
        get_type => || ffi::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    pub fn new<'a, P: Into<Option<&'a glib::VariantTy>>>(name: &str, parameter_type: P) -> SimpleAction {
        let parameter_type = parameter_type.into();
        unsafe {
            from_glib_full(ffi::g_simple_action_new(name.to_glib_none().0, parameter_type.to_glib_none().0))
        }
    }

    pub fn new_stateful<'a, P: Into<Option<&'a glib::VariantTy>>>(name: &str, parameter_type: P, state: &glib::Variant) -> SimpleAction {
        let parameter_type = parameter_type.into();
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(name.to_glib_none().0, parameter_type.to_glib_none().0, state.to_glib_none().0))
        }
    }
}

pub const NONE_SIMPLE_ACTION: Option<&SimpleAction> = None;

pub trait SimpleActionExt: 'static {
    fn set_enabled(&self, enabled: bool);

    fn set_state(&self, value: &glib::Variant);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn set_state_hint<'a, P: Into<Option<&'a glib::Variant>>>(&self, state_hint: P);

    fn connect_activate<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_change_state<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SimpleAction>> SimpleActionExt for O {
    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::g_simple_action_set_enabled(self.as_ref().to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn set_state_hint<'a, P: Into<Option<&'a glib::Variant>>>(&self, state_hint: P) {
        let state_hint = state_hint.into();
        unsafe {
            ffi::g_simple_action_set_state_hint(self.as_ref().to_glib_none().0, state_hint.to_glib_none().0);
        }
    }

    fn connect_activate<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"activate\0".as_ptr() as *const _,
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_change_state<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"change-state\0".as_ptr() as *const _,
                transmute(change_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::enabled\0".as_ptr() as *const _,
                transmute(notify_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::state-type\0".as_ptr() as *const _,
                transmute(notify_state_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GSimpleAction, parameter: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    let f: &&(Fn(&P, &Option<glib::Variant>) + 'static) = transmute(f);
    f(&SimpleAction::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(parameter))
}

unsafe extern "C" fn change_state_trampoline<P>(this: *mut ffi::GSimpleAction, value: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    let f: &&(Fn(&P, &Option<glib::Variant>) + 'static) = transmute(f);
    f(&SimpleAction::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(value))
}

unsafe extern "C" fn notify_enabled_trampoline<P>(this: *mut ffi::GSimpleAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SimpleAction::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_state_type_trampoline<P>(this: *mut ffi::GSimpleAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SimpleAction::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SimpleAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleAction")
    }
}
