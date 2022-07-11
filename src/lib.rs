use std::arch::asm;
use rand::Rng;

#[inline]
pub unsafe fn polymorph() {

    let x: u32 = rng.gen_range(0..8);
    for a in 0..x {

        match x {
            0 => asm!(
            "add eax, {x}",
            "sub eax, {x}"
            ),
            1 => asm!(
            "add ecx, {x}",
            "sub ecx, {x}"
            ),
            2 => asm!(
            "inc ecx",
            "dec ecx"
            ),
            3 => asm!(
            "mov eax, eax",
            "sub eax, {x}",
            "add eax, {x}"
            ),
            4 => asm!(
            "mov ecx, ecx",
            "sub ecx, {x}",
            "add ecx, {x}"
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
            "pop ecx",
            "push ecx"
            ),
            8 => asm!(
            "dec eax",
            "add eax, 1"
            ),
            _ => {
                println!("wtf");
            }
        }
    }

}

