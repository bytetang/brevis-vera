pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2206768u32;
pub const PC_MAX: u32 = 2209080u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x0021ac30,
        block_0x0021ac40,
        block_0x0021ac94,
        block_0x0021aca4,
        block_0x0021aca8,
        block_0x0021acb4,
        block_0x0021acd0,
        block_0x0021acec,
        block_0x0021ad00,
        block_0x0021ad08,
        block_0x0021ad18,
        block_0x0021ad1c,
        block_0x0021ad48,
        block_0x0021ad50,
        block_0x0021ad54,
        block_0x0021ad5c,
        block_0x0021ad68,
        block_0x0021ad78,
        block_0x0021ad88,
        block_0x0021ad90,
        block_0x0021adb0,
        block_0x0021adcc,
        block_0x0021add0,
        block_0x0021adec,
        block_0x0021adf4,
        block_0x0021ae20,
        block_0x0021ae58,
        block_0x0021ae84,
        block_0x0021aeb4,
        block_0x0021aec8,
        block_0x0021aef0,
        block_0x0021af10,
        block_0x0021af1c,
        block_0x0021af20,
        block_0x0021af3c,
        block_0x0021af40,
        block_0x0021af50,
        block_0x0021af58,
        block_0x0021af60,
        block_0x0021af70,
        block_0x0021af90,
        block_0x0021afa4,
        block_0x0021afb8,
        block_0x0021afd0,
        block_0x0021aff8,
        block_0x0021b018,
        block_0x0021b020,
        block_0x0021b030,
        block_0x0021b034,
        block_0x0021b038,
        block_0x0021b03c,
        block_0x0021b048,
        block_0x0021b054,
        block_0x0021b060,
        block_0x0021b064,
        block_0x0021b074,
        block_0x0021b088,
        block_0x0021b09c,
        block_0x0021b0a4,
        block_0x0021b0b8,
        block_0x0021b0bc,
        block_0x0021b0c0,
        block_0x0021b0c4,
        block_0x0021b0d4,
        block_0x0021b0f0,
        block_0x0021b104,
        block_0x0021b118,
        block_0x0021b13c,
        block_0x0021b140,
        block_0x0021b178,
        block_0x0021b18c,
        block_0x0021b1a0,
        block_0x0021b1b4,
        block_0x0021b1c4,
        block_0x0021b1d0,
        block_0x0021b1ec,
        block_0x0021b1fc,
        block_0x0021b240,
        block_0x0021b294,
        block_0x0021b2a0,
        block_0x0021b2d8,
        block_0x0021b2dc,
        block_0x0021b2e0,
        block_0x0021b2f8,
        block_0x0021b2fc,
        block_0x0021b318,
        block_0x0021b32c,
        block_0x0021b330,
        block_0x0021b334,
        block_0x0021b360,
        block_0x0021b380,
        block_0x0021b38c,
        block_0x0021b3ac,
        block_0x0021b3cc,
        block_0x0021b3fc,
        block_0x0021b410,
        block_0x0021b428,
        block_0x0021b438,
        block_0x0021b440,
        block_0x0021b450,
        block_0x0021b460,
        block_0x0021b480,
        block_0x0021b488,
        block_0x0021b4b8,
        block_0x0021b4c8,
        block_0x0021b4cc,
        block_0x0021b500,
        block_0x0021b510,
        block_0x0021b52c,
        block_0x0021b538,
    ];
    const IDX: [u16; 579usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 15u16, 0u16, 16u16, 0u16, 0u16,
        17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 49u16,
        50u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16, 0u16, 0u16, 54u16, 55u16,
        0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 62u16, 63u16, 0u16,
        0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16,
        80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 81u16, 82u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16,
        0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        110u16,
    ];
    if pc < 2206768u32 || pc > 2209080u32 {
        return None;
    }
    let word_offset = ((pc - 2206768u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021ac30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2206772u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2206776u32)?;
    emu.apc_no_count(1usize, 2206776u32, 4294897664u32, 2206780u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206784u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ac40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2206788u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2206792u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2206796u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2206800u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206804u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966432u32, 2206808u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2206812u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2206816u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2206820u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2206824u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2206828u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2206832u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2206836u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2206840u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2206844u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2206848u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2206852u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2206856u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2206860u32);
    emu.apc_no_count(1usize, 2206860u32, 0u32, 2206864u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ac94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2206872u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2206876u32)?;
    emu.apc_no_count(1usize, 2206876u32, 4294963200u32, 2206880u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2206884u32));
}
#[inline(always)]
pub fn block_0x0021aca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2206892u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2206896u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206900u32;
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
pub fn block_0x0021acb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2206904u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2206908u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2206912u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2206916u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2206920u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2206924u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206928u32;
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
pub fn block_0x0021acd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2206932u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2206936u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2206940u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2206944u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2206948u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2206952u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2206956u32;
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
pub fn block_0x0021acec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2206960u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2206964u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966516u32, 2206968u32);
    emu.apc_no_count(6usize, 2206968u32, 28672u32, 2206972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2206976u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ad00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2206980u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2207000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad18));
    } else {
        emu.pc = 2206984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad08));
    }
}
#[inline(always)]
pub fn block_0x0021ad08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2206988u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2206992u32);
    emu.apc_no_count(6usize, 2206992u32, 4294864896u32, 2206996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2207000u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ad18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207004u32;
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
#[inline]
pub fn block_0x0021ad1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207008u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207012u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207016u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2207020u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2207024u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2207028u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2207032u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2207036u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2207040u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2207044u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2207056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad50));
    } else {
        emu.pc = 2207048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad48));
    }
}
#[inline(always)]
pub fn block_0x0021ad48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2207052u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad68));
}
#[inline(always)]
pub fn block_0x0021ad50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2207068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad5c));
    } else {
        emu.pc = 2207060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad54));
    }
}
#[inline(always)]
pub fn block_0x0021ad54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2207064u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad68));
}
#[inline(always)]
pub fn block_0x0021ad5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2207072u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2207076u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2207080u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2207080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ad68));
}
#[inline(always)]
pub fn block_0x0021ad68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2207084u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2207088u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207092u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2207120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad90));
    } else {
        emu.pc = 2207096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad78));
    }
}
#[inline(always)]
pub fn block_0x0021ad78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2207100u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2207104u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2207108u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2207180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adcc));
    } else {
        emu.pc = 2207112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad88));
    }
}
#[inline(always)]
pub fn block_0x0021ad88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2207116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae58));
}
#[inline(always)]
pub fn block_0x0021ad90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2207124u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2207128u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2207132u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2207136u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2207140u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2207144u32);
    emu.apc_no_count(1usize, 2207144u32, 4294959104u32, 2207148u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021adb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2207156u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2207160u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2207164u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2207168u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2207172u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2207176u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2207112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ad88));
    } else {
        emu.pc = 2207180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adcc));
    }
}
#[inline(always)]
pub fn block_0x0021adcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2207212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adec));
    } else {
        emu.pc = 2207184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021add0));
    }
}
#[inline(always)]
pub fn block_0x0021add0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2207188u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207192u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2207196u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207200u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207204u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2207208u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2207212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae58));
}
#[inline(always)]
pub fn block_0x0021adec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2207216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2207264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ae20));
    } else {
        emu.pc = 2207220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021adf4));
    }
}
#[inline]
pub fn block_0x0021adf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2207224u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2207228u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207232u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2207236u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2207240u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207244u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2207248u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207252u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2207256u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2207260u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2207264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae58));
}
#[inline]
pub fn block_0x0021ae20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2207268u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2207272u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2207276u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2207280u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2207284u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2207288u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2207292u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2207296u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2207300u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2207304u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2207308u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2207312u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2207316u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2207320u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2207320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ae58));
}
#[inline]
pub fn block_0x0021ae58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2207324u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2207328u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2207332u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2207336u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207340u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2207344u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2207348u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2207352u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2207356u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207360u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207364u32;
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
#[inline]
pub fn block_0x0021ae84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2207368u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2207372u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2207376u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2207380u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2207384u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2207388u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2207392u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2207396u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2207400u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2207404u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2207408u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2207472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aef0));
    } else {
        emu.pc = 2207412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aeb4));
    }
}
#[inline(always)]
pub fn block_0x0021aeb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2207416u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2207420u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207424u32);
    emu.apc_no_count(1usize, 2207424u32, 4294897664u32, 2207428u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021aec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2207436u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2207440u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2207444u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2207448u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2207452u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2207456u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2207460u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2207464u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2207468u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207472u32;
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
pub fn block_0x0021aef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2207476u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2207480u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2207484u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2207488u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2207492u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2207496u32);
    emu.apc_no_count(1usize, 2207496u32, 4294959104u32, 2207500u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021af10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2207508u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2207512u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2207516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021aeb4));
}
#[inline(always)]
pub fn block_0x0021af1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2207516u32));
}
#[inline(always)]
pub fn block_0x0021af20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2207524u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2207528u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2207532u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2207536u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2207540u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2207544u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2207548u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2207548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af3c));
}
#[inline(always)]
pub fn block_0x0021af3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2207576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af58));
    } else {
        emu.pc = 2207552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af40));
    }
}
#[inline(always)]
pub fn block_0x0021af40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2207556u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2207560u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2207564u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2207548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af3c));
    } else {
        emu.pc = 2207568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021af50));
    }
}
#[inline(always)]
pub fn block_0x0021af50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2207572u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2207576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af60));
}
#[inline(always)]
pub fn block_0x0021af58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2207580u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2207584u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2207584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021af60));
}
#[inline(always)]
pub fn block_0x0021af60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2207588u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2207592u32);
    emu.apc_no_count(1usize, 2207592u32, 4294897664u32, 2207596u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021af70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2207604u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2207608u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2207612u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2207616u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2207620u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2207624u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2207628u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207632u32;
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
pub fn block_0x0021af90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2207636u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2207640u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2207644u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2207648u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2207672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afb8));
    } else {
        emu.pc = 2207652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021afa4));
    }
}
#[inline(always)]
pub fn block_0x0021afa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2207656u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2207660u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2207664u32);
    emu.apc_no_count(1usize, 2207664u32, 4294897664u32, 2207668u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021afb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2207676u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2207680u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2207684u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2207688u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2207692u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207696u32;
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
#[inline]
pub fn block_0x0021afd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2207700u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2207704u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2207708u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2207712u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2207716u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2207720u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2207724u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2207728u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2207732u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2207804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b03c));
    } else {
        emu.pc = 2207736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aff8));
    }
}
#[inline(always)]
pub fn block_0x0021aff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2207740u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2207744u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2207748u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2207752u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2207756u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2207760u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2207764u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2207768u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2207768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b018));
}
#[inline(always)]
pub fn block_0x0021b018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2207772u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2207796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b034));
    } else {
        emu.pc = 2207776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b020));
    }
}
#[inline(always)]
pub fn block_0x0021b020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2207780u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2207784u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2207788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2207768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b018));
    } else {
        emu.pc = 2207792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b030));
    }
}
#[inline(always)]
pub fn block_0x0021b030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2207796u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2207796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b034));
}
#[inline(always)]
pub fn block_0x0021b034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2208120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b178));
    } else {
        emu.pc = 2207800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b038));
    }
}
#[inline(always)]
pub fn block_0x0021b038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2207816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b048));
    } else {
        emu.pc = 2207804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b03c));
    }
}
#[inline(always)]
pub fn block_0x0021b03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2207808u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2207812u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2207816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b118));
}
#[inline(always)]
pub fn block_0x0021b048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2207820u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2207824u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2207828u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2207828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b054));
}
#[inline(always)]
pub fn block_0x0021b054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2207832u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2207836u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2207840u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2207840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b060));
}
#[inline(always)]
pub fn block_0x0021b060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2207984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0f0));
    } else {
        emu.pc = 2207844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b064));
    }
}
#[inline(always)]
pub fn block_0x0021b064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2207848u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2207852u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2207856u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2207840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b060));
    } else {
        emu.pc = 2207860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b074));
    }
}
#[inline(always)]
pub fn block_0x0021b074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2207864u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2207868u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2207872u32);
    emu.apc_no_count(1usize, 2207872u32, 4294897664u32, 2207876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2207880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2207884u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2207888u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2207892u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2207896u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2207900u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2207900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b09c));
}
#[inline(always)]
pub fn block_0x0021b09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2207904u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2207932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0bc));
    } else {
        emu.pc = 2207908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0a4));
    }
}
#[inline(always)]
pub fn block_0x0021b0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2207912u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2207916u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2207920u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2207924u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2207900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b09c));
    } else {
        emu.pc = 2207928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0b8));
    }
}
#[inline(always)]
pub fn block_0x0021b0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2207932u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2207932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b0bc));
}
#[inline(always)]
pub fn block_0x0021b0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2208140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b18c));
    } else {
        emu.pc = 2207936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0c0));
    }
}
#[inline(always)]
pub fn block_0x0021b0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2208060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b13c));
    } else {
        emu.pc = 2207940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0c4));
    }
}
#[inline(always)]
pub fn block_0x0021b0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2207944u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2207948u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2207952u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2208160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1a0));
    } else {
        emu.pc = 2207956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b0d4));
    }
}
#[inline(always)]
pub fn block_0x0021b0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2207960u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2207964u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2207968u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2207972u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2207976u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2207980u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2207984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2207828u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b054));
}
#[inline(always)]
pub fn block_0x0021b0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2207988u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2207992u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2207996u32);
    emu.apc_no_count(1usize, 2207996u32, 4294897664u32, 2208000u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208008u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966680u32, 2208012u32)?;
    emu.lw_no_count(10usize, 10usize, 4294966684u32, 2208016u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2208020u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2208024u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2208024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b118));
}
#[inline]
pub fn block_0x0021b118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2208028u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2208032u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2208036u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2208040u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2208044u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2208048u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2208052u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2208056u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208060u32;
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
pub fn block_0x0021b13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2207804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b03c));
    } else {
        emu.pc = 2208064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b140));
    }
}
#[inline]
pub fn block_0x0021b140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208068u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966580u32, 2208072u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2208076u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2208080u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2208084u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2208088u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2208092u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2208096u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2208100u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208104u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966588u32, 2208108u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2208112u32);
    emu.apc_no_count(1usize, 2208112u32, 12288u32, 2208116u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208124u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966620u32, 2208128u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2208132u32);
    emu.apc_no_count(1usize, 2208132u32, 40960u32, 2208136u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208144u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966620u32, 2208148u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2208152u32);
    emu.apc_no_count(1usize, 2208152u32, 40960u32, 2208156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208164u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966604u32, 2208168u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2208172u32);
    emu.apc_no_count(1usize, 2208172u32, 40960u32, 2208176u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208184u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2208188u32)?;
    emu.apc_no_count(1usize, 2208188u32, 4294959104u32, 2208192u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2208200u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208204u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208208u32;
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
pub fn block_0x0021b1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2208212u32);
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208216u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 241u32, 2208220u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2208224u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2208228u32);
    emu.sb_no_count(10usize, 11usize, 241u32, 2208232u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2208252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1fc));
    } else {
        emu.pc = 2208236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b1ec));
    }
}
#[inline(always)]
pub fn block_0x0021b1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2208240u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 241u32, 2208244u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2208248u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208252u32;
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
#[inline]
pub fn block_0x0021b1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208256u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966812u32, 2208260u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2208264u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2208268u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2208272u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2208276u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2208280u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2208284u32)?;
    let a = 0u32.wrapping_add(2256896u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208288u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 319u32, 2208292u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2208296u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966820u32, 2208300u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2208304u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2208308u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2208312u32);
    emu.apc_no_count(1usize, 2208312u32, 0u32, 2208316u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2208324u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2208328u32)?;
    emu.adi_no_count(11usize, 12usize, 0u32, 2208332u32);
    emu.sb_no_count(14usize, 2usize, 43u32, 2208336u32);
    emu.adi_no_count(12usize, 2usize, 43u32, 2208340u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2208344u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 672u32, 2208348u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2208352u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966716u32, 2208356u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2208360u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2208364u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2208368u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2208372u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2208376u32);
    emu.lw_no_count(13usize, 13usize, 36u32, 2208380u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2208384u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2208388u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2208392u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2208396u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2208400u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2208404u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021b294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2208408u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2208412u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208416u32;
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
#[inline]
pub fn block_0x0021b2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208420u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2208424u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2208428u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2208432u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2208436u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2208440u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2208444u32)?;
    emu.lbu_no_count(18usize, 10usize, 0u32, 2208448u32);
    emu.lw_no_count(9usize, 12usize, 12u32, 2208452u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208456u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966688u32, 2208460u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2208464u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2208468u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2208472u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021b2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2f8));
    } else {
        emu.pc = 2208476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2dc));
    }
}
#[inline(always)]
pub fn block_0x0021b2dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2208480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2208480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b2e0));
}
#[inline(always)]
pub fn block_0x0021b2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2208484u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2208488u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2208492u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2208496u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208500u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208504u32;
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
pub fn block_0x0021b2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2208536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b318));
    } else {
        emu.pc = 2208508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2fc));
    }
}
#[inline(always)]
pub fn block_0x0021b2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2208512u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2208516u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2208520u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2208524u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2208528u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208532u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208536u32;
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
pub fn block_0x0021b318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2208540u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966705u32, 2208544u32);
    emu.adi_no_count(12usize, 0usize, 88u32, 2208548u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2208552u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2208556u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021b32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2208476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b2dc));
    } else {
        emu.pc = 2208560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b330));
    }
}
#[inline(always)]
pub fn block_0x0021b330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2208564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2208480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b2e0));
}
#[inline]
pub fn block_0x0021b334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208568u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2208572u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2208576u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2208580u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2208584u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2208588u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2208592u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2208596u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2208600u32);
    emu.apc_no_count(1usize, 2208600u32, 4294897664u32, 2208604u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2208612u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2208616u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2208620u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2208624u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2208628u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2208632u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208636u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208640u32;
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
pub fn block_0x0021b380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2208644u32)?;
    emu.apc_no_count(6usize, 2208644u32, 28672u32, 2208648u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208652u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2208656u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2208660u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2208664u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2208668u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2208672u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2208676u32);
    emu.apc_no_count(6usize, 2208676u32, 28672u32, 2208680u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208684u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b3ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2208688u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2208692u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2208696u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2208700u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2208704u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2208708u32);
    emu.apc_no_count(6usize, 2208708u32, 32768u32, 2208712u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208716u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208720u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2208724u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2208728u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2208732u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2208736u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2208740u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966796u32, 2208744u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2208748u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2208752u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2208756u32);
    emu.apc_no_count(1usize, 2208756u32, 8192u32, 2208760u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2208768u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2208772u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2208776u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2208780u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208784u32;
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
pub fn block_0x0021b410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2208788u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2208792u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2208796u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2208800u32)?;
    emu.lw_no_count(11usize, 8usize, 12u32, 2208804u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2208824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b438));
    } else {
        emu.pc = 2208808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b428));
    }
}
#[inline(always)]
pub fn block_0x0021b428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2208812u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2208816u32);
    emu.apc_no_count(1usize, 2208816u32, 4294864896u32, 2208820u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2208828u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2208848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b450));
    } else {
        emu.pc = 2208832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b440));
    }
}
#[inline(always)]
pub fn block_0x0021b440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2208836u32)?;
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2208840u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2208844u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2208864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b460));
    } else {
        emu.pc = 2208848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b450));
    }
}
#[inline(always)]
pub fn block_0x0021b450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2208852u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2208856u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208860u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208864u32;
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
pub fn block_0x0021b460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 24u32, 2208868u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2208872u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2208876u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2208880u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2208884u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2208888u32);
    emu.apc_no_count(6usize, 2208888u32, 4294864896u32, 2208892u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208896u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2208896u32, 4294959104u32, 2208900u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2208904u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2208908u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2208912u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2208916u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2208920u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2208924u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2208928u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2208932u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2208936u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2208940u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2208944u32);
    emu.apc_no_count(1usize, 2208944u32, 4294893568u32, 2208948u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b4b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2208956u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2208960u32);
    emu.apc_no_count(1usize, 2208960u32, 4294864896u32, 2208964u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2208968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b4c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2209024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b500));
    } else {
        emu.pc = 2208972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b4cc));
    }
}
#[inline]
pub fn block_0x0021b4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 10usize, 0u32, 2208976u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2208980u32)?;
    emu.sb_no_count(18usize, 10usize, 8u32, 2208984u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2208988u32);
    emu.sb_no_count(11usize, 8usize, 0u32, 2208992u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2208996u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2209000u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2209004u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2209008u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2209012u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2209016u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2209020u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209024u32;
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
pub fn block_0x0021b500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2209028u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2209032u32);
    emu.apc_no_count(1usize, 2209032u32, 0u32, 2209036u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2209044u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2209048u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2209052u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2209056u32);
    emu.sb_no_count(12usize, 2usize, 7u32, 2209060u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2209064u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2209080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b538));
    } else {
        emu.pc = 2209068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b52c));
    }
}
#[inline(always)]
pub fn block_0x0021b52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2209072u32);
    emu.adi_no_count(2usize, 2usize, 32u32, 2209076u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209080u32;
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
#[inline]
pub fn block_0x0021b538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209084u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966812u32, 2209088u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2209092u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2209096u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2209100u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2209104u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2209108u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2209112u32)?;
    let a = 0u32.wrapping_add(2256896u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209116u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 319u32, 2209120u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2209124u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966820u32, 2209128u32);
    emu.adi_no_count(11usize, 2usize, 7u32, 2209132u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2209136u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2209140u32);
    emu.apc_no_count(1usize, 2209140u32, 0u32, 2209144u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
