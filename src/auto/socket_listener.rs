// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use Socket;
use SocketAddress;
use SocketConnection;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use SocketListenerEvent;
use SocketProtocol;
use SocketType;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
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
    pub struct SocketListener(Object<ffi::GSocketListener, ffi::GSocketListenerClass>);

    match fn {
        get_type => || ffi::g_socket_listener_get_type(),
    }
}

impl SocketListener {
    pub fn new() -> SocketListener {
        unsafe {
            from_glib_full(ffi::g_socket_listener_new())
        }
    }
}

impl Default for SocketListener {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SocketListenerExt: Sized {
    fn accept<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(SocketConnection, Option<glib::Object>), Error>;

    fn accept_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(SocketConnection, glib::Object), Error>) + Send + 'static>(&self, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn accept_async_future(&self) -> Box_<futures_core::Future<Item = (Self, (SocketConnection, glib::Object)), Error = (Self, Error)>>;

    fn accept_socket<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(Socket, Option<glib::Object>), Error>;

    fn add_address<'a, P: IsA<SocketAddress>, Q: IsA<glib::Object> + 'a, R: Into<Option<&'a Q>>>(&self, address: &P, type_: SocketType, protocol: SocketProtocol, source_object: R) -> Result<SocketAddress, Error>;

    fn add_any_inet_port<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, source_object: Q) -> Result<u16, Error>;

    fn add_inet_port<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, port: u16, source_object: Q) -> Result<(), Error>;

    fn add_socket<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, socket: &Socket, source_object: Q) -> Result<(), Error>;

    fn close(&self);

    fn set_backlog(&self, listen_backlog: i32);

    fn get_property_listen_backlog(&self) -> i32;

    fn set_property_listen_backlog(&self, listen_backlog: i32);

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketListener> + IsA<glib::object::Object> + Clone + 'static> SocketListenerExt for O {
    fn accept<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(SocketConnection, Option<glib::Object>), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept(self.to_glib_none().0, &mut source_object, cancellable.0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib_none(source_object))) } else { Err(from_glib_full(error)) }
        }
    }

    fn accept_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(SocketConnection, glib::Object), Error>) + Send + 'static>(&self, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn accept_async_trampoline<Q: FnOnce(Result<(SocketConnection, glib::Object), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut source_object = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept_finish(_source_object as *mut _, res, &mut source_object, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), from_glib_none(source_object))) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = accept_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_listener_accept_async(self.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn accept_async_future(&self) -> Box_<futures_core::Future<Item = (Self, (SocketConnection, glib::Object)), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.accept_async(
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn accept_socket<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(Socket, Option<glib::Object>), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut source_object = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_accept_socket(self.to_glib_none().0, &mut source_object, cancellable.0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib_none(source_object))) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_address<'a, P: IsA<SocketAddress>, Q: IsA<glib::Object> + 'a, R: Into<Option<&'a Q>>>(&self, address: &P, type_: SocketType, protocol: SocketProtocol, source_object: R) -> Result<SocketAddress, Error> {
        let source_object = source_object.into();
        let source_object = source_object.to_glib_none();
        unsafe {
            let mut effective_address = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listener_add_address(self.to_glib_none().0, address.to_glib_none().0, type_.to_glib(), protocol.to_glib(), source_object.0, &mut effective_address, &mut error);
            if error.is_null() { Ok(from_glib_full(effective_address)) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_any_inet_port<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, source_object: Q) -> Result<u16, Error> {
        let source_object = source_object.into();
        let source_object = source_object.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_listener_add_any_inet_port(self.to_glib_none().0, source_object.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_inet_port<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, port: u16, source_object: Q) -> Result<(), Error> {
        let source_object = source_object.into();
        let source_object = source_object.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listener_add_inet_port(self.to_glib_none().0, port, source_object.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn add_socket<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(&self, socket: &Socket, source_object: Q) -> Result<(), Error> {
        let source_object = source_object.into();
        let source_object = source_object.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_socket_listener_add_socket(self.to_glib_none().0, socket.to_glib_none().0, source_object.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn close(&self) {
        unsafe {
            ffi::g_socket_listener_close(self.to_glib_none().0);
        }
    }

    fn set_backlog(&self, listen_backlog: i32) {
        unsafe {
            ffi::g_socket_listener_set_backlog(self.to_glib_none().0, listen_backlog);
        }
    }

    fn get_property_listen_backlog(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "listen-backlog".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_listen_backlog(&self, listen_backlog: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "listen-backlog".to_glib_none().0, Value::from(&listen_backlog).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn connect_event<F: Fn(&Self, SocketListenerEvent, &Socket) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, SocketListenerEvent, &Socket) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "event",
                transmute(event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_listen_backlog_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::listen-backlog",
                transmute(notify_listen_backlog_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_46", feature = "dox"))]
unsafe extern "C" fn event_trampoline<P>(this: *mut ffi::GSocketListener, event: ffi::GSocketListenerEvent, socket: *mut ffi::GSocket, f: glib_ffi::gpointer)
where P: IsA<SocketListener> {
    callback_guard!();
    let f: &&(Fn(&P, SocketListenerEvent, &Socket) + 'static) = transmute(f);
    f(&SocketListener::from_glib_borrow(this).downcast_unchecked(), from_glib(event), &from_glib_borrow(socket))
}

unsafe extern "C" fn notify_listen_backlog_trampoline<P>(this: *mut ffi::GSocketListener, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketListener> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SocketListener::from_glib_borrow(this).downcast_unchecked())
}
