pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2203928u32;
pub const PC_MAX: u32 = 2206756u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x0021a118,
        block_0x0021a120,
        block_0x0021a15c,
        block_0x0021a174,
        block_0x0021a1a4,
        block_0x0021a1c0,
        block_0x0021a1c8,
        block_0x0021a1f4,
        block_0x0021a20c,
        block_0x0021a220,
        block_0x0021a230,
        block_0x0021a248,
        block_0x0021a250,
        block_0x0021a270,
        block_0x0021a27c,
        block_0x0021a290,
        block_0x0021a2a4,
        block_0x0021a2b4,
        block_0x0021a2e4,
        block_0x0021a2f0,
        block_0x0021a2f8,
        block_0x0021a300,
        block_0x0021a30c,
        block_0x0021a314,
        block_0x0021a334,
        block_0x0021a360,
        block_0x0021a378,
        block_0x0021a384,
        block_0x0021a3a0,
        block_0x0021a3c0,
        block_0x0021a3d0,
        block_0x0021a3dc,
        block_0x0021a3e0,
        block_0x0021a3f8,
        block_0x0021a400,
        block_0x0021a408,
        block_0x0021a41c,
        block_0x0021a444,
        block_0x0021a450,
        block_0x0021a458,
        block_0x0021a460,
        block_0x0021a474,
        block_0x0021a4a8,
        block_0x0021a4b4,
        block_0x0021a4cc,
        block_0x0021a4e0,
        block_0x0021a4e8,
        block_0x0021a4f0,
        block_0x0021a4f8,
        block_0x0021a504,
        block_0x0021a528,
        block_0x0021a544,
        block_0x0021a5a4,
        block_0x0021a5c8,
        block_0x0021a5f8,
        block_0x0021a608,
        block_0x0021a60c,
        block_0x0021a63c,
        block_0x0021a64c,
        block_0x0021a658,
        block_0x0021a6c8,
        block_0x0021a6fc,
        block_0x0021a708,
        block_0x0021a714,
        block_0x0021a768,
        block_0x0021a774,
        block_0x0021a78c,
        block_0x0021a7ac,
        block_0x0021a7bc,
        block_0x0021a7c0,
        block_0x0021a7e4,
        block_0x0021a7f4,
        block_0x0021a800,
        block_0x0021a810,
        block_0x0021a828,
        block_0x0021a840,
        block_0x0021a844,
        block_0x0021a848,
        block_0x0021a854,
        block_0x0021a858,
        block_0x0021a864,
        block_0x0021a890,
        block_0x0021a8c0,
        block_0x0021a8e4,
        block_0x0021a934,
        block_0x0021a940,
        block_0x0021a94c,
        block_0x0021a99c,
        block_0x0021a9a4,
        block_0x0021a9c4,
        block_0x0021a9e4,
        block_0x0021aa2c,
        block_0x0021aa38,
        block_0x0021aa44,
        block_0x0021aa60,
        block_0x0021aa64,
        block_0x0021aa84,
        block_0x0021aa88,
        block_0x0021aa90,
        block_0x0021aa9c,
        block_0x0021ab04,
        block_0x0021ab50,
        block_0x0021ab68,
        block_0x0021ab74,
        block_0x0021ab78,
        block_0x0021abac,
        block_0x0021abc4,
        block_0x0021abc8,
        block_0x0021abd8,
        block_0x0021abe0,
        block_0x0021ac10,
        block_0x0021ac18,
        block_0x0021ac20,
        block_0x0021ac24,
    ];
    const IDX: [u16; 708usize] = [
        1u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 20u16, 0u16, 21u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 39u16,
        0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16, 0u16,
        48u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 63u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 78u16, 0u16, 0u16, 79u16, 80u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 99u16, 0u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16,
        0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 108u16, 0u16,
        0u16, 0u16, 109u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 0u16, 113u16, 114u16,
    ];
    if pc < 2203928u32 || pc > 2206756u32 {
        return None;
    }
    let word_offset = ((pc - 2203928u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021a118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2203932u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2203936u32;
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
pub fn block_0x0021a120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2203940u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2203944u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2203948u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2203952u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2203956u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2203960u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2203964u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2203968u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2203972u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2203976u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2203980u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2203984u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2203988u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2203992u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2204068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a1a4));
    } else {
        emu.pc = 2203996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a15c));
    }
}
#[inline(always)]
pub fn block_0x0021a15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2204000u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2204004u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2204008u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2204012u32);
    emu.apc_no_count(1usize, 2204012u32, 4294901760u32, 2204016u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2204024u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2204028u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2204032u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2204036u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2204040u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2204044u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2204048u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2204052u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2204056u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2204060u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2204064u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204068u32;
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
pub fn block_0x0021a1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2204072u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2204076u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2204080u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2204084u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2204088u32);
    emu.apc_no_count(1usize, 2204088u32, 4294963200u32, 2204092u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2204100u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2204104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2203996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a15c));
}
#[inline]
pub fn block_0x0021a1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2204108u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2204112u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2204116u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2204120u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2204124u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2204128u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2204132u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2204136u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2204140u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2204144u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2204340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a2b4));
    } else {
        emu.pc = 2204148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a1f4));
    }
}
#[inline(always)]
pub fn block_0x0021a1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2204152u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2204156u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2204160u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2204164u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2204168u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2204172u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2204172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a20c));
}
#[inline(always)]
pub fn block_0x0021a20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2204176u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2204180u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2204184u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2204188u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2204172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a20c));
    } else {
        emu.pc = 2204192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a220));
    }
}
#[inline(always)]
pub fn block_0x0021a220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2204196u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2204200u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2204204u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2204284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a27c));
    } else {
        emu.pc = 2204208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a230));
    }
}
#[inline(always)]
pub fn block_0x0021a230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2204212u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2204216u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2204220u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2204224u32);
    emu.apc_no_count(1usize, 2204224u32, 4294963200u32, 2204228u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2204236u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2204240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a27c));
}
#[inline(always)]
pub fn block_0x0021a250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2204244u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2204248u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2204252u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2204256u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2204260u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2204264u32);
    emu.apc_no_count(1usize, 2204264u32, 4294963200u32, 2204268u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2204276u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2204280u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2204284u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204304u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a290));
}
#[inline(always)]
pub fn block_0x0021a27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2204288u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2204292u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2204296u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2204300u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2204240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a250));
    } else {
        emu.pc = 2204304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a290));
    }
}
#[inline(always)]
pub fn block_0x0021a290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2204308u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2204312u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2204316u32);
    emu.apc_no_count(1usize, 2204316u32, 4294901760u32, 2204320u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2204328u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2204332u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2204336u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2204284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a27c));
    } else {
        emu.pc = 2204340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a2b4));
    }
}
#[inline]
pub fn block_0x0021a2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2204344u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2204348u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2204352u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2204356u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2204360u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2204364u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2204368u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2204372u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2204376u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2204380u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2204384u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204388u32;
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
pub fn block_0x0021a2e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2204392u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2204396u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204400u32;
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
pub fn block_0x0021a2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2204404u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204408u32;
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
pub fn block_0x0021a2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2204408u32, 4096u32, 2204412u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2204416u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2204420u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2204424u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2204436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a314));
    } else {
        emu.pc = 2204428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a30c));
    }
}
#[inline(always)]
pub fn block_0x0021a30c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2204428u32, 4096u32, 2204432u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2204440u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2204444u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2204448u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2204452u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2204456u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2204460u32)?;
    emu.apc_no_count(1usize, 2204460u32, 0u32, 2204464u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2204472u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2204476u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2204480u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2204484u32);
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2204488u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 216u32, 2204492u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2204496u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2204500u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2204504u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2204508u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2204640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3e0));
    } else {
        emu.pc = 2204512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a360));
    }
}
#[inline(always)]
pub fn block_0x0021a360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2204516u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 220u32, 2204520u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2204524u32);
    emu.sb_no_count(9usize, 10usize, 216u32, 2204528u32);
    emu.sw_no_count(0usize, 18usize, 220u32, 2204532u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2204640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3e0));
    } else {
        emu.pc = 2204536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a378));
    }
}
#[inline(always)]
pub fn block_0x0021a378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2204540u32);
    emu.apc_no_count(1usize, 2204540u32, 4096u32, 2204544u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204548u32;
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
#[inline(always)]
pub fn block_0x0021a384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2204552u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2204556u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204560u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965880u32, 2204564u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2204568u32);
    emu.apc_no_count(1usize, 2204568u32, 0u32, 2204572u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2204580u32);
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204584u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 220u32, 2204588u32)?;
    emu.sb_no_count(9usize, 11usize, 216u32, 2204592u32);
    emu.sw_no_count(19usize, 18usize, 220u32, 2204596u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2204600u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2204604u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2204664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3f8));
    } else {
        emu.pc = 2204608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3c0));
    }
}
#[inline(always)]
pub fn block_0x0021a3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2204612u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2204616u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2204620u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2204664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3f8));
    } else {
        emu.pc = 2204624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a3d0));
    }
}
#[inline(always)]
pub fn block_0x0021a3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2204628u32);
    emu.apc_no_count(1usize, 2204628u32, 4096u32, 2204632u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2204640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a3f8));
}
#[inline(always)]
pub fn block_0x0021a3e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2204644u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965920u32, 2204648u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2204652u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2204656u32);
    emu.apc_no_count(1usize, 2204656u32, 0u32, 2204660u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2204664u32, 4096u32, 2204668u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204672u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2204672u32, 4096u32, 2204676u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2204680u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2204684u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2204688u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2204692u32);
    emu.apc_no_count(1usize, 2204692u32, 4096u32, 2204696u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2204704u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2204708u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2204712u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2204716u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2204720u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2204724u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2204728u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2204732u32);
    emu.apc_no_count(1usize, 2204732u32, 4096u32, 2204736u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2204744u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2204748u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2204876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4cc));
    } else {
        emu.pc = 2204752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a450));
    }
}
#[inline(always)]
pub fn block_0x0021a450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2204756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2204852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4b4));
    } else {
        emu.pc = 2204760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a458));
    }
}
#[inline(always)]
pub fn block_0x0021a458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2204764u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2204920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4f8));
    } else {
        emu.pc = 2204768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a460));
    }
}
#[inline(always)]
pub fn block_0x0021a460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204772u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 240u32, 2204776u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2204780u32);
    emu.sb_no_count(10usize, 11usize, 240u32, 2204784u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2204920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a4f8));
    } else {
        emu.pc = 2204788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a474));
    }
}
#[inline]
pub fn block_0x0021a474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204792u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966040u32, 2204796u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2204800u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2204804u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2204808u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2204812u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2204816u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2204820u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2204824u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2204828u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2204832u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2204836u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2204840u32;
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
pub fn block_0x0021a4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2204844u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2204848u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2204852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a4f0));
}
#[inline(always)]
pub fn block_0x0021a4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2204856u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2204860u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2204864u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2204868u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2204872u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2204876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2204896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a4e0));
}
#[inline(always)]
pub fn block_0x0021a4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2204880u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2204884u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2204888u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2204892u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2204896u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2204896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a4e0));
}
#[inline(always)]
pub fn block_0x0021a4e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2204896u32, 4096u32, 2204900u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2204908u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2204912u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2204912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a4f0));
}
#[inline(always)]
pub fn block_0x0021a4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2204912u32, 4294963200u32, 2204916u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2204924u32);
    emu.apc_no_count(6usize, 2204924u32, 0u32, 2204928u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2204932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2204936u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2204940u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2204944u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2204948u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2204952u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2204956u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2204960u32);
    emu.apc_no_count(1usize, 2204960u32, 0u32, 2204964u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2204968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2204972u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2204976u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2204980u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2204984u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2204988u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2204992u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2205128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a5c8));
    } else {
        emu.pc = 2204996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a544));
    }
}
#[inline]
pub fn block_0x0021a544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2205000u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2205004u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2205008u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2205012u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2205016u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2205020u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2205024u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2205028u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2205032u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2205036u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2205040u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2205044u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2205048u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2205052u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2205056u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2205060u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2205064u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2205068u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205072u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966048u32, 2205076u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2205080u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2205084u32);
    emu.apc_no_count(1usize, 2205084u32, 28672u32, 2205088u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2205096u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2205100u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2205104u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2205108u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2205112u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2205116u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2205120u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2205124u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2205128u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2205128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a5c8));
}
#[inline]
pub fn block_0x0021a5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2205132u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2205136u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2205140u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2205144u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2205148u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2205152u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2205156u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2205160u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2205164u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2205168u32)?;
    emu.apc_no_count(1usize, 2205168u32, 4294897664u32, 2205172u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205176u32;
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
pub fn block_0x0021a5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2205180u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2205184u32);
    emu.apc_no_count(1usize, 2205184u32, 4294868992u32, 2205188u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a63c));
    } else {
        emu.pc = 2205196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a60c));
    }
}
#[inline]
pub fn block_0x0021a60c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2205200u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2205204u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2205208u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205212u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966072u32, 2205216u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2205220u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2205224u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2205228u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2205232u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2205236u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2205240u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205244u32;
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
pub fn block_0x0021a63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2205248u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2205252u32);
    emu.apc_no_count(1usize, 2205252u32, 4096u32, 2205256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2205264u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2205268u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2205436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a6fc));
    } else {
        emu.pc = 2205272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a658));
    }
}
#[inline(never)]
pub fn block_0x0021a658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2205276u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2205280u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2205284u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2205288u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2205292u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2205296u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2205300u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2205304u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2205308u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2205312u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2205316u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2205320u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2205324u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2205328u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2205332u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2205336u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2205340u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2205344u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2205348u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2205352u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2205356u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205360u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966048u32, 2205364u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2205368u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2205372u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2205376u32);
    emu.apc_no_count(1usize, 2205376u32, 28672u32, 2205380u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2205388u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2205392u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2205396u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2205400u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2205404u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2205408u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2205412u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2205416u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2205420u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2205424u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2205428u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2205432u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2205436u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2205436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a6fc));
}
#[inline(always)]
pub fn block_0x0021a6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205440u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966072u32, 2205444u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205448u32;
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
pub fn block_0x0021a708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2205452u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2205456u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2205556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a774));
    } else {
        emu.pc = 2205460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a714));
    }
}
#[inline]
pub fn block_0x0021a714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2205464u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2205468u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2205472u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2205476u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2205480u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2205484u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2205488u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2205492u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2205496u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2205500u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2205504u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2205508u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2205512u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2205516u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2205520u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2205524u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2205528u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2205532u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2205536u32);
    emu.apc_no_count(1usize, 2205536u32, 28672u32, 2205540u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2205548u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2205552u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205556u32;
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
pub fn block_0x0021a774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2205560u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2205564u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2205568u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2205572u32);
    emu.apc_no_count(6usize, 2205572u32, 32768u32, 2205576u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2205580u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2205584u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2205588u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2205592u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2205596u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2205600u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2205604u32)?;
    emu.apc_no_count(1usize, 2205604u32, 4294897664u32, 2205608u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2205616u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2205620u32);
    emu.apc_no_count(1usize, 2205620u32, 4294864896u32, 2205624u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7e4));
    } else {
        emu.pc = 2205632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a7c0));
    }
}
#[inline]
pub fn block_0x0021a7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205636u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966088u32, 2205640u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2205644u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2205648u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2205652u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2205656u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2205660u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2205664u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205668u32;
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
pub fn block_0x0021a7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2205672u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2205676u32);
    emu.apc_no_count(1usize, 2205676u32, 4096u32, 2205680u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966088u32, 2205692u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205696u32;
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
pub fn block_0x0021a800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2205700u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2205704u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2205708u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205712u32;
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
pub fn block_0x0021a810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2205716u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2205720u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2205724u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2205728u32);
    emu.apc_no_count(6usize, 2205728u32, 32768u32, 2205732u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2205736u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021a828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2205740u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2205744u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2205748u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2205752u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2205756u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2205780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a854));
    } else {
        emu.pc = 2205760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a840));
    }
}
#[inline(always)]
pub fn block_0x0021a840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2205840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a890));
    } else {
        emu.pc = 2205764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a844));
    }
}
#[inline(always)]
pub fn block_0x0021a844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a890));
    } else {
        emu.pc = 2205768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a848));
    }
}
#[inline(always)]
pub fn block_0x0021a848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2205772u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2205776u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2205780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2205796u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a864));
}
#[inline(always)]
pub fn block_0x0021a854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2205840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a890));
    } else {
        emu.pc = 2205784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a858));
    }
}
#[inline(always)]
pub fn block_0x0021a858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2205788u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2205792u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2205796u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2205796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a864));
}
#[inline]
pub fn block_0x0021a864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2205800u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2205804u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2205808u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2205812u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2205816u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2205820u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205824u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966104u32, 2205828u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2205832u32);
    emu.apc_no_count(1usize, 2205832u32, 0u32, 2205836u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2205844u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2205848u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2205852u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2205856u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2205860u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2205864u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2205868u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2205872u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966132u32, 2205876u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2205880u32);
    emu.apc_no_count(1usize, 2205880u32, 0u32, 2205884u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2205888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021a8c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2205892u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2205896u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2205900u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2205904u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2205908u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2205912u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2205916u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2205920u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2205924u32;
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
#[inline]
pub fn block_0x0021a8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2205928u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2205932u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2205936u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2205940u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2205944u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2205948u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2205952u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2205956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2205960u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2205964u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2205968u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2205972u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2205976u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2205980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2205984u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2205988u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2205992u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2205996u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2206000u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a940));
    } else {
        emu.pc = 2206004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a934));
    }
}
#[inline(always)]
pub fn block_0x0021a934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2206008u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2206012u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2206016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a9a4));
}
#[inline(always)]
pub fn block_0x0021a940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2206020u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2206024u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2206028u32;
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
#[inline]
pub fn block_0x0021a94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2206032u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2206036u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2206040u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2206044u32)?;
    let a = 0u32.wrapping_add(4151074816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206048u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965826u32, 2206052u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2206056u32);
    let a = 0u32.wrapping_add(228253696u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 203u32, 2206064u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2206068u32);
    let a = 0u32.wrapping_add(1618087936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206072u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1443u32, 2206076u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2206080u32);
    let a = 0u32.wrapping_add(4257644544u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 861u32, 2206088u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2206092u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2206096u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2206100u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2206104u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a9c4));
    } else {
        emu.pc = 2206108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021a99c));
    }
}
#[inline(always)]
pub fn block_0x0021a99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2206112u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2206116u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2206116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021a9a4));
}
#[inline(always)]
pub fn block_0x0021a9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2206120u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2206124u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2206128u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2206132u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2206136u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2206140u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2206144u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206148u32;
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
pub fn block_0x0021a9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2206152u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966160u32, 2206156u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2206160u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2206164u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2206168u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2206172u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2206176u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206180u32;
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
pub fn block_0x0021a9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2206184u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2206188u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2206192u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2206196u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2206200u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2206204u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2206208u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2206212u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2206216u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2206220u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2206224u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2206228u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2206232u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2206236u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2206240u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2206244u32);
    emu.apc_no_count(1usize, 2206244u32, 4096u32, 2206248u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aa2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2206256u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2206260u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2206344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa88));
    } else {
        emu.pc = 2206264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa38));
    }
}
#[inline(always)]
pub fn block_0x0021aa38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2206268u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 228u32, 2206272u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2206756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ac24));
    } else {
        emu.pc = 2206276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa44));
    }
}
#[inline(always)]
pub fn block_0x0021aa44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 228u32, 2206280u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2206284u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2206288u32);
    emu.sw_no_count(11usize, 10usize, 228u32, 2206292u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2206296u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2206300u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2206580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab74));
    } else {
        emu.pc = 2206304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa60));
    }
}
#[inline(always)]
pub fn block_0x0021aa60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2206308u32;
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
pub fn block_0x0021aa64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2206312u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2206316u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2206320u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2206324u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2206328u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2206332u32);
    emu.apc_no_count(1usize, 2206332u32, 0u32, 2206336u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021aa84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2206344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021abac));
}
#[inline(always)]
pub fn block_0x0021aa88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2206348u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2206468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ab04));
    } else {
        emu.pc = 2206352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021aa90));
    }
}
#[inline(always)]
pub fn block_0x0021aa90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2206356u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2206360u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2206364u32;
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
#[inline(never)]
pub fn block_0x0021aa9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2206368u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2206372u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206376u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1636u32, 2206380u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2206384u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2206388u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 940u32, 2206392u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2206396u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966288u32, 2206400u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2206404u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2206408u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2206412u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2206416u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2206420u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2206424u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2206428u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2206432u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2206436u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2206440u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2206444u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2206448u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2206452u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2206456u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2206460u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2206464u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2206468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ab50));
}
#[inline]
pub fn block_0x0021ab04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2206472u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2206476u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1636u32, 2206480u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2206484u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2206488u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966568u32, 2206492u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2206496u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966200u32, 2206500u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2206504u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2206508u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2206512u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2206516u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2206520u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2206524u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2206528u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2206532u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2206536u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2206540u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2206544u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2206544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ab50));
}
#[inline(always)]
pub fn block_0x0021ab50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2206548u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2206552u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2206556u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2206560u32);
    emu.apc_no_count(1usize, 2206560u32, 0u32, 2206564u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ab68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2206572u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2206576u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2206580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac18));
}
#[inline(always)]
pub fn block_0x0021ab74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2206584u32;
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
#[inline]
pub fn block_0x0021ab78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2206588u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 228u32, 2206592u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2206596u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2206600u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2206604u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2206608u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2206612u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2206616u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2206620u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2206624u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2206628u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2206632u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2206636u32;
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
pub fn block_0x0021abac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2206640u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 228u32, 2206644u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2206648u32);
    emu.sw_no_count(11usize, 10usize, 228u32, 2206652u32)?;
    emu.apc_no_count(1usize, 2206652u32, 4096u32, 2206656u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021abc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2206680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021abd8));
    } else {
        emu.pc = 2206664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021abc8));
    }
}
#[inline(always)]
pub fn block_0x0021abc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2206668u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2206672u32);
    emu.apc_no_count(1usize, 2206672u32, 0u32, 2206676u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021abd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2206684u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966360u32, 2206688u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2206688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021abe0));
}
#[inline]
pub fn block_0x0021abe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2206692u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2206696u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2206700u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2206704u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2206708u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2206712u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2206716u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2206720u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2206724u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2206728u32);
    emu.apc_no_count(1usize, 2206728u32, 0u32, 2206732u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ac10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2206740u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2206744u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2206744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ac18));
}
#[inline(always)]
pub fn block_0x0021ac18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2206744u32, 4294963200u32, 2206748u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2206752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ac20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2206752u32));
}
#[inline(always)]
pub fn block_0x0021ac24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2206760u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966508u32, 2206764u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2206768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2206688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021abe0));
}
