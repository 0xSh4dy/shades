use utils::file;
use codegenerator::generator::generate_code;
use lexer::scanner::start_scanner;
mod lexer;
mod utils;
mod ast;
mod codegenerator;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: hicraft <path_to_file>")
    } else {
        let res = file::open_file(&args[1]);
        match res {
            Ok(program) => {
                let res = file::read_file_string(program);
                match res {
                    Ok(file_data) => {
                        let res = start_scanner(file_data);
                        match res{
                            Ok(mut token_list)=>{
                                generate_code(&mut token_list);
                            },
                            Err(err)=>{println!("{}",err)}
                        };
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
