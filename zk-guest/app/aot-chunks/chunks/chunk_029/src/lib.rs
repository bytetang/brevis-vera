pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2223728u32;
pub const PC_MAX: u32 = 2225840u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x0021ee70,
        block_0x0021ee74,
        block_0x0021ee80,
        block_0x0021ee88,
        block_0x0021ee94,
        block_0x0021ee9c,
        block_0x0021eed4,
        block_0x0021eedc,
        block_0x0021ef08,
        block_0x0021ef14,
        block_0x0021ef1c,
        block_0x0021ef28,
        block_0x0021ef3c,
        block_0x0021ef58,
        block_0x0021ef64,
        block_0x0021ef6c,
        block_0x0021ef78,
        block_0x0021ef94,
        block_0x0021efb0,
        block_0x0021efcc,
        block_0x0021f004,
        block_0x0021f010,
        block_0x0021f02c,
        block_0x0021f05c,
        block_0x0021f064,
        block_0x0021f06c,
        block_0x0021f074,
        block_0x0021f088,
        block_0x0021f0b0,
        block_0x0021f0b8,
        block_0x0021f0c0,
        block_0x0021f0d8,
        block_0x0021f0e0,
        block_0x0021f0ec,
        block_0x0021f10c,
        block_0x0021f110,
        block_0x0021f140,
        block_0x0021f148,
        block_0x0021f160,
        block_0x0021f164,
        block_0x0021f174,
        block_0x0021f178,
        block_0x0021f180,
        block_0x0021f190,
        block_0x0021f194,
        block_0x0021f1a4,
        block_0x0021f1ac,
        block_0x0021f1cc,
        block_0x0021f1d4,
        block_0x0021f1f0,
        block_0x0021f208,
        block_0x0021f228,
        block_0x0021f230,
        block_0x0021f240,
        block_0x0021f248,
        block_0x0021f250,
        block_0x0021f25c,
        block_0x0021f278,
        block_0x0021f288,
        block_0x0021f298,
        block_0x0021f2d4,
        block_0x0021f2ec,
        block_0x0021f308,
        block_0x0021f324,
        block_0x0021f360,
        block_0x0021f380,
        block_0x0021f3b0,
        block_0x0021f3b8,
        block_0x0021f3c0,
        block_0x0021f3c8,
        block_0x0021f3dc,
        block_0x0021f404,
        block_0x0021f40c,
        block_0x0021f414,
        block_0x0021f42c,
        block_0x0021f434,
        block_0x0021f440,
        block_0x0021f460,
        block_0x0021f464,
        block_0x0021f494,
        block_0x0021f49c,
        block_0x0021f4b4,
        block_0x0021f4b8,
        block_0x0021f4c8,
        block_0x0021f4cc,
        block_0x0021f4d4,
        block_0x0021f4e4,
        block_0x0021f4e8,
        block_0x0021f4f8,
        block_0x0021f500,
        block_0x0021f524,
        block_0x0021f52c,
        block_0x0021f534,
        block_0x0021f538,
        block_0x0021f560,
        block_0x0021f568,
        block_0x0021f588,
        block_0x0021f58c,
        block_0x0021f5a8,
        block_0x0021f5b0,
        block_0x0021f5cc,
        block_0x0021f5d4,
        block_0x0021f5e4,
        block_0x0021f5ec,
        block_0x0021f5f4,
        block_0x0021f600,
        block_0x0021f61c,
        block_0x0021f634,
        block_0x0021f644,
        block_0x0021f654,
        block_0x0021f694,
        block_0x0021f6b0,
    ];
    const IDX: [u16; 529usize] = [
        1u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16,
        0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16,
        0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16, 26u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16, 41u16,
        42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 55u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        67u16, 0u16, 68u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16,
        74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16,
        0u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 112u16,
    ];
    if pc < 2223728u32 || pc > 2225840u32 {
        return None;
    }
    let word_offset = ((pc - 2223728u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021ee70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2223992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef78));
    } else {
        emu.pc = 2223732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee74));
    }
}
#[inline(always)]
pub fn block_0x0021ee74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 0u32, 2223736u32);
    emu.adi_no_count(17usize, 0usize, 48u32, 2223740u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2224020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef94));
    } else {
        emu.pc = 2223744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee80));
    }
}
#[inline(always)]
pub fn block_0x0021ee80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 3u32, 2223748u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2224048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021efb0));
    } else {
        emu.pc = 2223752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee88));
    }
}
#[inline(always)]
pub fn block_0x0021ee88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 2u32, 2223756u32);
    emu.sh_no_count(16usize, 14usize, 0u32, 2223760u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(0usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2223836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eedc));
    } else {
        emu.pc = 2223764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee94));
    }
}
#[inline(always)]
pub fn block_0x0021ee94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 14usize, 4u32, 2223768u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2223912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef28));
    } else {
        emu.pc = 2223772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee9c));
    }
}
#[inline]
pub fn block_0x0021ee9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2223776u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2223780u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1944u32, 2223784u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2223788u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2223792u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2223796u32);
    emu.sw_no_count(12usize, 14usize, 8u32, 2223800u32)?;
    emu.sh_no_count(15usize, 14usize, 12u32, 2223804u32)?;
    emu.sw_no_count(16usize, 14usize, 16u32, 2223808u32)?;
    emu.sw_no_count(17usize, 14usize, 20u32, 2223812u32)?;
    emu.sh_no_count(15usize, 14usize, 24u32, 2223816u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2223820u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2223824u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2223900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef1c));
    } else {
        emu.pc = 2223828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eed4));
    }
}
#[inline(always)]
pub fn block_0x0021eed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2223832u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef58));
}
#[inline]
pub fn block_0x0021eedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 0usize, 12usize, 2223840u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2223844u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1945u32, 2223848u32);
    emu.sh_no_count(16usize, 14usize, 24u32, 2223852u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2223856u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2223860u32)?;
    emu.sw_no_count(17usize, 14usize, 4u32, 2223864u32)?;
    emu.sw_no_count(16usize, 14usize, 8u32, 2223868u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2223872u32)?;
    emu.sw_no_count(15usize, 14usize, 16u32, 2223876u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2223900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef1c));
    } else {
        emu.pc = 2223880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef08));
    }
}
#[inline(always)]
pub fn block_0x0021ef08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2223884u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2223888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2223972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef64));
    } else {
        emu.pc = 2223892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef14));
    }
}
#[inline(always)]
pub fn block_0x0021ef14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 12usize, 2223896u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef58));
}
#[inline(always)]
pub fn block_0x0021ef1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2223904u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2223908u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223912u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ef28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 12usize, 11usize, 2223916u32);
    emu.sw_no_count(11usize, 14usize, 8u32, 2223920u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2223924u32)?;
    emu.sw_no_count(12usize, 14usize, 16u32, 2223928u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2223980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef6c));
    } else {
        emu.pc = 2223932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ef3c));
    }
}
#[inline(always)]
pub fn block_0x0021ef3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2223936u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2223940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1944u32, 2223944u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2223948u32);
    emu.sh_no_count(10usize, 14usize, 24u32, 2223952u32)?;
    emu.sw_no_count(11usize, 14usize, 28u32, 2223956u32)?;
    emu.sw_no_count(12usize, 14usize, 32u32, 2223960u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2223960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef58));
}
#[inline(always)]
pub fn block_0x0021ef58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(0usize, 14usize, 36u32, 2223964u32)?;
    emu.sw_no_count(13usize, 14usize, 40u32, 2223968u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2223972u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2223972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ef64));
}
#[inline(always)]
pub fn block_0x0021ef64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2223976u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223980u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ef6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2223984u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2223988u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223992u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ef78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2223996u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1792u32, 2224000u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2224004u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1828u32, 2224008u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2224012u32);
    emu.apc_no_count(1usize, 2224012u32, 4294963200u32, 2224016u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ef94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224024u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1844u32, 2224028u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2224032u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1876u32, 2224036u32);
    emu.adi_no_count(11usize, 0usize, 31u32, 2224040u32);
    emu.apc_no_count(1usize, 2224040u32, 4294963200u32, 2224044u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021efb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224052u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1892u32, 2224056u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2224060u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1928u32, 2224064u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2224068u32);
    emu.apc_no_count(1usize, 2224068u32, 4294963200u32, 2224072u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021efcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2224080u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2224084u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2224088u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2224092u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2224096u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2224100u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2224104u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2224108u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2224112u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2224116u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2224120u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2224124u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2224128u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2224876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f2ec));
    } else {
        emu.pc = 2224132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f004));
    }
}
#[inline(always)]
pub fn block_0x0021f004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2224136u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2224140u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2224904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f308));
    } else {
        emu.pc = 2224144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f010));
    }
}
#[inline(always)]
pub fn block_0x0021f010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2224148u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2224152u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224156u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2224160u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2224164u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2224168u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2224264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f088));
    } else {
        emu.pc = 2224172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f02c));
    }
}
#[inline]
pub fn block_0x0021f02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2224176u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2224180u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2224184u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2224188u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2224192u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2224196u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2224200u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2224204u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2224208u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2224212u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2224216u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2224304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0b0));
    } else {
        emu.pc = 2224220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f05c));
    }
}
#[inline(always)]
pub fn block_0x0021f05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2224224u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2224312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0b8));
    } else {
        emu.pc = 2224228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f064));
    }
}
#[inline(always)]
pub fn block_0x0021f064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2224232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2224320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0c0));
    } else {
        emu.pc = 2224236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f06c));
    }
}
#[inline(always)]
pub fn block_0x0021f06c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2224240u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2224396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f10c));
    } else {
        emu.pc = 2224244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f074));
    }
}
#[inline(always)]
pub fn block_0x0021f074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2224248u32);
    emu.adi_no_count(17usize, 17usize, 4294966221u32, 2224252u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2224256u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2224260u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2224264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f110));
}
#[inline]
pub fn block_0x0021f088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2224268u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2224272u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2224276u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2224280u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2224284u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2224288u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2224292u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2224296u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2224300u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2224220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f05c));
    } else {
        emu.pc = 2224304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0b0));
    }
}
#[inline(always)]
pub fn block_0x0021f0b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2224308u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f110));
}
#[inline(always)]
pub fn block_0x0021f0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2224316u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f110));
}
#[inline(always)]
pub fn block_0x0021f0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2224324u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2224328u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2224332u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2224336u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2224340u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2224352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0e0));
    } else {
        emu.pc = 2224344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f0d8));
    }
}
#[inline(always)]
pub fn block_0x0021f0d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224348u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f0ec));
}
#[inline(always)]
pub fn block_0x0021f0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2224356u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2224360u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2224364u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2224364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f0ec));
}
#[inline(always)]
pub fn block_0x0021f0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2224368u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2224372u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2224376u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2224380u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2224384u32);
    emu.adi_no_count(17usize, 11usize, 4294966220u32, 2224388u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2224392u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2224396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f110));
}
#[inline(always)]
pub fn block_0x0021f10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2224400u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2224400u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f110));
}
#[inline]
pub fn block_0x0021f110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 11usize, 255u32, 2224404u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2224408u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2224412u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2224416u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2224420u32)?;
    emu.sw_no_count(22usize, 2usize, 8u32, 2224424u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2224428u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2224432u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2224436u32)?;
    emu.sh_no_count(17usize, 2usize, 24u32, 2224440u32)?;
    emu.sb_no_count(11usize, 2usize, 26u32, 2224444u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2224480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f160));
    } else {
        emu.pc = 2224448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f140));
    }
}
#[inline(always)]
pub fn block_0x0021f140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2224452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2224504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f178));
    } else {
        emu.pc = 2224456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f148));
    }
}
#[inline(always)]
pub fn block_0x0021f148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2224460u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2224464u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2030u32, 2224468u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2224472u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2224476u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2224480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f288));
}
#[inline(always)]
pub fn block_0x0021f160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2224532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f194));
    } else {
        emu.pc = 2224484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f164));
    }
}
#[inline(always)]
pub fn block_0x0021f164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2224488u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 2029u32, 2224492u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2224496u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2224548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1a4));
    } else {
        emu.pc = 2224500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f174));
    }
}
#[inline(always)]
pub fn block_0x0021f174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2224504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f1ac));
}
#[inline(always)]
pub fn block_0x0021f178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 1u32, 2224508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2224688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f230));
    } else {
        emu.pc = 2224512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f180));
    }
}
#[inline(always)]
pub fn block_0x0021f180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2224516u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 2029u32, 2224520u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2224524u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2224704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f240));
    } else {
        emu.pc = 2224528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f190));
    }
}
#[inline(always)]
pub fn block_0x0021f190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2224532u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f248));
}
#[inline(always)]
pub fn block_0x0021f194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2224536u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 2028u32, 2224540u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2224544u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2224556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1ac));
    } else {
        emu.pc = 2224548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1a4));
    }
}
#[inline(always)]
pub fn block_0x0021f1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2224552u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2224556u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2224556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f1ac));
}
#[inline(always)]
pub fn block_0x0021f1ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 48u32, 2224560u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2224564u32);
    emu.adi_no_count(21usize, 15usize, 0u32, 2224568u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2224572u32);
    emu.adi_no_count(20usize, 16usize, 0u32, 2224576u32);
    emu.adi_no_count(13usize, 16usize, 0u32, 2224580u32);
    emu.apc_no_count(1usize, 2224580u32, 0u32, 2224584u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2224592u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2224624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1f0));
    } else {
        emu.pc = 2224596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f1d4));
    }
}
#[inline(always)]
pub fn block_0x0021f1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2224600u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2224604u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2224608u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2224612u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2224616u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2224620u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2224624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f208));
}
#[inline(always)]
pub fn block_0x0021f1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2224628u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2224632u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2224636u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2224640u32);
    emu.apc_no_count(1usize, 2224640u32, 20480u32, 2224644u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2224652u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2224656u32)?;
    emu.lh_no_count(12usize, 2usize, 44u32, 2224660u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2224664u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2224668u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2224672u32);
    emu.apc_no_count(1usize, 2224672u32, 0u32, 2224676u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2224684u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2224688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f298));
}
#[inline(always)]
pub fn block_0x0021f230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2224692u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 2028u32, 2224696u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2224700u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2224712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f248));
    } else {
        emu.pc = 2224704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f240));
    }
}
#[inline(always)]
pub fn block_0x0021f240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2224708u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2224712u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2224712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f248));
}
#[inline(always)]
pub fn block_0x0021f248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2224716u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2224760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f278));
    } else {
        emu.pc = 2224720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f250));
    }
}
#[inline(always)]
pub fn block_0x0021f250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2224724u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2224728u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2224852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f2d4));
    } else {
        emu.pc = 2224732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f25c));
    }
}
#[inline(always)]
pub fn block_0x0021f25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224736u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1945u32, 2224740u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2224744u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2224748u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2224752u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2224756u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2224760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f298));
}
#[inline(always)]
pub fn block_0x0021f278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2224764u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2224768u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2033u32, 2224772u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2224776u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2224776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f288));
}
#[inline(always)]
pub fn block_0x0021f288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2224780u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2224784u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2224788u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2224792u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2224792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f298));
}
#[inline]
pub fn block_0x0021f298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 9usize, 0u32, 2224796u32)?;
    emu.sw_no_count(22usize, 9usize, 4u32, 2224800u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2224804u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2224808u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2224812u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2224816u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2224820u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2224824u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2224828u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2224832u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2224836u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2224840u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2224844u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2224848u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224852u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224856u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 2036u32, 2224860u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2224864u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2224868u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2224872u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2224876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2224792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f298));
}
#[inline(always)]
pub fn block_0x0021f2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224880u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1892u32, 2224884u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2224888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1948u32, 2224892u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2224896u32);
    emu.apc_no_count(1usize, 2224896u32, 4294959104u32, 2224900u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2224908u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1964u32, 2224912u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2224916u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 2012u32, 2224920u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2224924u32);
    emu.apc_no_count(1usize, 2224924u32, 4294959104u32, 2224928u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2224932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021f324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2224936u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2224940u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2224944u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2224948u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2224952u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2224956u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2224960u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2224964u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2224968u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2224972u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2224976u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2224980u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2224984u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2224988u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2225812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f694));
    } else {
        emu.pc = 2224992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f360));
    }
}
#[inline(always)]
pub fn block_0x0021f360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2224996u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2225000u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2225004u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225008u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2225012u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2225016u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2225020u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2225116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3dc));
    } else {
        emu.pc = 2225024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f380));
    }
}
#[inline]
pub fn block_0x0021f380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2225028u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2225032u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2225036u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2225040u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2225044u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2225048u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2225052u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2225056u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2225060u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2225064u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2225068u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2225156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f404));
    } else {
        emu.pc = 2225072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3b0));
    }
}
#[inline(always)]
pub fn block_0x0021f3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2225076u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2225164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f40c));
    } else {
        emu.pc = 2225080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3b8));
    }
}
#[inline(always)]
pub fn block_0x0021f3b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2225084u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2225172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f414));
    } else {
        emu.pc = 2225088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3c0));
    }
}
#[inline(always)]
pub fn block_0x0021f3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2225092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2225248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f460));
    } else {
        emu.pc = 2225096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3c8));
    }
}
#[inline(always)]
pub fn block_0x0021f3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2225100u32);
    emu.adi_no_count(11usize, 17usize, 4294966221u32, 2225104u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2225108u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2225112u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2225116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f464));
}
#[inline]
pub fn block_0x0021f3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2225120u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2225124u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2225128u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2225132u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2225136u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2225140u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2225144u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2225148u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2225152u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2225072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f3b0));
    } else {
        emu.pc = 2225156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f404));
    }
}
#[inline(always)]
pub fn block_0x0021f404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 3u32, 2225160u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2225164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f464));
}
#[inline(always)]
pub fn block_0x0021f40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 2u32, 2225168u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2225172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f464));
}
#[inline(always)]
pub fn block_0x0021f414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2225176u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2225180u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2225184u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2225188u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2225192u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2225204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f434));
    } else {
        emu.pc = 2225196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f42c));
    }
}
#[inline(always)]
pub fn block_0x0021f42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225200u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2225204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f440));
}
#[inline(always)]
pub fn block_0x0021f434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2225208u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2225212u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2225216u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2225216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f440));
}
#[inline(always)]
pub fn block_0x0021f440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2225220u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2225224u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2225228u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2225232u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2225236u32);
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2225240u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2225244u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2225248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225252u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f464));
}
#[inline(always)]
pub fn block_0x0021f460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 4u32, 2225252u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2225252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f464));
}
#[inline]
pub fn block_0x0021f464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 17usize, 255u32, 2225256u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2225260u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2225264u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2225268u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2225272u32)?;
    emu.sw_no_count(23usize, 2usize, 8u32, 2225276u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2225280u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2225284u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2225288u32)?;
    emu.sh_no_count(11usize, 2usize, 24u32, 2225292u32)?;
    emu.sb_no_count(17usize, 2usize, 26u32, 2225296u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2225332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4b4));
    } else {
        emu.pc = 2225300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f494));
    }
}
#[inline(always)]
pub fn block_0x0021f494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2225304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2225356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4cc));
    } else {
        emu.pc = 2225308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f49c));
    }
}
#[inline(always)]
pub fn block_0x0021f49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2225312u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2225316u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2030u32, 2225320u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2225324u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2225328u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2225332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225732u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f644));
}
#[inline(always)]
pub fn block_0x0021f4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2225384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4e8));
    } else {
        emu.pc = 2225336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4b8));
    }
}
#[inline(always)]
pub fn block_0x0021f4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2225340u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 2029u32, 2225344u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2225348u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2225400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4f8));
    } else {
        emu.pc = 2225352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4c8));
    }
}
#[inline(always)]
pub fn block_0x0021f4c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2225356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225408u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f500));
}
#[inline(always)]
pub fn block_0x0021f4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 1u32, 2225360u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2225620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5d4));
    } else {
        emu.pc = 2225364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4d4));
    }
}
#[inline(always)]
pub fn block_0x0021f4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2225368u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 2029u32, 2225372u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2225376u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2225636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5e4));
    } else {
        emu.pc = 2225380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4e4));
    }
}
#[inline(always)]
pub fn block_0x0021f4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2225384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f5ec));
}
#[inline(always)]
pub fn block_0x0021f4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2225388u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 2028u32, 2225392u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2225396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2225408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f500));
    } else {
        emu.pc = 2225400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f4f8));
    }
}
#[inline(always)]
pub fn block_0x0021f4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2225404u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2225408u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2225408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f500));
}
#[inline]
pub fn block_0x0021f500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 16u32, 2225412u32);
    emu.sai_no_count(10usize, 11usize, 1055u32, 2225416u32);
    emu.ani_no_count(10usize, 10usize, 4294967279u32, 2225420u32);
    emu.adi_no_count(10usize, 10usize, 5u32, 2225424u32);
    emu.sai_no_count(11usize, 11usize, 1040u32, 2225428u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2225432u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2225436u32);
    emu.adi_no_count(21usize, 10usize, 21u32, 2225440u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2225840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6b0));
    } else {
        emu.pc = 2225444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f524));
    }
}
#[inline(always)]
pub fn block_0x0021f524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 15u32, 2225448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2225460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f534));
    } else {
        emu.pc = 2225452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f52c));
    }
}
#[inline(always)]
pub fn block_0x0021f52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4294934528u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225456u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2225460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f538));
}
#[inline(always)]
pub fn block_0x0021f534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 18usize, 2225464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2225464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f538));
}
#[inline]
pub fn block_0x0021f538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2225468u32);
    emu.sai_no_count(20usize, 10usize, 1040u32, 2225472u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2225476u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2225480u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2225484u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2225488u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2225492u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2225496u32);
    emu.apc_no_count(1usize, 2225496u32, 4096u32, 2225500u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2225508u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2225548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f58c));
    } else {
        emu.pc = 2225512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f568));
    }
}
#[inline(always)]
pub fn block_0x0021f568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2225516u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2225520u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2225524u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2225528u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2225532u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2225536u32)?;
    emu.lh_no_count(12usize, 2usize, 40u32, 2225540u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2225584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5b0));
    } else {
        emu.pc = 2225544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f588));
    }
}
#[inline(always)]
pub fn block_0x0021f588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2225548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f5f4));
}
#[inline(always)]
pub fn block_0x0021f58c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2225552u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2225556u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2225560u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2225564u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2225568u32);
    emu.apc_no_count(1usize, 2225568u32, 20480u32, 2225572u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(12usize, 2usize, 40u32, 2225580u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2225652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5f4));
    } else {
        emu.pc = 2225584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5b0));
    }
}
#[inline(always)]
pub fn block_0x0021f5b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2225588u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2225592u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2225596u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2225600u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2225604u32);
    emu.apc_no_count(1usize, 2225604u32, 0u32, 2225608u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f5cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2225616u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2225620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f654));
}
#[inline(always)]
pub fn block_0x0021f5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2225624u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 2028u32, 2225628u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2225632u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2225644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5ec));
    } else {
        emu.pc = 2225636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5e4));
    }
}
#[inline(always)]
pub fn block_0x0021f5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2225640u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2225644u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2225644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f5ec));
}
#[inline(always)]
pub fn block_0x0021f5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2225648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2225716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f634));
    } else {
        emu.pc = 2225652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f5f4));
    }
}
#[inline(always)]
pub fn block_0x0021f5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2225656u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2225660u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2225692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f61c));
    } else {
        emu.pc = 2225664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f600));
    }
}
#[inline(always)]
pub fn block_0x0021f600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225668u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1945u32, 2225672u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2225676u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2225680u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2225684u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2225688u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2225692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f654));
}
#[inline(always)]
pub fn block_0x0021f61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225696u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 2036u32, 2225700u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2225704u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2225708u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2225712u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2225716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f654));
}
#[inline(always)]
pub fn block_0x0021f634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2225720u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2225724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2033u32, 2225728u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2225732u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2225732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f644));
}
#[inline(always)]
pub fn block_0x0021f644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2225736u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2225740u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2225744u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2225748u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2225748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f654));
}
#[inline]
pub fn block_0x0021f654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 9usize, 0u32, 2225752u32)?;
    emu.sw_no_count(23usize, 9usize, 4u32, 2225756u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2225760u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2225764u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2225768u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2225772u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2225776u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2225780u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2225784u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2225788u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2225792u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2225796u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2225800u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2225804u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2225808u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225812u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(0u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225816u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1892u32, 2225820u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2225824u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 2040u32, 2225828u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2225832u32);
    emu.apc_no_count(1usize, 2225832u32, 4294959104u32, 2225836u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021f6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2225844u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965256u32, 2225848u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2225852u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965296u32, 2225856u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2225860u32);
    emu.apc_no_count(1usize, 2225860u32, 4294959104u32, 2225864u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2225868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
