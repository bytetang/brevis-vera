pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2124284u32;
pub const PC_MAX: u32 = 2127068u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x002069fc,
        block_0x00206a30,
        block_0x00206a3c,
        block_0x00206a58,
        block_0x00206a74,
        block_0x00206a90,
        block_0x00206aac,
        block_0x00206ac8,
        block_0x00206ae4,
        block_0x00206b00,
        block_0x00206b10,
        block_0x00206b18,
        block_0x00206b20,
        block_0x00206b28,
        block_0x00206b30,
        block_0x00206b6c,
        block_0x00206b7c,
        block_0x00206b84,
        block_0x00206b8c,
        block_0x00206b94,
        block_0x00206b9c,
        block_0x00206bb8,
        block_0x00206bbc,
        block_0x00206be4,
        block_0x00206bf0,
        block_0x00206bf8,
        block_0x00206c14,
        block_0x00206c30,
        block_0x00206c58,
        block_0x00206c64,
        block_0x00206c74,
        block_0x00206c7c,
        block_0x00206c84,
        block_0x00206c8c,
        block_0x00206c94,
        block_0x00206cb4,
        block_0x00206cc0,
        block_0x00206cdc,
        block_0x00206ce4,
        block_0x00206cec,
        block_0x00206d08,
        block_0x00206d10,
        block_0x00206d14,
        block_0x00206d30,
        block_0x00206d64,
        block_0x00206d70,
        block_0x00206d94,
        block_0x00206da0,
        block_0x00206dc0,
        block_0x00206dcc,
        block_0x00206e34,
        block_0x00206e40,
        block_0x00206e50,
        block_0x00206e80,
        block_0x00206e8c,
        block_0x00206ea0,
        block_0x00206eb0,
        block_0x00206ee0,
        block_0x00206eec,
        block_0x00206f00,
        block_0x00206f0c,
        block_0x00206f18,
        block_0x00206f20,
        block_0x00206f28,
        block_0x00206f30,
        block_0x00206f38,
        block_0x00206f48,
        block_0x00206f54,
        block_0x00206f64,
        block_0x00206f6c,
        block_0x00206f70,
        block_0x00206fa4,
        block_0x00207014,
        block_0x00207024,
        block_0x00207034,
        block_0x002070e4,
        block_0x002070f4,
        block_0x00207100,
        block_0x00207150,
        block_0x00207160,
        block_0x002071b4,
        block_0x002071c8,
        block_0x002071e0,
        block_0x002071f4,
        block_0x00207208,
        block_0x0020721c,
        block_0x00207230,
        block_0x00207240,
        block_0x00207278,
        block_0x002072cc,
        block_0x00207320,
        block_0x0020733c,
        block_0x00207358,
        block_0x00207370,
        block_0x00207388,
        block_0x002073b4,
        block_0x002073c4,
        block_0x002073d4,
        block_0x002073dc,
        block_0x002073e4,
        block_0x00207404,
        block_0x0020742c,
        block_0x00207454,
        block_0x0020746c,
        block_0x00207488,
        block_0x002074a8,
        block_0x002074d0,
        block_0x002074dc,
    ];
    const IDX: [u16; 697usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 13u16, 0u16, 14u16, 0u16,
        15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 18u16, 0u16, 19u16, 0u16,
        20u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16, 26u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 33u16, 0u16, 34u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16,
        50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 61u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 65u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16,
        0u16, 70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16,
        0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16,
        88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 108u16,
    ];
    if pc < 2124284u32 || pc > 2127068u32 {
        return None;
    }
    let word_offset = ((pc - 2124284u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x002069fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2124288u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2124292u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124296u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1320u32, 2124300u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2124304u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966756u32, 2124308u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2124312u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1304u32, 2124316u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2124320u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2124324u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2124328u32);
    emu.apc_no_count(1usize, 2124328u32, 114688u32, 2124332u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965800u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2124340u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124344u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124348u32;
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
pub fn block_0x00206a3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124352u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1227u32, 2124356u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2124360u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124364u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124368u32);
    emu.apc_no_count(6usize, 2124368u32, 114688u32, 2124372u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124376u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124380u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1147u32, 2124384u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2124388u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124392u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124396u32);
    emu.apc_no_count(6usize, 2124396u32, 114688u32, 2124400u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124404u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965632u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124408u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1254u32, 2124412u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2124416u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124420u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124424u32);
    emu.apc_no_count(6usize, 2124424u32, 114688u32, 2124428u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124432u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124436u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1018u32, 2124440u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2124444u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124448u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124452u32);
    emu.apc_no_count(6usize, 2124452u32, 114688u32, 2124456u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124460u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124464u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1248u32, 2124468u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2124472u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124476u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124480u32);
    emu.apc_no_count(6usize, 2124480u32, 114688u32, 2124484u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124488u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124492u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966764u32, 2124496u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2124500u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124504u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124508u32);
    emu.apc_no_count(6usize, 2124508u32, 114688u32, 2124512u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124516u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124520u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1278u32, 2124524u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2124528u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124532u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124536u32);
    emu.apc_no_count(6usize, 2124536u32, 114688u32, 2124540u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124544u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2124548u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2124552u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2124556u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b20));
    } else {
        emu.pc = 2124560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b10));
    }
}
#[inline(always)]
pub fn block_0x00206b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2124564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b28));
    } else {
        emu.pc = 2124568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b18));
    }
}
#[inline(always)]
pub fn block_0x00206b18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124568u32, 90112u32, 2124572u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124576u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124576u32, 90112u32, 2124580u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124584u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00206b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124584u32, 90112u32, 2124588u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124592u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2124596u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2124600u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2124604u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965324u32, 2124608u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2124612u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965492u32, 2124616u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2124620u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2124624u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2124628u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2124632u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2124636u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2124640u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2124644u32);
    emu.apc_no_count(6usize, 2124644u32, 114688u32, 2124648u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124652u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2124656u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2124660u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2124664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b8c));
    } else {
        emu.pc = 2124668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b7c));
    }
}
#[inline(always)]
pub fn block_0x00206b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2124672u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b94));
    } else {
        emu.pc = 2124676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b84));
    }
}
#[inline(always)]
pub fn block_0x00206b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124676u32, 90112u32, 2124680u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124684u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124684u32, 90112u32, 2124688u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124692u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124692u32, 90112u32, 2124696u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124700u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2124704u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2124708u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2124712u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2124716u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2124720u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2124724u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bf0));
    } else {
        emu.pc = 2124728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bb8));
    }
}
#[inline(always)]
pub fn block_0x00206bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2124820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c14));
    } else {
        emu.pc = 2124732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bbc));
    }
}
#[inline]
pub fn block_0x00206bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2124736u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2124740u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124744u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 620u32, 2124748u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124752u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 552u32, 2124756u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2124760u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2124764u32);
    emu.apc_no_count(1usize, 2124764u32, 114688u32, 2124768u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2124776u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124780u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124784u32;
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
pub fn block_0x00206bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2124788u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2124848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c30));
    } else {
        emu.pc = 2124792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bf8));
    }
}
#[inline(always)]
pub fn block_0x00206bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124796u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 673u32, 2124800u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2124804u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124808u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124812u32);
    emu.apc_no_count(6usize, 2124812u32, 110592u32, 2124816u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124820u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124824u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 632u32, 2124828u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2124832u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2124836u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124840u32);
    emu.apc_no_count(6usize, 2124840u32, 110592u32, 2124844u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124848u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2124852u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2124856u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124860u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 708u32, 2124864u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2124868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 692u32, 2124872u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2124876u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2124880u32);
    emu.apc_no_count(1usize, 2124880u32, 114688u32, 2124884u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2124892u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2124896u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124900u32;
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
pub fn block_0x00206c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2124904u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2124908u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2124912u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c84));
    } else {
        emu.pc = 2124916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c74));
    }
}
#[inline(always)]
pub fn block_0x00206c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2124920u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2124940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c8c));
    } else {
        emu.pc = 2124924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c7c));
    }
}
#[inline(always)]
pub fn block_0x00206c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124924u32, 90112u32, 2124928u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124932u32, 90112u32, 2124936u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124940u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2124940u32, 90112u32, 2124944u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124948u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2124952u32)?;
    emu.lw_no_count(10usize, 12usize, 4u32, 2124956u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2124960u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2124964u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2124968u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2124972u32);
    emu.apc_no_count(6usize, 2124972u32, 114688u32, 2124976u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124980u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2124984u32)?;
    emu.apc_no_count(6usize, 2124984u32, 69632u32, 2124988u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2124992u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2124996u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2125000u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2125004u32)?;
    emu.lw_no_count(14usize, 12usize, 0u32, 2125008u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2125012u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2125016u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2125064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d08));
    } else {
        emu.pc = 2125020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206cdc));
    }
}
#[inline(always)]
pub fn block_0x00206cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2125024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2125072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d10));
    } else {
        emu.pc = 2125028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ce4));
    }
}
#[inline(always)]
pub fn block_0x00206ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2125032u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2125104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d30));
    } else {
        emu.pc = 2125036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206cec));
    }
}
#[inline(always)]
pub fn block_0x00206cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125040u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 632u32, 2125044u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2125048u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125052u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125056u32);
    emu.apc_no_count(6usize, 2125056u32, 110592u32, 2125060u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125064u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 4294967294u32, 2125068u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2125028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ce4));
    } else {
        emu.pc = 2125072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d10));
    }
}
#[inline(always)]
pub fn block_0x00206d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2125168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d70));
    } else {
        emu.pc = 2125076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206d14));
    }
}
#[inline(always)]
pub fn block_0x00206d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 524u32, 2125084u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2125088u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2125092u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125096u32);
    emu.apc_no_count(6usize, 2125096u32, 110592u32, 2125100u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125104u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2125108u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2125112u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125116u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 660u32, 2125120u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2125124u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 670u32, 2125128u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2125132u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 644u32, 2125136u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2125140u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2125144u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2125148u32);
    emu.apc_no_count(1usize, 2125148u32, 110592u32, 2125152u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125160u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125164u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125168u32;
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
pub fn block_0x00206d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 4u32, 2125172u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 620u32, 2125180u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125184u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 552u32, 2125188u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2125192u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2125196u32);
    emu.apc_no_count(1usize, 2125196u32, 114688u32, 2125200u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125208u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125212u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125216u32;
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
pub fn block_0x00206da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2125220u32)?;
    emu.lw_no_count(10usize, 12usize, 0u32, 2125224u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2125228u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2125232u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2125236u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2125240u32);
    emu.apc_no_count(6usize, 2125240u32, 114688u32, 2125244u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125248u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2125252u32)?;
    emu.apc_no_count(6usize, 2125252u32, 81920u32, 2125256u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125260u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00206dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2125264u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2125268u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2125272u32);
    emu.lw_no_count(15usize, 10usize, 0u32, 2125276u32)?;
    emu.adi_no_count(10usize, 15usize, 4u32, 2125280u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2125284u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2125288u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1844u32, 2125292u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2125296u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2125300u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125304u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1860u32, 2125308u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2125312u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1869u32, 2125316u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2125320u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1828u32, 2125324u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2125328u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1880u32, 2125332u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2125336u32);
    emu.adi_no_count(14usize, 0usize, 11u32, 2125340u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2125344u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2125348u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2125352u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2125356u32);
    emu.apc_no_count(1usize, 2125356u32, 110592u32, 2125360u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2125368u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2125372u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125376u32;
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
pub fn block_0x00206e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2125380u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2125384u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2125388u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2125452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e8c));
    } else {
        emu.pc = 2125392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e50));
    }
}
#[inline]
pub fn block_0x00206e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2125396u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2125400u32)?;
    emu.adi_no_count(12usize, 12usize, 4u32, 2125404u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2125408u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125412u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 592u32, 2125416u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125420u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1892u32, 2125424u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2125428u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2125432u32);
    emu.apc_no_count(1usize, 2125432u32, 114688u32, 2125436u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125444u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125448u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125452u32;
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
pub fn block_0x00206e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125456u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 612u32, 2125460u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2125464u32);
    emu.apc_no_count(6usize, 2125464u32, 110592u32, 2125468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125472u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2125476u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2125480u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2125484u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2125548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206eec));
    } else {
        emu.pc = 2125488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206eb0));
    }
}
#[inline]
pub fn block_0x00206eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2125492u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2125496u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2125500u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2125504u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125508u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 592u32, 2125512u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2125516u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 824u32, 2125520u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2125524u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2125528u32);
    emu.apc_no_count(1usize, 2125528u32, 114688u32, 2125532u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125536u32;
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
pub fn block_0x00206ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2125540u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2125544u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125548u32;
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
pub fn block_0x00206eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125552u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 612u32, 2125556u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2125560u32);
    emu.apc_no_count(6usize, 2125560u32, 110592u32, 2125564u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125568u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2125572u32)?;
    emu.apc_no_count(6usize, 2125572u32, 90112u32, 2125576u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125580u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2125584u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2125588u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2125608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f28));
    } else {
        emu.pc = 2125592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f18));
    }
}
#[inline(always)]
pub fn block_0x00206f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2125596u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2125616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f30));
    } else {
        emu.pc = 2125600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f20));
    }
}
#[inline(always)]
pub fn block_0x00206f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2125600u32, 90112u32, 2125604u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125608u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2125608u32, 90112u32, 2125612u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125616u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2125616u32, 90112u32, 2125620u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125624u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2125628u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2125632u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2125636u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2125676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f6c));
    } else {
        emu.pc = 2125640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f48));
    }
}
#[inline(always)]
pub fn block_0x00206f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2125644u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2125648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2125676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f6c));
    } else {
        emu.pc = 2125652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f54));
    }
}
#[inline(always)]
pub fn block_0x00206f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2125656u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2125660u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2125664u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2125676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f6c));
    } else {
        emu.pc = 2125668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f64));
    }
}
#[inline(always)]
pub fn block_0x00206f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2125672u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2125676u32;
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
pub fn block_0x00206f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125680u32;
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
pub fn block_0x00206f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2125684u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2125688u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2125692u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2125696u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2125700u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2125704u32)?;
    emu.adi_no_count(18usize, 13usize, 0u32, 2125708u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2125712u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2125716u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2125720u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2125724u32);
    emu.apc_no_count(1usize, 2125724u32, 28672u32, 2125728u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125732u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00206fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2125736u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2125740u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2125744u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2125748u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2125752u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2125756u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2125760u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2125764u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2125768u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2125772u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2125776u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2125780u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2125784u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2125788u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2125792u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2125796u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2125800u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2125804u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2125808u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2125812u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2125816u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2125820u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2125824u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2125828u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2125832u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2125836u32);
    emu.apc_no_count(1usize, 2125836u32, 32768u32, 2125840u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2125848u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2125852u32);
    emu.apc_no_count(1usize, 2125852u32, 16384u32, 2125856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2125864u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2125868u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2125872u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207404));
    } else {
        emu.pc = 2125876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207034));
    }
}
#[inline(never)]
pub fn block_0x00207034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 44u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2125880u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2125884u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2125888u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2125892u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2125896u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2125900u32)?;
    emu.sw_no_count(12usize, 2usize, 64u32, 2125904u32)?;
    emu.sw_no_count(13usize, 2usize, 68u32, 2125908u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2125912u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2125916u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2125920u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2125924u32)?;
    emu.sw_no_count(10usize, 2usize, 40u32, 2125928u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2125932u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2125936u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2125940u32)?;
    emu.lw_no_count(10usize, 18usize, 48u32, 2125944u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2125948u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2125952u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2125956u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2125960u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2125964u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2125968u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2125972u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2125976u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2125980u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2125984u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2125988u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2125992u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2125996u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2126000u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2126004u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2126008u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2126012u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2126016u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2126020u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2126024u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2126028u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2126032u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2126036u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2126040u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2126044u32);
    emu.apc_no_count(1usize, 2126044u32, 32768u32, 2126048u32);
    emu.add_memory_rw_events(44usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002070e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2126056u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2126060u32);
    emu.apc_no_count(1usize, 2126060u32, 16384u32, 2126064u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126068u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002070f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2126072u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2126076u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207404));
    } else {
        emu.pc = 2126080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207100));
    }
}
#[inline]
pub fn block_0x00207100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2126084u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2126088u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2126092u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2126096u32)?;
    emu.sw_no_count(10usize, 2usize, 88u32, 2126100u32)?;
    emu.sw_no_count(11usize, 2usize, 92u32, 2126104u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2126108u32)?;
    emu.sw_no_count(13usize, 2usize, 100u32, 2126112u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2126116u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2126120u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2126124u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2126128u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2126132u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2126136u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2126140u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2126144u32)?;
    emu.adi_no_count(10usize, 2usize, 460u32, 2126148u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2126152u32);
    emu.apc_no_count(1usize, 2126152u32, 28672u32, 2126156u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 492u32, 2126164u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2126168u32);
    emu.sb_no_count(10usize, 2usize, 268u32, 2126172u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2126892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020742c));
    } else {
        emu.pc = 2126176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207160));
    }
}
#[inline]
pub fn block_0x00207160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2126180u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2126184u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2126188u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2126192u32)?;
    emu.sw_no_count(10usize, 2usize, 120u32, 2126196u32)?;
    emu.sw_no_count(11usize, 2usize, 124u32, 2126200u32)?;
    emu.sw_no_count(12usize, 2usize, 128u32, 2126204u32)?;
    emu.sw_no_count(13usize, 2usize, 132u32, 2126208u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2126212u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2126216u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2126220u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2126224u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2126228u32)?;
    emu.sw_no_count(11usize, 2usize, 108u32, 2126232u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2126236u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2126240u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2126244u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2126248u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2126252u32);
    emu.apc_no_count(1usize, 2126252u32, 28672u32, 2126256u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(928u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2126264u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2126268u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2126272u32);
    emu.apc_no_count(1usize, 2126272u32, 28672u32, 2126276u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126284u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1924u32, 2126288u32);
    emu.adi_no_count(10usize, 2usize, 460u32, 2126292u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2126296u32);
    emu.apc_no_count(1usize, 2126296u32, 12288u32, 2126300u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 268u32, 2126308u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2126312u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2126316u32);
    emu.apc_no_count(1usize, 2126316u32, 4294955008u32, 2126320u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2126328u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2126332u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2126336u32);
    emu.apc_no_count(1usize, 2126336u32, 12288u32, 2126340u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 364u32, 2126348u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2126352u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2126356u32);
    emu.apc_no_count(1usize, 2126356u32, 4294955008u32, 2126360u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020721c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2126368u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126372u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2126376u32);
    emu.apc_no_count(1usize, 2126376u32, 4294959104u32, 2126380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126384u32;
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
pub fn block_0x00207230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 524u32, 2126388u32);
    emu.adi_no_count(10usize, 2usize, 200u32, 2126392u32);
    emu.apc_no_count(1usize, 2126392u32, 45056u32, 2126396u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 604u32, 2126404u32)?;
    emu.sw_no_count(0usize, 2usize, 608u32, 2126408u32)?;
    emu.sw_no_count(0usize, 2usize, 612u32, 2126412u32)?;
    emu.sw_no_count(0usize, 2usize, 616u32, 2126416u32)?;
    emu.lbu_no_count(13usize, 2usize, 232u32, 2126420u32);
    emu.sw_no_count(0usize, 2usize, 588u32, 2126424u32)?;
    emu.sw_no_count(0usize, 2usize, 592u32, 2126428u32)?;
    emu.sw_no_count(0usize, 2usize, 596u32, 2126432u32)?;
    emu.sw_no_count(0usize, 2usize, 600u32, 2126436u32)?;
    emu.adi_no_count(10usize, 2usize, 556u32, 2126440u32);
    emu.adi_no_count(11usize, 2usize, 588u32, 2126444u32);
    emu.adi_no_count(12usize, 2usize, 200u32, 2126448u32);
    emu.apc_no_count(1usize, 2126448u32, 45056u32, 2126452u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2126460u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2126464u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2126468u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2126472u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2126476u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2126480u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2126484u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2126488u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2126492u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2126496u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2126500u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2126504u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2126508u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2126512u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2126516u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2126520u32)?;
    emu.adi_no_count(10usize, 2usize, 268u32, 2126524u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126528u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2126532u32);
    emu.apc_no_count(1usize, 2126532u32, 16384u32, 2126536u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002072cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 508u32, 2126544u32)?;
    emu.lw_no_count(11usize, 2usize, 512u32, 2126548u32)?;
    emu.lw_no_count(12usize, 2usize, 516u32, 2126552u32)?;
    emu.lw_no_count(13usize, 2usize, 520u32, 2126556u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2126560u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2126564u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2126568u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2126572u32)?;
    emu.lw_no_count(10usize, 2usize, 492u32, 2126576u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2126580u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2126584u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2126588u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2126592u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2126596u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2126600u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2126604u32)?;
    emu.adi_no_count(10usize, 2usize, 300u32, 2126608u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126612u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2126616u32);
    emu.apc_no_count(1usize, 2126616u32, 16384u32, 2126620u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 232u32, 2126628u32);
    emu.sb_no_count(0usize, 2usize, 332u32, 2126632u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2126636u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126640u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2126644u32);
    emu.apc_no_count(1usize, 2126644u32, 12288u32, 2126648u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126652u32;
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
pub fn block_0x0020733c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 432u32, 2126656u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 124u32, 2126664u32);
    emu.adi_no_count(10usize, 2usize, 268u32, 2126668u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2126672u32);
    emu.apc_no_count(1usize, 2126672u32, 12288u32, 2126676u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 200u32, 2126684u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126688u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2126692u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2126696u32);
    emu.apc_no_count(1usize, 2126696u32, 45056u32, 2126700u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 300u32, 2126708u32);
    emu.adi_no_count(12usize, 2usize, 396u32, 2126712u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2126716u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2126720u32);
    emu.apc_no_count(1usize, 2126720u32, 45056u32, 2126724u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 332u32, 2126732u32);
    emu.lbu_no_count(11usize, 2usize, 428u32, 2126736u32);
    emu.sbr_no_count(12usize, 0usize, 9usize, 2126740u32);
    emu.xrr_no_count(11usize, 11usize, 10usize, 2126744u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2126748u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2126752u32);
    emu.sb_no_count(10usize, 2usize, 264u32, 2126756u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2126760u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2126764u32);
    emu.apc_no_count(1usize, 2126764u32, 45056u32, 2126768u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2126776u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126780u32);
    emu.apc_no_count(1usize, 2126780u32, 28672u32, 2126784u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 40u32, 2126792u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2126796u32);
    emu.apc_no_count(1usize, 2126796u32, 28672u32, 2126800u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2126808u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2126820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002073e4));
    } else {
        emu.pc = 2126812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002073dc));
    }
}
#[inline(always)]
pub fn block_0x002073dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2126816u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2126820u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2126820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002073e4));
}
#[inline(always)]
pub fn block_0x002073e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2126824u32)?;
    emu.lw_no_count(1usize, 2usize, 636u32, 2126828u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2126832u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2126836u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2126840u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2126844u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2126848u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126852u32;
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
pub fn block_0x00207404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 460u32, 2126856u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126860u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966761u32, 2126864u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126868u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966764u32, 2126872u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126876u32);
    emu.adi_no_count(13usize, 2usize, 460u32, 2126880u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2126884u32);
    emu.apc_no_count(1usize, 2126884u32, 16384u32, 2126888u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020742c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 364u32, 2126896u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126900u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 412u32, 2126904u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126908u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 416u32, 2126912u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126916u32);
    emu.adi_no_count(13usize, 2usize, 364u32, 2126920u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2126924u32);
    emu.apc_no_count(1usize, 2126924u32, 16384u32, 2126928u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2126936u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2126940u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2126944u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2126948u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2126952u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2126984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207488));
    } else {
        emu.pc = 2126956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020746c));
    }
}
#[inline(always)]
pub fn block_0x0020746c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2126960u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2126964u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2126968u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 376u32, 2126972u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2126976u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2126980u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2126984u32;
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
pub fn block_0x00207488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126988u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 13usize, 11usize, 2126992u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2126996u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2127000u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 376u32, 2127004u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2127008u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2127012u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2127016u32;
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
pub fn block_0x002074a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2127020u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2127024u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 784u32, 2127032u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127036u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 768u32, 2127040u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2127044u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2127048u32);
    emu.apc_no_count(1usize, 2127048u32, 110592u32, 2127052u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002074d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127060u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127064u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127068u32;
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
pub fn block_0x002074dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2127072u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2127076u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 896u32, 2127084u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127088u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 880u32, 2127092u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2127096u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2127100u32);
    emu.apc_no_count(1usize, 2127100u32, 110592u32, 2127104u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
