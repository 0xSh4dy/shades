use std::{cell::RefCell, sync::RwLock};

use lazy_static::lazy_static;

const N_SYMBOLS:usize = 1024;

#[derive(Clone)]
pub struct SymTab{
    name:String
}

lazy_static!{
    static ref GLOB_SYMTAB:RwLock<Vec<SymTab>> = {
        let symtab = SymTab{name:String::from("")}; 
        RwLock::new(vec![symtab;N_SYMBOLS])
    };
}

pub fn find_symbol(name:&str)->Option<usize>{
    let guard = GLOB_SYMTAB.read().unwrap();
    guard.iter().position(|x|x.name == name)
}

pub fn get_corresp_symtab(slot:usize)->SymTab{
    let read_guard = GLOB_SYMTAB.read().unwrap();
    let symt = read_guard.get(slot).expect("Couldn't find the desired slot");
    return symt.clone();
}

pub fn add_symbol(name:String)->Option<usize>{
    let val = {
        let read_guard = GLOB_SYMTAB.read().unwrap();
        let idx = read_guard.iter().position(|x|x.name=="");
        idx
    };
    if let Some(idx) = val{
        let mut write_guard = GLOB_SYMTAB.write().unwrap();
        write_guard[idx].name = name;
        return Some(idx);
    }
    None
}

impl SymTab{
    pub fn get_name(&self)->String{
        return self.name.clone();
    }
}