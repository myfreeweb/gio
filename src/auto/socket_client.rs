// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use IOStream;
use ProxyResolver;
use SocketAddress;
use SocketClientEvent;
use SocketConnectable;
use SocketConnection;
use SocketFamily;
use SocketProtocol;
use SocketType;
use TlsCertificateFlags;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
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
    pub struct SocketClient(Object<ffi::GSocketClient, ffi::GSocketClientClass, SocketClientClass>);

    match fn {
        get_type => || ffi::g_socket_client_get_type(),
    }
}

impl SocketClient {
    pub fn new() -> SocketClient {
        unsafe {
            from_glib_full(ffi::g_socket_client_new())
        }
    }
}

impl Default for SocketClient {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SOCKET_CLIENT: Option<&SocketClient> = None;

pub trait SocketClientExt: 'static {
    fn add_application_proxy(&self, protocol: &str);

    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(&self, connectable: &P, cancellable: Option<&Q>) -> Result<SocketConnection, Error>;

    fn connect_async<P: IsA<SocketConnectable>, Q: IsA<Cancellable>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(&self, connectable: &P) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone;

    fn connect_to_host<P: IsA<Cancellable>>(&self, host_and_port: &str, default_port: u16, cancellable: Option<&P>) -> Result<SocketConnection, Error>;

    fn connect_to_host_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn connect_to_host_async_future(&self, host_and_port: &str, default_port: u16) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone;

    fn connect_to_service<P: IsA<Cancellable>>(&self, domain: &str, service: &str, cancellable: Option<&P>) -> Result<SocketConnection, Error>;

    fn connect_to_service_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn connect_to_service_async_future(&self, domain: &str, service: &str) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone;

    fn connect_to_uri<P: IsA<Cancellable>>(&self, uri: &str, default_port: u16, cancellable: Option<&P>) -> Result<SocketConnection, Error>;

    fn connect_to_uri_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn connect_to_uri_async_future(&self, uri: &str, default_port: u16) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone;

    fn get_enable_proxy(&self) -> bool;

    fn get_family(&self) -> SocketFamily;

    fn get_local_address(&self) -> Option<SocketAddress>;

    fn get_protocol(&self) -> SocketProtocol;

    fn get_proxy_resolver(&self) -> Option<ProxyResolver>;

    fn get_socket_type(&self) -> SocketType;

    fn get_timeout(&self) -> u32;

    fn get_tls(&self) -> bool;

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags;

    fn set_enable_proxy(&self, enable: bool);

    fn set_family(&self, family: SocketFamily);

    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>);

    fn set_protocol(&self, protocol: SocketProtocol);

    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>);

    fn set_socket_type(&self, type_: SocketType);

    fn set_timeout(&self, timeout: u32);

    fn set_tls(&self, tls: bool);

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags);

    fn get_property_type(&self) -> SocketType;

    fn set_property_type(&self, type_: SocketType);

    fn connect_event<F: Fn(&Self, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SocketClient>> SocketClientExt for O {
    fn add_application_proxy(&self, protocol: &str) {
        unsafe {
            ffi::g_socket_client_add_application_proxy(self.as_ref().to_glib_none().0, protocol.to_glib_none().0);
        }
    }

    fn connect<P: IsA<SocketConnectable>, Q: IsA<Cancellable>>(&self, connectable: &P, cancellable: Option<&Q>) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect(self.as_ref().to_glib_none().0, connectable.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_async<P: IsA<SocketConnectable>, Q: IsA<Cancellable>, R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, connectable: &P, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn connect_async_trampoline<R: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<R>;
        unsafe {
            ffi::g_socket_client_connect_async(self.as_ref().to_glib_none().0, connectable.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_async_future<P: IsA<SocketConnectable> + Clone + 'static>(&self, connectable: &P) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let connectable = connectable.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_async(
                &connectable,
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

    fn connect_to_host<P: IsA<Cancellable>>(&self, host_and_port: &str, default_port: u16, cancellable: Option<&P>) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host(self.as_ref().to_glib_none().0, host_and_port.to_glib_none().0, default_port, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_host_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, host_and_port: &str, default_port: u16, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_host_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_host_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_host_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_host_async(self.as_ref().to_glib_none().0, host_and_port.to_glib_none().0, default_port, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_host_async_future(&self, host_and_port: &str, default_port: u16) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let host_and_port = String::from(host_and_port);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_to_host_async(
                &host_and_port,
                default_port,
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

    fn connect_to_service<P: IsA<Cancellable>>(&self, domain: &str, service: &str, cancellable: Option<&P>) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service(self.as_ref().to_glib_none().0, domain.to_glib_none().0, service.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_service_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, domain: &str, service: &str, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_service_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_service_async(self.as_ref().to_glib_none().0, domain.to_glib_none().0, service.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_service_async_future(&self, domain: &str, service: &str) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let domain = String::from(domain);
        let service = String::from(service);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_to_service_async(
                &domain,
                &service,
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

    fn connect_to_uri<P: IsA<Cancellable>>(&self, uri: &str, default_port: u16, cancellable: Option<&P>) -> Result<SocketConnection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0, default_port, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_to_uri_async<P: IsA<Cancellable>, Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(&self, uri: &str, default_port: u16, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn connect_to_uri_async_trampoline<Q: FnOnce(Result<SocketConnection, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_client_connect_to_uri_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_to_uri_async_trampoline::<Q>;
        unsafe {
            ffi::g_socket_client_connect_to_uri_async(self.as_ref().to_glib_none().0, uri.to_glib_none().0, default_port, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn connect_to_uri_async_future(&self, uri: &str, default_port: u16) -> Box_<futures_core::Future<Item = (Self, SocketConnection), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let uri = String::from(uri);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.connect_to_uri_async(
                &uri,
                default_port,
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

    fn get_enable_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_enable_proxy(self.as_ref().to_glib_none().0))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_socket_client_get_family(self.as_ref().to_glib_none().0))
        }
    }

    fn get_local_address(&self) -> Option<SocketAddress> {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_local_address(self.as_ref().to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> SocketProtocol {
        unsafe {
            from_glib(ffi::g_socket_client_get_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_proxy_resolver(&self) -> Option<ProxyResolver> {
        unsafe {
            from_glib_none(ffi::g_socket_client_get_proxy_resolver(self.as_ref().to_glib_none().0))
        }
    }

    fn get_socket_type(&self) -> SocketType {
        unsafe {
            from_glib(ffi::g_socket_client_get_socket_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_timeout(&self) -> u32 {
        unsafe {
            ffi::g_socket_client_get_timeout(self.as_ref().to_glib_none().0)
        }
    }

    fn get_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls(self.as_ref().to_glib_none().0))
        }
    }

    fn get_tls_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_socket_client_get_tls_validation_flags(self.as_ref().to_glib_none().0))
        }
    }

    fn set_enable_proxy(&self, enable: bool) {
        unsafe {
            ffi::g_socket_client_set_enable_proxy(self.as_ref().to_glib_none().0, enable.to_glib());
        }
    }

    fn set_family(&self, family: SocketFamily) {
        unsafe {
            ffi::g_socket_client_set_family(self.as_ref().to_glib_none().0, family.to_glib());
        }
    }

    fn set_local_address<P: IsA<SocketAddress>>(&self, address: Option<&P>) {
        unsafe {
            ffi::g_socket_client_set_local_address(self.as_ref().to_glib_none().0, address.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_protocol(&self, protocol: SocketProtocol) {
        unsafe {
            ffi::g_socket_client_set_protocol(self.as_ref().to_glib_none().0, protocol.to_glib());
        }
    }

    fn set_proxy_resolver<P: IsA<ProxyResolver>>(&self, proxy_resolver: Option<&P>) {
        unsafe {
            ffi::g_socket_client_set_proxy_resolver(self.as_ref().to_glib_none().0, proxy_resolver.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_socket_type(&self, type_: SocketType) {
        unsafe {
            ffi::g_socket_client_set_socket_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::g_socket_client_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn set_tls(&self, tls: bool) {
        unsafe {
            ffi::g_socket_client_set_tls(self.as_ref().to_glib_none().0, tls.to_glib());
        }
    }

    fn set_tls_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_socket_client_set_tls_validation_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        }
    }

    fn get_property_type(&self) -> SocketType {
        unsafe {
            let mut value = Value::from_type(<SocketType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_type(&self, type_: SocketType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, Value::from(&type_).to_glib_none().0);
        }
    }

    fn connect_event<F: Fn(&Self, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"event\0".as_ptr() as *const _,
                Some(transmute(event_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_enable_proxy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enable-proxy\0".as_ptr() as *const _,
                Some(transmute(notify_enable_proxy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::family\0".as_ptr() as *const _,
                Some(transmute(notify_family_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_local_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::local-address\0".as_ptr() as *const _,
                Some(transmute(notify_local_address_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::protocol\0".as_ptr() as *const _,
                Some(transmute(notify_protocol_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_proxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::proxy-resolver\0".as_ptr() as *const _,
                Some(transmute(notify_proxy_resolver_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute(notify_timeout_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tls_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tls\0".as_ptr() as *const _,
                Some(transmute(notify_tls_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tls_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tls-validation-flags\0".as_ptr() as *const _,
                Some(transmute(notify_tls_validation_flags_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn event_trampoline<P, F: Fn(&P, SocketClientEvent, &SocketConnectable, &Option<IOStream>) + 'static>(this: *mut ffi::GSocketClient, event: ffi::GSocketClientEvent, connectable: *mut ffi::GSocketConnectable, connection: *mut ffi::GIOStream, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast(), from_glib(event), &from_glib_borrow(connectable), &from_glib_borrow(connection))
}

unsafe extern "C" fn notify_enable_proxy_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_family_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_local_address_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_protocol_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_proxy_resolver_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tls_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tls_validation_flags_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GSocketClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SocketClient> {
    let f: &F = &*(f as *const F);
    f(&SocketClient::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SocketClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SocketClient")
    }
}
