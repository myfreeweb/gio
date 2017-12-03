// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

use Action;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ActionMap(Object<ffi::GActionMap, ffi::GActionMapInterface>);

    match fn {
        get_type => || ffi::g_action_map_get_type(),
    }
}

pub trait ActionMapExt {
    fn add_action<P: IsA<Action>>(&self, action: &P);

    //fn add_action_entries<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, entries: /*Ignored*/&[&ActionEntry], user_data: P);

    fn lookup_action(&self, action_name: &str) -> Option<Action>;

    fn remove_action(&self, action_name: &str);
}

impl<O: IsA<ActionMap>> ActionMapExt for O {
    fn add_action<P: IsA<Action>>(&self, action: &P) {
        unsafe {
            ffi::g_action_map_add_action(self.to_glib_none().0, action.to_glib_none().0);
        }
    }

    //fn add_action_entries<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, entries: /*Ignored*/&[&ActionEntry], user_data: P) {
    //    unsafe { TODO: call ffi::g_action_map_add_action_entries() }
    //}

    fn lookup_action(&self, action_name: &str) -> Option<Action> {
        unsafe {
            from_glib_none(ffi::g_action_map_lookup_action(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }

    fn remove_action(&self, action_name: &str) {
        unsafe {
            ffi::g_action_map_remove_action(self.to_glib_none().0, action_name.to_glib_none().0);
        }
    }
}
