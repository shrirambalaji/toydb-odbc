use std::collections::HashMap;
use std::{ffi::c_short, i16};

use crate::connection;
use crate::handle::{self, Handle, HandleType};
use std::ffi::c_char;
use std::sync::{LazyLock, Mutex};

static HANDLES: LazyLock<Mutex<HashMap<usize, handle::Handle>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[repr(C)]
pub struct SqlReturnCode(pub i16);

impl SqlReturnCode {
    pub const SQL_SUCCESS: i16 = 0;
    pub const SQL_ERROR: i16 = -1;
    pub const SQL_NO_DATA: i16 = 100;
}

#[no_mangle]
pub extern "C" fn SQLConnect(handle: *mut connection::Connection, dsn: *const char) -> i16 {
    // let connection_string = unsafe { CString::from_raw(dsn as *mut c_char) };
    // let connection_string = connection_string.to_str().unwrap();

    let connection = connection::Connection::new();
    // TODO: this is the default port and host to toydb, resolve from dsn
    match connection.connect("localhost", 9601) {
        Ok(_) => {
            unsafe { *handle = connection };
            SqlReturnCode::SQL_SUCCESS
        }
        Err(_) => SqlReturnCode::SQL_ERROR,
    }
}

#[no_mangle]
pub extern "C" fn SQLAllocHandle(
    handle_type: c_short,
    input_handle: usize,
    output_handle: *mut usize,
) -> i16 {
    let mut handles = HANDLES.lock().unwrap();
    let handle_id = handles.len() + 1; // Simple unique ID
    match HandleType::from_raw(handle_type) {
        Some(HandleType::SQL_HANDLE_ENV) => {
            // Allocate environment handle, no parent needed
            handles.insert(handle_id, Handle::Env);
            unsafe {
                *output_handle = handle_id;
            }
            SqlReturnCode::SQL_SUCCESS
        }
        Some(HandleType::SQL_HANDLE_DBC) => {
            if matches!(handles.get(&input_handle), Some(Handle::Env)) {
                handles.insert(handle_id, Handle::Conn { client: None });
                unsafe {
                    *output_handle = handle_id;
                }
                SqlReturnCode::SQL_SUCCESS
            } else {
                SqlReturnCode::SQL_ERROR // Invalid environment handle
            }
        }
        Some(HandleType::SQL_HANDLE_STMT) => {
            // Allocate statement handle, requires valid SQLHDBC
            if matches!(handles.get(&input_handle), Some(Handle::Conn { .. })) {
                handles.insert(
                    handle_id,
                    Handle::Stmt {
                        conn: input_handle,
                        rows: vec![],
                        cursor: 0,
                    },
                );
                unsafe {
                    *output_handle = handle_id;
                }
                SqlReturnCode::SQL_SUCCESS
            } else {
                SqlReturnCode::SQL_ERROR // Invalid connection handle
            }
        }
        _ => SqlReturnCode::SQL_ERROR, // Unsupported handle type
    }
}

#[no_mangle]
pub extern "C" fn SQLExecute(query_handle: *const c_char) -> i16 {
    SqlReturnCode::SQL_SUCCESS
}
