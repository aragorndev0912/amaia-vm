#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    ///Operation I/O.
    Read = 0x0A,
    Write = 0x0B,
    ///Operation load & store of data.
    Load = 0x14,
    Store = 0x15,
    ///Operation arithmetic.
    Add = 0x1E,
    Sub = 0x1F,
    Div = 0x20,
    Mul = 0x21,
    ///Operation of control transfer.
    Jump = 0x28,
    JumpNeg = 0x29,
    JumpZero = 0x2A,
    Stop = 0x2B
}
