/* automatically generated by rust-bindgen */

pub const WEBSOCAT_AUX_ORIENT: &'static [u8; 14usize] = b"orientedness?\0";
pub const WEBSOCAT_AUX_ORIENT_MSG: &'static [u8; 16usize] = b"MessageOriented\0";
pub const WEBSOCAT_AUX_ORIENT_STR: &'static [u8; 15usize] = b"StreamOriented\0";
pub const WEBSOCAT_AUX_DUPLEX: &'static [u8; 8usize] = b"duplex?\0";
pub const WEBSOCAT_AUX_DUPLEX_HALF: &'static [u8; 5usize] = b"half\0";
pub const WEBSOCAT_AUX_DUPLEX_FULL: &'static [u8; 5usize] = b"full\0";
pub const WEBSOCAT_AUX_CLIENT_ADDR: &'static [u8; 12usize] = b"client_addr\0";
pub const WEBSOCAT_AUX_URI: &'static [u8; 4usize] = b"uri\0";
pub const WEBSOCAT_AUX_CUSTOMPARAM_SET: &'static [u8; 13usize] = b"customparam?\0";
pub const WEBSOCAT_AUX_CUSTOMPARAM_GET: &'static [u8; 12usize] = b"customparam\0";
pub const WEBSOCAT_AUX_SHUTDOWN_READ: &'static [u8; 14usize] = b"shutdown_read\0";
pub const WEBSOCAT_AUX_SHUTDOWN_WRITE: &'static [u8; 15usize] = b"shutdown_write\0";
pub const WEBSOCAT_API_VERSION: u32 = 1;
pub type websocat_api_version = ::std::option::Option<unsafe extern "C" fn() -> u32>;
pub type websocat_create_connection = ::std::option::Option<
    unsafe extern "C" fn(restcmdline: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type websocat_destroy_connection =
    ::std::option::Option<unsafe extern "C" fn(connection: *mut ::std::os::raw::c_void)>;
pub type websocat_read = ::std::option::Option<
    unsafe extern "C" fn(
        connection: *mut ::std::os::raw::c_void,
        buf: *mut ::std::os::raw::c_void,
        buflen: u32,
    ) -> i32,
>;
pub type websocat_write = ::std::option::Option<
    unsafe extern "C" fn(
        connection: *mut ::std::os::raw::c_void,
        buf: *const ::std::os::raw::c_void,
        len: u32,
    ) -> i32,
>;
pub type websocat_create_listener = ::std::option::Option<
    unsafe extern "C" fn(restcmdline: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void,
>;
pub type websocat_destroy_listener =
    ::std::option::Option<unsafe extern "C" fn(listener: *mut ::std::os::raw::c_void)>;
pub type websocat_get_connection_from_listener = ::std::option::Option<
    unsafe extern "C" fn(listener: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
>;
pub type websocat_global_aux = ::std::option::Option<
    unsafe extern "C" fn(
        request: *const ::std::os::raw::c_char,
        param: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
>;
pub type websocat_connection_aux = ::std::option::Option<
    unsafe extern "C" fn(
        connection: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
        param: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
>;
pub type websocat_listener_aux = ::std::option::Option<
    unsafe extern "C" fn(
        listener: *mut ::std::os::raw::c_void,
        request: *const ::std::os::raw::c_char,
        param: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,
>;