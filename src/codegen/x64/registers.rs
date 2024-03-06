use crate::{codegen::regops::RegOperations, utils::errors::fatal_error};

const GP_REG_COUNT: usize = 16;
pub struct X64Registers<'a> {
    gpr_list: [&'a str; GP_REG_COUNT], // General purpose registers
    available_regs: [bool; GP_REG_COUNT],
}

#[derive(PartialEq)]
enum X64Regs {
    RBP,
    RSP,
    RAX,
    RBX,
}

#[allow(dead_code)]
impl<'a> X64Registers<'a> {
    pub fn new() -> Self {
        let gpr_list = [
            "rbp", "rsp", "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11",
            "r12", "r13", "r14", "r15",
        ];
        let available_regs = [true; GP_REG_COUNT];
        X64Registers {
            gpr_list: gpr_list,
            available_regs,
        }
    }
    pub fn alloc_register(&mut self) -> i32 {
        // Skipping 0 and 1 because we're not gonna use rbp rsp and rax over here
        for i in 3..GP_REG_COUNT {
            if self.available_regs[i] {
                self.available_regs[i] = false;
                return i as i32;
            }
        }
        return -1;
    }
    pub fn free_register(&mut self, ctr: usize) -> bool {
        if self.available_regs[ctr] {
            // If the register is already free
            return false;
        }
        self.available_regs[ctr] = true;
        return true;
    }
    pub fn free_all_registers(&mut self) {
        for i in 2..GP_REG_COUNT {
            self.available_regs[i] = true;
        }
    }

    pub fn func_preamble(&self) {
        println!("push   rbp;");
        println!("mov    rbp,rsp;");
    }

    pub fn func_postamble(&self) {
        println!("xor   rax,rax;");
        println!("pop   rbp;");
        println!("ret;");
    }
    pub fn alloc_specific_register(&mut self, val: usize) {
        self.available_regs[val] = false;
    }
}

impl<'a> RegOperations for X64Registers<'a> {
    fn ro_add(&mut self, reg1: usize, reg2: usize) -> i32 {
        println!("add   {}, {};", self.gpr_list[reg1], self.gpr_list[reg2]);
        self.free_register(reg2);
        return reg1 as i32;
    }
    fn ro_sub(&mut self, reg1: usize, reg2: usize) -> i32 {
        println!("sub   {}, {};", self.gpr_list[reg1], self.gpr_list[reg2]);
        self.free_register(reg2);
        return reg1 as i32;
    }
    fn ro_mul(&mut self, reg1: usize, reg2: usize) -> i32 {
        // Commutative, load any value into rax and multiply with the other.
        println!("mov   rax,{};", self.gpr_list[reg1]);
        println!("mul   {};", self.gpr_list[reg2]);
        println!("mov   {},rax;", self.gpr_list[reg1]);
        self.free_register(reg2);
        return reg1 as i32;
    }
    fn ro_div(&mut self, reg1: usize, reg2: usize) -> i32 {
        // Load the dividend in rax. Quotient goes to rax and remainder goes to rdx
        println!("mov   rax,{};", self.gpr_list[reg1]);
        println!("div   {};", self.gpr_list[reg2]);
        println!("mov   {},rax;", self.gpr_list[reg1]);
        self.free_register(reg2);
        return reg1 as i32;
    }
    fn ro_load(&mut self, val: i64) -> i32 {
        let reg = self.alloc_register();
        if reg == -1 {
            fatal_error("Failed to load register", 1);
        }
        println!("mov   {}, 0x{:X};", self.gpr_list[reg as usize], val);
        return reg;
    }
}
