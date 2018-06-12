// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DriveStartStopType;
use Icon;
use Volume;
use ffi;
use glib;
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
    pub struct Drive(Object<ffi::GDrive, ffi::GDriveIface>);

    match fn {
        get_type => || ffi::g_drive_get_type(),
    }
}

pub trait DriveExt {
    fn can_eject(&self) -> bool;

    fn can_poll_for_media(&self) -> bool;

    fn can_start(&self) -> bool;

    fn can_start_degraded(&self) -> bool;

    fn can_stop(&self) -> bool;

    //#[deprecated]
    //fn eject<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, cancellable: P, callback: Q, user_data: R);

    //fn eject_with_operation<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S);

    fn enumerate_identifiers(&self) -> Vec<String>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_identifier(&self, kind: &str) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_sort_key(&self) -> Option<String>;

    fn get_start_stop_type(&self) -> DriveStartStopType;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon>;

    fn get_volumes(&self) -> Vec<Volume>;

    fn has_media(&self) -> bool;

    fn has_volumes(&self) -> bool;

    fn is_media_check_automatic(&self) -> bool;

    fn is_media_removable(&self) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn is_removable(&self) -> bool;

    //fn poll_for_media<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //fn start<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: DriveStartFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S);

    //fn stop<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Drive> + IsA<glib::object::Object>> DriveExt for O {
    fn can_eject(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_eject(self.to_glib_none().0))
        }
    }

    fn can_poll_for_media(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_poll_for_media(self.to_glib_none().0))
        }
    }

    fn can_start(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_start(self.to_glib_none().0))
        }
    }

    fn can_start_degraded(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_start_degraded(self.to_glib_none().0))
        }
    }

    fn can_stop(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_can_stop(self.to_glib_none().0))
        }
    }

    //fn eject<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::g_drive_eject() }
    //}

    //fn eject_with_operation<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S) {
    //    unsafe { TODO: call ffi::g_drive_eject_with_operation() }
    //}

    fn enumerate_identifiers(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_enumerate_identifiers(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_drive_get_icon(self.to_glib_none().0))
        }
    }

    fn get_identifier(&self, kind: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_drive_get_identifier(self.to_glib_none().0, kind.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_drive_get_name(self.to_glib_none().0))
        }
    }

    fn get_sort_key(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_drive_get_sort_key(self.to_glib_none().0))
        }
    }

    fn get_start_stop_type(&self) -> DriveStartStopType {
        unsafe {
            from_glib(ffi::g_drive_get_start_stop_type(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_drive_get_symbolic_icon(self.to_glib_none().0))
        }
    }

    fn get_volumes(&self) -> Vec<Volume> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_drive_get_volumes(self.to_glib_none().0))
        }
    }

    fn has_media(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_has_media(self.to_glib_none().0))
        }
    }

    fn has_volumes(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_has_volumes(self.to_glib_none().0))
        }
    }

    fn is_media_check_automatic(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_check_automatic(self.to_glib_none().0))
        }
    }

    fn is_media_removable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_media_removable(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn is_removable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_drive_is_removable(self.to_glib_none().0))
        }
    }

    //fn poll_for_media<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::g_drive_poll_for_media() }
    //}

    //fn start<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: DriveStartFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S) {
    //    unsafe { TODO: call ffi::g_drive_start() }
    //}

    //fn stop<'a, 'b, 'c, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: Into<Option<&'c /*Unimplemented*/AsyncReadyCallback>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, flags: /*Ignored*/MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R, user_data: S) {
    //    unsafe { TODO: call ffi::g_drive_stop() }
    //}

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_disconnected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "disconnected",
                transmute(disconnected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_eject_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "eject-button",
                transmute(eject_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_stop_button<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stop-button",
                transmute(stop_button_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GDrive, f: glib_ffi::gpointer)
where P: IsA<Drive> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Drive::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn disconnected_trampoline<P>(this: *mut ffi::GDrive, f: glib_ffi::gpointer)
where P: IsA<Drive> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Drive::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn eject_button_trampoline<P>(this: *mut ffi::GDrive, f: glib_ffi::gpointer)
where P: IsA<Drive> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Drive::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn stop_button_trampoline<P>(this: *mut ffi::GDrive, f: glib_ffi::gpointer)
where P: IsA<Drive> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Drive::from_glib_borrow(this).downcast_unchecked())
}
