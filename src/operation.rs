//!Operaciones aceptadas por la virtual machine

///Operaciones de entrada y salida.
pub const READ:u32 = 0x0A; 
pub const WRITE:u32 = 0x0B; 

///Operaciones de carga y almacenamiento.
pub const LOAD:u32 = 0x14;
pub const STORE:u32 = 0x15;

///Operaciones aritmeticas.
pub const ADD:u32 = 0x1E;
pub const SUB:u32 = 0x1F;
pub const DIV:u32 = 0x20;
pub const MUL:u32 = 0x21;

///Operaciones de transferencia de control.
pub const JUMP:u32 = 0x28;
pub const JUMP_NEG:u32 = 0x29;
pub const JUMP_ZERO:u32 = 0x2A;
pub const STOP:u32 = 0x2B;
