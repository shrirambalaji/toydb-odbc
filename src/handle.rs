use toydb::Client;

pub enum Handle {
    Env,
    Conn {
        client: Option<Client>,
    },
    Stmt {
        conn: usize,
        rows: Vec<Vec<String>>,
        cursor: usize,
    },
}

/// Represented in C headers as SQLSMALLINT
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i16)]
pub enum HandleType {
    SQL_HANDLE_ENV = 1,
    SQL_HANDLE_DBC = 2,
    SQL_HANDLE_STMT = 3,
    SQL_HANDLE_DESC = 4,
}

impl HandleType {
    pub fn from_raw(value: i16) -> Option<Self> {
        match value {
            1 => Some(HandleType::SQL_HANDLE_ENV),
            2 => Some(HandleType::SQL_HANDLE_DBC),
            3 => Some(HandleType::SQL_HANDLE_STMT),
            _ => None,
        }
    }
}
