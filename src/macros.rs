#[macro_export]
macro_rules! skip_fail {
    ($res:expr, $error_msg:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                println!("Error: {}", $error_msg);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! skip_none {
    ($res:expr, $error_msg:expr) => {
        match $res {
            Some(val) => val,
            None => {
                println!("Error: {}", $error_msg);
                continue;
            }
        }
    };
}
