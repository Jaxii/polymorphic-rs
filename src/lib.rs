use std::arch::asm;
use rand::{Rng, thread_rng};

#[inline(always)]
pub unsafe fn polymorph() {
    let mut rng = thread_rng();
    let x: u32 = rng.gen_range(0..8);
    for a in 0..x {


        match x {
            0 => asm!(
            "add eax, 2",
            "sub eax, 2"
            ),
            1 => asm!(
            "add ecx, 3",
            "sub ecx, 3"
            ),
            2 => asm!(
            "inc ecx",
            "dec ecx"
            ),
            3 => asm!(
            "mov eax, eax",
            "sub eax, 8",
            "add eax, 8"
            ),
            4 => asm!(
            "mov ecx, ecx",
            "sub ecx, 4",
            "add ecx, 4"
            ),
            5 => asm!(
            "sub eax, 1",
            "inc eax"
            ),
            6 => asm!(
            "xor eax, eax",
            "mov eax, eax"
            ),
            7 => asm!(
            "xor ecx, ecx",
            "mov ecx, ecx"
            ),
            8 => asm!(
            "dec eax",
            "add eax, 1"
            ),
            _ => {
            }
        }
    }

}


#[test]
pub fn main() {
    unsafe { polymorph(); }

}
