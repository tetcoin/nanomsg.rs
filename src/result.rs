extern crate libc;

use libc::{c_int};
use tetsy_nanomsg_sys;

use std::str;
use std::fmt;
use std::io;
use std::convert::From;
use std::ffi::CStr;
use std::result;
use std::error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    Unknown                    = 0 as isize,
    OperationNotSupported      = tetsy_nanomsg_sys::ENOTSUP          as isize,
    ProtocolNotSupported       = tetsy_nanomsg_sys::EPROTONOSUPPORT  as isize,
    NoBufferSpace              = tetsy_nanomsg_sys::ENOBUFS          as isize,
    NetworkDown                = tetsy_nanomsg_sys::ENETDOWN         as isize,
    AddressInUse               = tetsy_nanomsg_sys::EADDRINUSE       as isize,
    AddressNotAvailable        = tetsy_nanomsg_sys::EADDRNOTAVAIL    as isize,
    ConnectionRefused          = tetsy_nanomsg_sys::ECONNREFUSED     as isize,
    OperationNowInProgress     = tetsy_nanomsg_sys::EINPROGRESS      as isize,
    NotSocket                  = tetsy_nanomsg_sys::ENOTSOCK         as isize,
    AddressFamilyNotSupported  = tetsy_nanomsg_sys::EAFNOSUPPORT     as isize,
    WrongProtocol              = tetsy_nanomsg_sys::EPROTO           as isize,
    TryAgain                   = tetsy_nanomsg_sys::EAGAIN           as isize,
    BadFileDescriptor          = tetsy_nanomsg_sys::EBADF            as isize,
    InvalidInput               = tetsy_nanomsg_sys::EINVAL           as isize,
    TooManyOpenFiles           = tetsy_nanomsg_sys::EMFILE           as isize,
    BadAddress                 = tetsy_nanomsg_sys::EFAULT           as isize,
    PermissionDenied           = tetsy_nanomsg_sys::EACCESS          as isize,
    NetworkReset               = tetsy_nanomsg_sys::ENETRESET        as isize,
    NetworkUnreachable         = tetsy_nanomsg_sys::ENETUNREACH      as isize,
    HostUnreachable            = tetsy_nanomsg_sys::EHOSTUNREACH     as isize,
    NotConnected               = tetsy_nanomsg_sys::ENOTCONN         as isize,
    MessageTooLong             = tetsy_nanomsg_sys::EMSGSIZE         as isize,
    TimedOut                   = tetsy_nanomsg_sys::ETIMEDOUT        as isize,
    ConnectionAborted          = tetsy_nanomsg_sys::ECONNABORTED     as isize,
    ConnectionReset            = tetsy_nanomsg_sys::ECONNRESET       as isize,
    ProtocolNotAvailable       = tetsy_nanomsg_sys::ENOPROTOOPT      as isize,
    AlreadyConnected           = tetsy_nanomsg_sys::EISCONN          as isize,
    SocketTypeNotSupported     = tetsy_nanomsg_sys::ESOCKTNOSUPPORT  as isize,
    Terminating                = tetsy_nanomsg_sys::ETERM            as isize,
    NameTooLong                = tetsy_nanomsg_sys::ENAMETOOLONG     as isize,
    NoDevice                   = tetsy_nanomsg_sys::ENODEV           as isize,
    FileStateMismatch          = tetsy_nanomsg_sys::EFSM             as isize,
    Interrupted                = tetsy_nanomsg_sys::EINTR            as isize
}

impl Error {
    pub fn to_raw(&self) -> c_int {
        *self as c_int
    }

    pub fn from_raw(raw: c_int) -> Error {
        match raw {
            tetsy_nanomsg_sys::ENOTSUP         => Error::OperationNotSupported    ,
            tetsy_nanomsg_sys::EPROTONOSUPPORT => Error::ProtocolNotSupported     ,
            tetsy_nanomsg_sys::ENOBUFS         => Error::NoBufferSpace            ,
            tetsy_nanomsg_sys::ENETDOWN        => Error::NetworkDown              ,
            tetsy_nanomsg_sys::EADDRINUSE      => Error::AddressInUse             ,
            tetsy_nanomsg_sys::EADDRNOTAVAIL   => Error::AddressNotAvailable      ,
            tetsy_nanomsg_sys::ECONNREFUSED    => Error::ConnectionRefused        ,
            tetsy_nanomsg_sys::EINPROGRESS     => Error::OperationNowInProgress   ,
            tetsy_nanomsg_sys::ENOTSOCK        => Error::NotSocket                ,
            tetsy_nanomsg_sys::EAFNOSUPPORT    => Error::AddressFamilyNotSupported,
            tetsy_nanomsg_sys::EPROTO          => Error::WrongProtocol            ,
            tetsy_nanomsg_sys::EAGAIN          => Error::TryAgain                 ,
            tetsy_nanomsg_sys::EBADF           => Error::BadFileDescriptor        ,
            tetsy_nanomsg_sys::EINVAL          => Error::InvalidInput             ,
            tetsy_nanomsg_sys::EMFILE          => Error::TooManyOpenFiles         ,
            tetsy_nanomsg_sys::EFAULT          => Error::BadAddress               ,
            tetsy_nanomsg_sys::EACCESS         => Error::PermissionDenied         ,
            tetsy_nanomsg_sys::ENETRESET       => Error::NetworkReset             ,
            tetsy_nanomsg_sys::ENETUNREACH     => Error::NetworkUnreachable       ,
            tetsy_nanomsg_sys::EHOSTUNREACH    => Error::HostUnreachable          ,
            tetsy_nanomsg_sys::ENOTCONN        => Error::NotConnected             ,
            tetsy_nanomsg_sys::EMSGSIZE        => Error::MessageTooLong           ,
            tetsy_nanomsg_sys::ETIMEDOUT       => Error::TimedOut                 ,
            tetsy_nanomsg_sys::ECONNABORTED    => Error::ConnectionAborted        ,
            tetsy_nanomsg_sys::ECONNRESET      => Error::ConnectionReset          ,
            tetsy_nanomsg_sys::ENOPROTOOPT     => Error::ProtocolNotAvailable     ,
            tetsy_nanomsg_sys::EISCONN         => Error::AlreadyConnected         ,
            tetsy_nanomsg_sys::ESOCKTNOSUPPORT => Error::SocketTypeNotSupported   ,
            tetsy_nanomsg_sys::ETERM           => Error::Terminating              ,
            tetsy_nanomsg_sys::ENAMETOOLONG    => Error::NameTooLong              ,
            tetsy_nanomsg_sys::ENODEV          => Error::NoDevice                 ,
            tetsy_nanomsg_sys::EFSM            => Error::FileStateMismatch        ,
            tetsy_nanomsg_sys::EINTR           => Error::Interrupted              ,
            _                            => Error::Unknown
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        unsafe {
            let nn_errno = *self as c_int;
            let c_ptr: *const libc::c_char = tetsy_nanomsg_sys::nn_strerror(nn_errno);
            let c_str = CStr::from_ptr(c_ptr);
            let bytes = c_str.to_bytes();
            let desc = str::from_utf8(bytes).unwrap_or("Error");

            desc
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        match err.kind() {
            io::ErrorKind::PermissionDenied    => Error::PermissionDenied,
            io::ErrorKind::ConnectionRefused   => Error::ConnectionRefused,
            io::ErrorKind::ConnectionReset     => Error::ConnectionReset,
            io::ErrorKind::ConnectionAborted   => Error::ConnectionAborted,
            io::ErrorKind::NotConnected        => Error::NotConnected,
            io::ErrorKind::AddrInUse           => Error::AddressInUse,
            io::ErrorKind::AddrNotAvailable    => Error::AddressNotAvailable,
            io::ErrorKind::AlreadyExists       => Error::AlreadyConnected,
            io::ErrorKind::WouldBlock          => Error::TryAgain,
            io::ErrorKind::InvalidInput        => Error::InvalidInput,
            io::ErrorKind::TimedOut            => Error::TimedOut,
            io::ErrorKind::Interrupted         => Error::Interrupted,
            _                                  => Error::Unknown
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        let as_std_error: &error::Error = &err;
        let description = as_std_error.description();
        match err {
            Error::PermissionDenied      => io::Error::new(io::ErrorKind::PermissionDenied,  description ),
            Error::ConnectionRefused     => io::Error::new(io::ErrorKind::ConnectionRefused, description ),
            Error::ConnectionReset       => io::Error::new(io::ErrorKind::ConnectionReset,   description ),
            Error::ConnectionAborted     => io::Error::new(io::ErrorKind::ConnectionAborted, description ),
            Error::NotConnected          => io::Error::new(io::ErrorKind::NotConnected,      description ),
            Error::AddressInUse          => io::Error::new(io::ErrorKind::AddrInUse,         description ),
            Error::AddressNotAvailable   => io::Error::new(io::ErrorKind::AddrNotAvailable,  description ),
            Error::AlreadyConnected      => io::Error::new(io::ErrorKind::AlreadyExists,     description ),
            Error::TryAgain              => io::Error::new(io::ErrorKind::WouldBlock,        description ),
            Error::InvalidInput          => io::Error::new(io::ErrorKind::InvalidInput,      description ),
            Error::TimedOut              => io::Error::new(io::ErrorKind::TimedOut,          description ),
            Error::Interrupted           => io::Error::new(io::ErrorKind::Interrupted,       description ),
            _                            => io::Error::new(io::ErrorKind::Other,             description )
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let as_std_error: &error::Error = self;
        let description = as_std_error.description();
        write!(formatter, "{}", description)
    }
}

pub fn last_nano_error() -> Error {
    let nn_errno = unsafe { tetsy_nanomsg_sys::nn_errno() };

    Error::from_raw(nn_errno)
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use tetsy_nanomsg_sys;
    use libc;
    use super::{Error};
    use std::io;
    use std::convert::From;

    fn assert_convert_error_code_to_error(error_code: libc::c_int, expected_error: Error) {
        let converted_error = Error::from_raw(error_code);
        assert_eq!(expected_error, converted_error)
    }

    #[test]
    fn can_convert_error_code_to_error() {
        assert_convert_error_code_to_error(tetsy_nanomsg_sys::ENOTSUP, Error::OperationNotSupported);
        assert_convert_error_code_to_error(tetsy_nanomsg_sys::EPROTONOSUPPORT, Error::ProtocolNotSupported);
        assert_convert_error_code_to_error(tetsy_nanomsg_sys::EADDRINUSE, Error::AddressInUse);
        assert_convert_error_code_to_error(tetsy_nanomsg_sys::EHOSTUNREACH, Error::HostUnreachable);
    }

    fn check_error_kind_match(nano_err: Error, io_err_kind: io::ErrorKind) {
        let io_err: io::Error = From::from(nano_err);

        assert_eq!(io_err_kind, io_err.kind())
    }

    #[test]
    fn nano_err_can_be_converted_to_io_err() {
        check_error_kind_match(Error::TimedOut, io::ErrorKind::TimedOut);
        check_error_kind_match(Error::PermissionDenied, io::ErrorKind::PermissionDenied);
        check_error_kind_match(Error::ConnectionRefused, io::ErrorKind::ConnectionRefused);
        check_error_kind_match(Error::OperationNotSupported, io::ErrorKind::Other);
        check_error_kind_match(Error::NotConnected, io::ErrorKind::NotConnected);
        check_error_kind_match(Error::Interrupted, io::ErrorKind::Interrupted);
    }

    #[test]
    fn nano_err_can_be_converted_from_io_err() {
        let io_err = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
        let nano_err: Error = From::from(io_err);

        assert_eq!(Error::TimedOut, nano_err)
    }
}
