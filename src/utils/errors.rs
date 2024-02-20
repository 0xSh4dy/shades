use std::process::exit;

pub fn throw_custom_error(error_message:&str)->Box<std::io::Error>{
    return Box::new(std::io::Error::new(std::io::ErrorKind::Other,error_message));
}

pub fn fatal_error(error_message:&str,code:i32){
    println!("{}",error_message);
    exit(code);
}