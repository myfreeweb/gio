// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BufferedInputStream;
use Cancellable;
use DataStreamByteOrder;
use DataStreamNewlineType;
use Error;
use FilterInputStream;
use InputStream;
use Seekable;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DataInputStream(Object<ffi::GDataInputStream, ffi::GDataInputStreamClass, DataInputStreamClass>) @extends BufferedInputStream, FilterInputStream, InputStream, @implements Seekable;

    match fn {
        get_type => || ffi::g_data_input_stream_get_type(),
    }
}

impl DataInputStream {
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> DataInputStream {
        unsafe {
            from_glib_full(ffi::g_data_input_stream_new(base_stream.as_ref().to_glib_none().0))
        }
    }
}

pub const NONE_DATA_INPUT_STREAM: Option<&DataInputStream> = None;

pub trait DataInputStreamExt: 'static {
    fn get_byte_order(&self) -> DataStreamByteOrder;

    fn get_newline_type(&self) -> DataStreamNewlineType;

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u8, Error>;

    fn read_int16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i16, Error>;

    fn read_int32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, Error>;

    fn read_int64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i64, Error>;

    //fn read_line_finish_utf8(&self, result: /*Ignored*/&AsyncResult) -> Result<(Option<GString>, usize), Error>;

    fn read_line_utf8<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(Option<GString>, usize), Error>;

    fn read_uint16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u16, Error>;

    fn read_uint32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u32, Error>;

    fn read_uint64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u64, Error>;

    #[cfg_attr(feature = "v2_56", deprecated)]
    fn read_until<P: IsA<Cancellable>>(&self, stop_chars: &str, cancellable: Option<&P>) -> Result<(GString, usize), Error>;

    #[cfg_attr(feature = "v2_56", deprecated)]
    fn read_until_async<P: IsA<Cancellable>, Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg_attr(feature = "v2_56", deprecated)]
    #[cfg(feature = "futures")]
    fn read_until_async_future(&self, stop_chars: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, (GString, usize)), Error = (Self, Error)>> where Self: Sized + Clone;

    fn read_upto<P: IsA<Cancellable>>(&self, stop_chars: &str, cancellable: Option<&P>) -> Result<(GString, usize), Error>;

    fn read_upto_async<P: IsA<Cancellable>, Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn read_upto_async_future(&self, stop_chars: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, (GString, usize)), Error = (Self, Error)>> where Self: Sized + Clone;

    fn set_byte_order(&self, order: DataStreamByteOrder);

    fn set_newline_type(&self, type_: DataStreamNewlineType);

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataInputStream>> DataInputStreamExt for O {
    fn get_byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_byte_order(self.as_ref().to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> DataStreamNewlineType {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_newline_type(self.as_ref().to_glib_none().0))
        }
    }

    fn read_byte<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u8, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_byte(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i16, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int16(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int32(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<i64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int64(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    //fn read_line_finish_utf8(&self, result: /*Ignored*/&AsyncResult) -> Result<(Option<GString>, usize), Error> {
    //    unsafe { TODO: call ffi::g_data_input_stream_read_line_finish_utf8() }
    //}

    fn read_line_utf8<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(Option<GString>, usize), Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_line_utf8(self.as_ref().to_glib_none().0, &mut length, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint16<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u16, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint16(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint32<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint32(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint64<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<u64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint64(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_until<P: IsA<Cancellable>>(&self, stop_chars: &str, cancellable: Option<&P>) -> Result<(GString, usize), Error> {
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_until(self.as_ref().to_glib_none().0, stop_chars.to_glib_none().0, &mut length, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_until_async<P: IsA<Cancellable>, Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_until_async_trampoline<Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let mut length = mem::uninitialized();
            let ret = ffi::g_data_input_stream_read_until_finish(_source_object as *mut _, res, &mut length, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_until_async_trampoline::<Q>;
        unsafe {
            ffi::g_data_input_stream_read_until_async(self.as_ref().to_glib_none().0, stop_chars.to_glib_none().0, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn read_until_async_future(&self, stop_chars: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, (GString, usize)), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let stop_chars = String::from(stop_chars);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.read_until_async(
                &stop_chars,
                io_priority,
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

    fn read_upto<P: IsA<Cancellable>>(&self, stop_chars: &str, cancellable: Option<&P>) -> Result<(GString, usize), Error> {
        let stop_chars_len = stop_chars.len() as isize;
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_upto(self.as_ref().to_glib_none().0, stop_chars.to_glib_none().0, stop_chars_len, &mut length, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_upto_async<P: IsA<Cancellable>, Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
        let stop_chars_len = stop_chars.len() as isize;
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_upto_async_trampoline<Q: FnOnce(Result<(GString, usize), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer) {
            let mut error = ptr::null_mut();
            let mut length = mem::uninitialized();
            let ret = ffi::g_data_input_stream_read_upto_finish(_source_object as *mut _, res, &mut length, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_upto_async_trampoline::<Q>;
        unsafe {
            ffi::g_data_input_stream_read_upto_async(self.as_ref().to_glib_none().0, stop_chars.to_glib_none().0, stop_chars_len, io_priority.to_glib(), cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn read_upto_async_future(&self, stop_chars: &str, io_priority: glib::Priority) -> Box_<futures_core::Future<Item = (Self, (GString, usize)), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let stop_chars = String::from(stop_chars);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.read_upto_async(
                &stop_chars,
                io_priority,
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

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_input_stream_set_byte_order(self.as_ref().to_glib_none().0, order.to_glib());
        }
    }

    fn set_newline_type(&self, type_: DataStreamNewlineType) {
        unsafe {
            ffi::g_data_input_stream_set_newline_type(self.as_ref().to_glib_none().0, type_.to_glib());
        }
    }

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::byte-order\0".as_ptr() as *const _,
                Some(transmute(notify_byte_order_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute(notify_newline_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_byte_order_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GDataInputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DataInputStream> {
    let f: &F = &*(f as *const F);
    f(&DataInputStream::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GDataInputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DataInputStream> {
    let f: &F = &*(f as *const F);
    f(&DataInputStream::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DataInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataInputStream")
    }
}
