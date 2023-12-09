use crate::{
    bindings::{shut_up_and_add, shut_up_and_sub, word},
    RArmSimKernel,
};
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_must_use)]

fn get_bit_range(instruction: word, range_start: u64, range_length: u32) -> u32 {
    (instruction >> range_start) & ((u64::pow(2, range_length) - 1) as u32)
}

fn mul_decode(instruction: word, k: &mut RArmSimKernel) {
    let rd_bits = get_bit_range(instruction, 16, 4);
    let rs_bits = get_bit_range(instruction, 8, 4);
    let rm_bits = get_bit_range(instruction, 0, 4);
    let temp: u64 = (k.regs[rm_bits as usize] as u64)
        .checked_mul(k.regs[rs_bits as usize] as u64)
        .unwrap_or(0);
    k.regs[rd_bits as usize] = (temp & 0xFFFFFFFF) as u32;
}

fn data_processor_decode(instruction: word, k: &mut RArmSimKernel) {
    let i_bit: u32 = get_bit_range(instruction, 25, 1);
    let _s_bit: u32 = get_bit_range(instruction, 20, 1);
    let opcode: u32 = get_bit_range(instruction, 21, 4);

    let rot_imm: u32 = get_bit_range(instruction, 8, 4) * 2;
    let immed_8: u32 = get_bit_range(instruction, 0, 8);
    let num: u32 = (immed_8.checked_shr(rot_imm).unwrap_or(0))
        | (immed_8.checked_shl(32 - rot_imm).unwrap_or(0));
    let six_four_range: u32 = get_bit_range(instruction, 4, 3);
    let shift_imm: u32 = get_bit_range(instruction, 7, 5);
    let rm: u32 = get_bit_range(instruction, 0, 4);
    let rm_value: u32 = if rm == 15 {
        k.regs[rm as usize] + 8
    } else {
        k.regs[rm as usize]
    };
    let mut temp = if rm == 15 {
        k.regs[rm as usize] + 8
    } else {
        k.regs[rm as usize]
    };
    let rn_bits = get_bit_range(instruction, 16, 4);
    let rd_bits = get_bit_range(instruction, 12, 4);

    // important for extended instructions
    let rs_bits = get_bit_range(instruction, 8, 4);
    let rs_value = if rs_bits == 15 {
        k.regs[rs_bits as usize] + 8
    } else {
        k.regs[rs_bits as usize]
    };
    let rs_bottom_8 = get_bit_range(rs_value, 0, 8);

    if six_four_range == 0b000 {
        // Logical Left shift by imm
        temp = rm_value.checked_shl(shift_imm).unwrap_or(0);
    } else if six_four_range == 0b010 {
        // lsr
        // Logical Right shift by imm
        temp = rm_value.checked_shr(shift_imm).unwrap_or(0);
    } else if six_four_range == 0b100 {
        // Arithmetic Right shift

        temp = rm_value.checked_shr(shift_imm).unwrap_or(0);
        let mut i = 0;
        let mut mask: String = String::new().to_owned();
        let stbit = (rm_value & 0x8000_0000)
            .checked_shr(31)
            .unwrap()
            .to_string()
            .to_owned();
        while i < shift_imm {
            mask.push_str(&stbit);
            i = i + 1;
        }
        i = 0;
        while i < 32 - shift_imm {
            mask.push_str("0");
            i = i + 1;
        }
        temp = temp | u32::from_str_radix(&mask, 2).unwrap();
    } else if six_four_range == 0b110 {
        // Rotate Right

        temp = (rm_value.checked_shr(shift_imm).unwrap_or(0))
            | (rm_value.checked_shl(32 - shift_imm).unwrap_or(0));
    } else if get_bit_range(instruction, 4, 4) == 0b0001 {
        // lsl apparently
        if rs_bottom_8 == 0 {
            temp = rm_value;
        } else if rs_bottom_8 < 32 {
            temp = rm_value.checked_shl(rs_bottom_8).unwrap_or(0);
        }
    } else if get_bit_range(instruction, 4, 4) == 0b0011 {
        // LSR by register
        if rs_bottom_8 == 0 {
            temp = rm_value;
        } else if rs_bottom_8 < 32 {
            temp = rm_value.checked_shr(rs_bottom_8).unwrap_or(0);
        }
    } else if get_bit_range(instruction, 4, 4) == 0b0111 {
        //ROR by register
        // TODO - double check this
        if rs_bottom_8 == 0 {
            temp = rm_value;
        } else if (rs_bottom_8 & 00011111) == 0 {
            temp = rm_value;
            // carry out rm[31]
        } else {
            let v = rs_bottom_8 & 00011111;
            temp = rm_value.rotate_right(v);
        }
    } else if get_bit_range(instruction, 4, 4) == 0b0101 {
        // ASR by register
        if rs_bottom_8 == 0 {
            temp = rm_value;
        } else if rs_bottom_8 < 32 {
            temp = rm_value.checked_shr(rs_bottom_8).unwrap_or(0);
            let mut i = 0;
            let mut mask: String = String::new().to_owned();
            let stbit = (rm_value & 0x8000_0000)
                .checked_shr(31)
                .unwrap()
                .to_string()
                .to_owned();
            while i < rs_bottom_8 {
                mask.push_str(&stbit);
                i = i + 1;
            }
            i = 0;
            while i < 32 - rs_bottom_8 {
                mask.push_str("0");
                i = i + 1;
            }
            temp = temp | u32::from_str_radix(&mask, 2).unwrap();
        } else {
            if (rm_value & 0x8000_0000) == 1 {
                temp = 0xFFFF_FFFF;
            } else {
                temp = 0;
            }
        }
    }

    if opcode == 0x0 {
        // AND instruction
        if i_bit != 0 {
            // immediate operand2
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] & num;
        } else {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] & temp;
        }
    } else if opcode == 0x1 {
        // EOR  (exclusive or) instruction
        if i_bit != 0 {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] ^ num;
        } else {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] ^ temp;
        }
    } else if opcode == 0x2 {
        // SUB instruction
        let mut cf: i32 = 0;
        unsafe {
            if i_bit != 0 {
                k.regs[rd_bits as usize] = shut_up_and_sub(k.regs[rn_bits as usize], num, &mut cf);
            } else {
                k.regs[rd_bits as usize] = shut_up_and_sub(k.regs[rn_bits as usize], temp, &mut cf);
            }
        }
    } else if opcode == 0x3 {
        // RSB (reverse sub) instruction
        if i_bit != 0 {
            k.regs[rd_bits as usize] =
                (num as i32 + ((k.regs[rn_bits as usize] as i32) * -1)) as u32;
        } else {
            k.regs[rd_bits as usize] =
                (temp as i32 + ((k.regs[rn_bits as usize] as i32) * -1)) as u32;
        }
    } else if opcode == 0x4 {
        // ADD instruction
        unsafe {
            if i_bit != 0 {
                k.regs[rd_bits as usize] = shut_up_and_add(k.regs[rn_bits as usize], num);
            } else {
                k.regs[rd_bits as usize] = shut_up_and_add(k.regs[rn_bits as usize], temp);
            }
        }
    } else if opcode == 0xc {
        // ORR (inclusive or) instruction
        if i_bit != 0 {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] | num;
        } else {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] | temp;
        }
    } else if opcode == 0xd {
        // MOV instruction

        if i_bit != 0 {
            // Immediate operand2
            k.regs[rd_bits as usize] = num; // https://stackoverflow.com/questions/65261859/why-cant-i-index-a-u32-with-a-u32
        } else {
            // needs work
            // https://stackoverflow.com/questions/51571066/what-are-the-exact-semantics-of-rusts-shift-operators
            k.regs[rd_bits as usize] = temp;
        }
    } else if opcode == 0xe {
        // BIC (Bit Clear) instruction

        if i_bit != 0 {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] & !num;
        } else {
            k.regs[rd_bits as usize] = k.regs[rn_bits as usize] & !temp;
        }
    } else if opcode == 0xf {
        // MVN (Move not) instruction
        if i_bit != 0 {
            // Immediate operand2
            k.regs[rd_bits as usize] = !num; // https://stackoverflow.com/questions/65261859/why-cant-i-index-a-u32-with-a-u32
        } else {
            // https://stackoverflow.com/questions/51571066/what-are-the-exact-semantics-of-rusts-shift-operators
            k.regs[rd_bits as usize] = !temp;
        }
    } else {
        println!("Not yet implemented.");
    }
}

fn load_instruction_decode(instruction: word, k: &mut RArmSimKernel) {
    let i_bit = get_bit_range(instruction, 25, 1);
    let p_bit = get_bit_range(instruction, 24, 1);
    let u_bit = get_bit_range(instruction, 23, 1);
    let b_bit = get_bit_range(instruction, 22, 1);
    let w_bit = get_bit_range(instruction, 21, 1);
    let l_bit = get_bit_range(instruction, 20, 1);
    let rn_bits = get_bit_range(instruction, 16, 4);
    let rm_bits = get_bit_range(instruction, 0, 4);
    let rd = get_bit_range(instruction, 12, 4);
    let mut address: u32 = 0;
    let offset_12 = get_bit_range(instruction, 0, 12);
    let rn_reg = if rn_bits == 15 {
        k.regs[rn_bits as usize].checked_add(8).unwrap()
    } else {
        k.regs[rn_bits as usize]
    };
    let rm_reg = if rm_bits == 15 {
        k.regs[rm_bits as usize] + 8
    } else {
        k.regs[rm_bits as usize]
    };

    if (i_bit == 0) & (p_bit == 1) {
        if u_bit == 1 {
            address = rn_reg + offset_12;
        } else {
            unsafe {
                let mut cf = 0;
                address = shut_up_and_sub(rn_reg, offset_12, &mut cf);
            }
        }
    } else if (i_bit == 1) & (p_bit == 1) & (get_bit_range(instruction, 4, 8) == 0) {
        if u_bit == 1 {
            address = rn_reg + rm_reg;
        } else {
            address = rn_reg - rm_reg;
        }
    } else if (i_bit == 1) & (p_bit == 1) {
        let shift_imm = get_bit_range(instruction, 7, 4);
        let shift = get_bit_range(instruction, 5, 2);
        let mut index = 0;
        if shift == 0b00 {
            index = rm_reg << shift_imm;
        } else if shift == 0b01 {
            index = rm_reg >> shift_imm;
        } else if shift == 0b10 {
            // asr
            index = rm_reg.checked_shr(shift_imm).unwrap_or(0);
            let mut i = 0;
            let mut mask: String = String::new().to_owned();
            let stbit = (rm_reg & 0x8000_0000)
                .checked_shr(31)
                .unwrap()
                .to_string()
                .to_owned();
            while i < shift_imm {
                mask.push_str(&stbit);
                i = i + 1;
            }
            i = 0;
            while i < 32 - shift_imm {
                mask.push_str("0");
                i = i + 1;
            }
            index = index | u32::from_str_radix(&mask, 2).unwrap();
        } else if shift == 0b11 {
            // ror
            index = rm_reg.rotate_right(shift_imm)
        }
        if u_bit == 1 {
            address = rn_reg + index;
        } else {
            address = rn_reg - index;
        }
    } else if (i_bit == 1) & (p_bit == 0) {
        address = rn_reg;
        if u_bit == 1 {
            k.regs[rn_bits as usize] = rn_reg + rm_reg;
        } else {
            k.regs[rn_bits as usize] = rn_reg - rm_reg;
        }
    } else if (i_bit == 0) & (p_bit == 0) {
        address = rn_reg;
        if u_bit == 1 {
            k.regs[rn_bits as usize] = rn_reg + offset_12;
        } else {
            k.regs[rn_bits as usize] = rn_reg - offset_12;
        }
    }

    if l_bit == 1 {
        if b_bit == 1 {
            let memory = k.host_load(address);

            let t = k.regs[rd as usize] & 0xFFFF_FF00;
            k.regs[rd as usize] = t | (memory & 0x0000_00FF);
        } else {
            let memory = k.host_load(address);

            k.regs[rd as usize] = memory;
        }
    } else {
        if b_bit == 1 {
            let memory = k.host_load(address);
            let t = memory & &0xFFFF_FF00;
            k.host_store(address, t | (k.regs[rd as usize] & 0x0000_00FF))
        } else {
            k.host_store(address, k.regs[rd as usize])
        }
    }

    if w_bit == 1 {
        k.regs[rn_bits as usize] = address;
    }
}

fn swi_decode(_instruction: word, h: &mut RArmSimKernel) -> i32 {
    h.stop = true;
    return 1;
}

fn load_store_multiple_decode(instruction: word, k: &mut RArmSimKernel) {
    let p_bit = get_bit_range(instruction, 24, 1);
    let u_bit = get_bit_range(instruction, 23, 1);
    let w_bit = get_bit_range(instruction, 21, 1);
    let rn_bits = get_bit_range(instruction, 16, 4);

    let registers = get_bit_range(instruction, 0, 16);

    let mut address: u32;
    let mut count = 0;
    let mut i = 0;
    while i < 16 {
        if get_bit_range(registers, i, 1) == 1 {
            count = count + 1;
        }
        i = i + 1;
    }
    i = 0;
    if p_bit == 0 && u_bit == 1 && get_bit_range(instruction, 20, 1) == 1 {
        // LDMIA
        address = k.regs[rn_bits as usize];
        while i < 15 {
            if get_bit_range(registers, i, 1) == 1 {
                k.regs[i as usize] = k.host_load(address);
                address = address + 4;
            }
            i = i + 1;
        }
        if get_bit_range(registers, 15, 1) == 1 {
            let value = k.regs[i as usize];
            k.regs[15] = value & 0xFFFF_FFFE;
        }
    } else if p_bit == 1 && u_bit == 0 && get_bit_range(instruction, 20, 1) == 0 {
        // STMDB
        address = k.regs[rn_bits as usize] - (count * 4);
        while i < 16 {
            if get_bit_range(registers, i, 1) == 1 {
                k.host_store(
                    address,
                    if i == 15 {
                        k.regs[i as usize] + 4
                    } else {
                        k.regs[i as usize]
                    },
                );
                address = address + 4;
            }
            i = i + 1;
        }

        // if w_bit == 1 {
        //     k.regs[rn_bits as usize] = k.regs[rn_bits as usize] - (count * 4)
        // }
    } else if p_bit == 0 && u_bit == 1 && get_bit_range(instruction, 20, 0) == 0 {
        // STMIA
        address = k.regs[rn_bits as usize];
        while i < 16 {
            if get_bit_range(registers, i, 1) == 1 {
                k.host_store(
                    address,
                    if i == 15 {
                        k.regs[i as usize] + 4
                    } else {
                        k.regs[i as usize]
                    },
                );
                address = address + 4;
            }
            i = i + 1;
        }
    }

    if w_bit == 1 {
        if u_bit == 1 {
            k.regs[rn_bits as usize] = k.regs[rn_bits as usize] + (count * 4);
        } else {
            k.regs[rn_bits as usize] = k.regs[rn_bits as usize] - (count * 4);
        }
    }
}

fn branch_or_link(instruction: word, k: &mut RArmSimKernel) {
    k.host_log("Branching!");
    let link_bit = get_bit_range(instruction, 24, 1);
    let mut addr_adder = get_bit_range(instruction, 0, 24);
    if link_bit == 1 {
        k.regs[14] = k.regs[15] + 4;
    }
    // Use masking to set higher bits
    if get_bit_range(addr_adder, 23, 1) == 1 {
        addr_adder = addr_adder | 0xFF00_0000;
    } else {
        addr_adder = addr_adder & 0x00FF_FFFF;
    }
    addr_adder = addr_adder << 2;
    let signed_addr = addr_adder as i32;
    let t = i64::from(addr_adder);
    k.regs[15] = (k.regs[15] as i32 + signed_addr + 4) as u32;
}

fn do_bx(instruction: word, k: &mut RArmSimKernel) {
    let rm_bits = get_bit_range(instruction, 0, 4);
    // T flag....?
    
    k.regs[15] = k.regs[rm_bits as usize] & 0xFFFF_FFFE;
    k.regs[15] = k.regs[15] - 4;
}

fn cmp(instruction: word, k: &mut RArmSimKernel) {
    let rn_bits = get_bit_range(instruction, 16, 4);
    let mut test_value = 0;
    let i_bit = get_bit_range(instruction, 25, 1);
    if (i_bit == 0) && (get_bit_range(instruction, 4, 8) == 0) {
        //register
        let rm_bits = get_bit_range(instruction, 0, 4);
        test_value = k.regs[rm_bits as usize];
    } else if i_bit == 1 {
        // immediate
        let immediate_8 = get_bit_range(instruction, 0, 8);
        let rotate_imm = get_bit_range(instruction, 8, 4);
        test_value = immediate_8.rotate_right(rotate_imm * 2);
    }
    let rn_value = k.regs[rn_bits as usize];

    // SET N BIT
    unsafe {
        let mut cf: i32 = 0;
        let result = shut_up_and_sub(rn_value, test_value, &mut cf);
        if result > 0x8000_0000 {
            k.cpsr = k.cpsr | 0x8000_0000;
        } else {
            k.cpsr = k.cpsr & 0x7FFF_FFFF;
        }
    }
    // SET Z BIT
    if rn_value.checked_sub(test_value).unwrap_or(1) == 0 {
        // result is zero
        k.cpsr = k.cpsr | 0x40000000;
    } else {
        // result is not zero
        k.cpsr = k.cpsr & 0xBFFFFFFF;
    }

    // SET C BIT
    // 1 if NOT BORROW, else 0
    if test_value > rn_value {
        // borrow
        k.cpsr = k.cpsr & 0xDFFFFFFF;
    } else {
        k.cpsr = k.cpsr | 0x20000000;
    }

    // SET V BIT
    unsafe {
        let mut cf: i32 = 0;
        let result = shut_up_and_sub(rn_value, test_value, &mut cf);
        let rn_top_bit = get_bit_range(rn_value, 31, 1);
        let test_top_bit = get_bit_range(test_value, 31, 1);
        let result_top_bit = get_bit_range(result, 31, 1);
        // https://stackoverflow.com/questions/68582809/how-is-overflow-detected-when-doing-binary-subtraction
        if rn_top_bit != test_top_bit && result_top_bit != rn_top_bit {
            // overflow
            k.cpsr = k.cpsr | 0x10000000;
        } else {
            // no overflow
            k.cpsr = k.cpsr & 0xEFFFFFFF;
        }
    }
}
pub fn decode(instruction: word, h: &mut RArmSimKernel) -> i32 {
    // print!(
    //     "# {} instruction: {:#010x}",
    //     h.stats.instructions, instruction
    // );
    // Get the flags from the current processor state register
    let n = get_bit_range(h.cpsr, 31, 1);
    let z = get_bit_range(h.cpsr, 30, 1);
    let c = get_bit_range(h.cpsr, 29, 1);
    let v = get_bit_range(h.cpsr, 28, 1);
    // let _q = get_bit_range(h.cpsr, 27, 1);

    let condition_field = get_bit_range(instruction, 28, 4);
    let mut retme: i32 = 0;
    if condition_field == 0b1111 {
        // Never (should not be hit but for completeness)
        // println!("");
        return retme;
    } else if condition_field == 0b0000 && z != 1 {
        // Equal
        // println!("");
        return retme;
    } else if condition_field == 0b0001 && z != 0 {
        // Not Equal
        // println!("");
        return retme;
    } else if condition_field == 0b0010 && c != 1 {
        // Carry set/unsigned higher or same
        // println!("");
        return retme;
    } else if condition_field == 0b0011 && c != 0 {
        // Carry clear/unsigned lower
        // println!("");
        return retme;
    } else if condition_field == 0b0100 && n != 1 {
        // negative
        // println!("");
        return retme;
    } else if condition_field == 0b0101 && n != 0 {
        // positive
        // println!("");
        return retme;
    } else if condition_field == 0b0110 && v != 1 {
        // overflow
        // println!("");
        return retme;
    } else if condition_field == 0b0111 && v != 0 {
        // no overflow
        // println!("");
        return retme;
    } else if condition_field == 0b1000 && !(c == 1 && z == 0) {
        // unsigned higher
        // println!("");
        return retme;
    } else if condition_field == 0b1001 && !(c == 0 || z == 1) {
        // unsigned lower or same
        // println!("");
        return retme;
    } else if condition_field == 0b1010 && !(n == v) {
        // signed greater than or equal
        // println!("");
        return retme;
    } else if condition_field == 0b1011 && !(n != v) {
        // signed less than
        // println!("");
        return retme;
    } else if condition_field == 0b1100 && !(z == 0 && n == v) {
        // signed greater than
        // println!("");
        return retme;
    } else if condition_field == 0b1101 && !(z == 1 || n != v) {
        // signed less than or equal
        // println!("");
        return retme;
    } else {
        // print!(" taken ");
        if (instruction & 0x0F000000) == 0x0F000000 {
            // println!("SWI");
            swi_decode(instruction, h);
        } else if get_bit_range(instruction, 4, 24) == 0x12_FFF1 {
            // println!("BX");
            do_bx(instruction, h);
            // retme = 39;
        } else if get_bit_range(instruction, 25, 3) == 0b101 {
            // println!("B / BL");
            branch_or_link(instruction, h);
            retme = 39;
        } else if ((get_bit_range(instruction, 20, 8) | 0b00100000) == 0b00110101)
            && (get_bit_range(instruction, 12, 4) == 0)
        {
            // println!("CMP");
            cmp(instruction, h);
        } else if get_bit_range(instruction, 25, 3) == 4 {
            // println!("LDM / STM");
            load_store_multiple_decode(instruction, h);
        } else if (instruction & 0x0FE0F0F0) == 0x00000090 {
            // println!("MUL");
            mul_decode(instruction, h);
        } else if get_bit_range(instruction, 26, 2) == 0x1 {
            // println!("LD / STR");
            load_instruction_decode(instruction, h);
        } else if (instruction & 0x03000000) == 0x3000000
            || (instruction & 0x03000000) == 0x1000000
            || (instruction & 0x03000000) == 0x2000000
            || (instruction & 0x03000000) == 0x0000000
        {
            // println!("MOV ETC.");
            data_processor_decode(instruction, h);
        } else {
            println!(
                "This instruction {:#010x} has not yet been implemented.",
                instruction
            );
            retme = 2;
            h.host_panic("I don't know that instruction.")
        }
        return retme;
    }
}
