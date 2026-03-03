pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2127108u32;
pub const PC_MAX: u32 = 2129716u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 120usize] = [
        block_0x00207504,
        block_0x00207510,
        block_0x00207538,
        block_0x00207544,
        block_0x00207560,
        block_0x0020757c,
        block_0x002075a4,
        block_0x002075b0,
        block_0x002075cc,
        block_0x002075e8,
        block_0x0020760c,
        block_0x00207618,
        block_0x0020764c,
        block_0x00207660,
        block_0x00207688,
        block_0x0020769c,
        block_0x002076bc,
        block_0x002076cc,
        block_0x002076e0,
        block_0x00207700,
        block_0x00207710,
        block_0x00207724,
        block_0x00207744,
        block_0x00207754,
        block_0x00207768,
        block_0x00207788,
        block_0x00207798,
        block_0x002077ac,
        block_0x002077c8,
        block_0x002077d8,
        block_0x002077ec,
        block_0x00207808,
        block_0x00207818,
        block_0x00207820,
        block_0x00207844,
        block_0x0020786c,
        block_0x00207870,
        block_0x00207874,
        block_0x00207880,
        block_0x00207890,
        block_0x002078a0,
        block_0x002078a4,
        block_0x002078b0,
        block_0x002078b8,
        block_0x002078d0,
        block_0x002078d4,
        block_0x00207904,
        block_0x00207910,
        block_0x00207914,
        block_0x00207928,
        block_0x00207938,
        block_0x00207940,
        block_0x00207948,
        block_0x00207960,
        block_0x0020796c,
        block_0x00207970,
        block_0x00207974,
        block_0x0020797c,
        block_0x0020799c,
        block_0x002079a8,
        block_0x002079b8,
        block_0x002079d0,
        block_0x00207a08,
        block_0x00207a1c,
        block_0x00207a5c,
        block_0x00207a70,
        block_0x00207a90,
        block_0x00207aac,
        block_0x00207ac8,
        block_0x00207ae0,
        block_0x00207aec,
        block_0x00207af0,
        block_0x00207b04,
        block_0x00207b14,
        block_0x00207b2c,
        block_0x00207b3c,
        block_0x00207b48,
        block_0x00207b6c,
        block_0x00207b7c,
        block_0x00207b80,
        block_0x00207ba4,
        block_0x00207bac,
        block_0x00207bdc,
        block_0x00207c2c,
        block_0x00207c30,
        block_0x00207c34,
        block_0x00207c48,
        block_0x00207c60,
        block_0x00207c64,
        block_0x00207c94,
        block_0x00207cc8,
        block_0x00207cd4,
        block_0x00207cf4,
        block_0x00207d00,
        block_0x00207d14,
        block_0x00207d20,
        block_0x00207d30,
        block_0x00207d34,
        block_0x00207d44,
        block_0x00207d60,
        block_0x00207d6c,
        block_0x00207da4,
        block_0x00207dbc,
        block_0x00207e00,
        block_0x00207e04,
        block_0x00207e08,
        block_0x00207e10,
        block_0x00207e28,
        block_0x00207e2c,
        block_0x00207e58,
        block_0x00207e74,
        block_0x00207e7c,
        block_0x00207e98,
        block_0x00207ea8,
        block_0x00207eac,
        block_0x00207ebc,
        block_0x00207ed8,
        block_0x00207ef8,
        block_0x00207f04,
        block_0x00207f34,
    ];
    const IDX: [u16; 653usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 30u16,
        0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        0u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 38u16,
        0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 41u16, 42u16, 0u16,
        0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 48u16,
        49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16, 0u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 56u16, 57u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16,
        0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 77u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 79u16,
        80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 84u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16,
        96u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 106u16, 0u16, 107u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 114u16, 115u16, 0u16,
        0u16, 0u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16,
    ];
    if pc < 2127108u32 || pc > 2129716u32 {
        return None;
    }
    let word_offset = ((pc - 2127108u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00207504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127112u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127116u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127120u32;
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
pub fn block_0x00207510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2127124u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2127128u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 840u32, 2127136u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127140u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 824u32, 2127144u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2127148u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2127152u32);
    emu.apc_no_count(1usize, 2127152u32, 110592u32, 2127156u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127160u32;
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
pub fn block_0x00207538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127164u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127172u32;
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
pub fn block_0x00207544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 859u32, 2127180u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2127184u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2127188u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127192u32);
    emu.apc_no_count(6usize, 2127192u32, 110592u32, 2127196u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2127200u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 949u32, 2127208u32);
    emu.adi_no_count(12usize, 0usize, 22u32, 2127212u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2127216u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127220u32);
    emu.apc_no_count(6usize, 2127220u32, 110592u32, 2127224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2127228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020757c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2127232u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2127236u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127240u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 804u32, 2127244u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127248u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 788u32, 2127252u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2127256u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2127260u32);
    emu.apc_no_count(1usize, 2127260u32, 110592u32, 2127264u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127268u32;
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
pub fn block_0x002075a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127272u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127276u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127280u32;
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
pub fn block_0x002075b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127284u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 914u32, 2127288u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2127292u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2127296u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127300u32);
    emu.apc_no_count(6usize, 2127300u32, 110592u32, 2127304u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2127308u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127312u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 940u32, 2127316u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2127320u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2127324u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127328u32);
    emu.apc_no_count(6usize, 2127328u32, 110592u32, 2127332u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2127336u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002075e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2127340u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127344u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 988u32, 2127348u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127352u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 972u32, 2127356u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2127360u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2127364u32);
    emu.apc_no_count(1usize, 2127364u32, 110592u32, 2127368u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127372u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020760c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2127376u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2127380u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127384u32;
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
pub fn block_0x00207618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2127388u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2127392u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2127396u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2127400u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2127404u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2127408u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2127412u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2127416u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2127420u32);
    emu.lw_no_count(11usize, 11usize, 12u32, 2127424u32)?;
    emu.lw_no_count(18usize, 8usize, 20u32, 2127428u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2127432u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2127456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207660));
    } else {
        emu.pc = 2127436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020764c));
    }
}
#[inline(always)]
pub fn block_0x0020764c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 12u32, 2127440u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1908u32, 2127448u32);
    emu.apc_no_count(1usize, 2127448u32, 12288u32, 2127452u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2127460u32)?;
    emu.sli_no_count(11usize, 18usize, 2u32, 2127464u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2127468u32);
    emu.adi_no_count(11usize, 0usize, 192u32, 2127472u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2127476u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2127480u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2127484u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2127488u32);
    emu.sw_no_count(18usize, 8usize, 20u32, 2127492u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2127516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020769c));
    } else {
        emu.pc = 2127496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207688));
    }
}
#[inline(always)]
pub fn block_0x00207688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127500u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127504u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127508u32);
    emu.apc_no_count(1usize, 2127508u32, 69632u32, 2127512u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127516u32;
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
pub fn block_0x0020769c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127520u32)?;
    emu.adi_no_count(18usize, 9usize, 32u32, 2127524u32);
    emu.sli_no_count(11usize, 20usize, 5u32, 2127528u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2127532u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127536u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2127540u32);
    emu.apc_no_count(1usize, 2127540u32, 12288u32, 2127544u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2127552u32)?;
    emu.adi_no_count(21usize, 20usize, 1u32, 2127556u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2127560u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2127584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076e0));
    } else {
        emu.pc = 2127564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076cc));
    }
}
#[inline(always)]
pub fn block_0x002076cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127568u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127572u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127576u32);
    emu.apc_no_count(1usize, 2127576u32, 69632u32, 2127580u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127588u32)?;
    emu.adi_no_count(19usize, 9usize, 64u32, 2127592u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2127596u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2127600u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127604u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2127608u32);
    emu.apc_no_count(1usize, 2127608u32, 12288u32, 2127612u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127616u32;
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
pub fn block_0x00207700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2127620u32)?;
    emu.adi_no_count(21usize, 20usize, 2u32, 2127624u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2127628u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2127652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207724));
    } else {
        emu.pc = 2127632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207710));
    }
}
#[inline(always)]
pub fn block_0x00207710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127636u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127640u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127644u32);
    emu.apc_no_count(1usize, 2127644u32, 69632u32, 2127648u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127656u32)?;
    emu.adi_no_count(18usize, 9usize, 96u32, 2127660u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2127664u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2127668u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127672u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2127676u32);
    emu.apc_no_count(1usize, 2127676u32, 12288u32, 2127680u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2127688u32)?;
    emu.adi_no_count(21usize, 20usize, 3u32, 2127692u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2127696u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2127720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207768));
    } else {
        emu.pc = 2127700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207754));
    }
}
#[inline(always)]
pub fn block_0x00207754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127704u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127708u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127712u32);
    emu.apc_no_count(1usize, 2127712u32, 69632u32, 2127716u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127724u32)?;
    emu.adi_no_count(19usize, 9usize, 128u32, 2127728u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2127732u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2127736u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127740u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2127744u32);
    emu.apc_no_count(1usize, 2127744u32, 12288u32, 2127748u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2127756u32)?;
    emu.adi_no_count(18usize, 20usize, 4u32, 2127760u32);
    emu.sw_no_count(18usize, 8usize, 8u32, 2127764u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2127788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077ac));
    } else {
        emu.pc = 2127768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207798));
    }
}
#[inline(always)]
pub fn block_0x00207798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127776u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127780u32);
    emu.apc_no_count(1usize, 2127780u32, 69632u32, 2127784u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127788u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002077ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127792u32)?;
    emu.sli_no_count(18usize, 18usize, 5u32, 2127796u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2127800u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127804u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2127808u32);
    emu.apc_no_count(1usize, 2127808u32, 12288u32, 2127812u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002077c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2127820u32)?;
    emu.adi_no_count(18usize, 20usize, 5u32, 2127824u32);
    emu.sw_no_count(18usize, 8usize, 8u32, 2127828u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2127852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077ec));
    } else {
        emu.pc = 2127832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077d8));
    }
}
#[inline(always)]
pub fn block_0x002077d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965292u32, 2127840u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2127844u32);
    emu.apc_no_count(1usize, 2127844u32, 69632u32, 2127848u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002077ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2127856u32)?;
    emu.adi_no_count(11usize, 9usize, 160u32, 2127860u32);
    emu.sli_no_count(18usize, 18usize, 5u32, 2127864u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2127868u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2127872u32);
    emu.apc_no_count(1usize, 2127872u32, 12288u32, 2127876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 20u32, 2127884u32)?;
    emu.adi_no_count(20usize, 20usize, 6u32, 2127888u32);
    emu.sw_no_count(20usize, 8usize, 8u32, 2127892u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2127904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207820));
    } else {
        emu.pc = 2127896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207818));
    }
}
#[inline(always)]
pub fn block_0x00207818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2127900u32);
    emu.sw_no_count(10usize, 8usize, 20u32, 2127904u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2127904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207820));
}
#[inline]
pub fn block_0x00207820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2127908u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2127912u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2127916u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2127920u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2127924u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2127928u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2127932u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2127936u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127940u32;
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
pub fn block_0x00207844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2127944u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2127948u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2127952u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2127956u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2127960u32)?;
    emu.adi_no_count(11usize, 10usize, 0u32, 2127964u32);
    emu.lw_no_count(12usize, 10usize, 4u32, 2127968u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2127972u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2127976u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2128032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078a0));
    } else {
        emu.pc = 2127980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020786c));
    }
}
#[inline(always)]
pub fn block_0x0020786c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2128036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078a4));
    } else {
        emu.pc = 2127984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207870));
    }
}
#[inline(always)]
pub fn block_0x00207870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2128036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078a4));
    } else {
        emu.pc = 2127988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207874));
    }
}
#[inline(always)]
pub fn block_0x00207874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2127992u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2127996u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2128000u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2128000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207880));
}
#[inline(always)]
pub fn block_0x00207880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2128004u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2128008u32);
    emu.apc_no_count(1usize, 2128008u32, 12288u32, 2128012u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128016u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 4u32, 2128020u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2128024u32)?;
    emu.sw_no_count(8usize, 2usize, 12u32, 2128028u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2128032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002078b0));
}
#[inline(always)]
pub fn block_0x002078a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2128132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207904));
    } else {
        emu.pc = 2128036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078a4));
    }
}
#[inline(always)]
pub fn block_0x002078a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2128040u32);
    emu.apc_no_count(1usize, 2128040u32, 81920u32, 2128044u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002078b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2128048u32, 8192u32, 2128052u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002078b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128060u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2128064u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2128068u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2128072u32);
    emu.apc_no_count(1usize, 2128072u32, 12288u32, 2128076u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002078d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2128168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207928));
    } else {
        emu.pc = 2128084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078d4));
    }
}
#[inline]
pub fn block_0x002078d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2128088u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2128092u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2128096u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2128100u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2128104u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2128108u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2128112u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2128116u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2128120u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2128124u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2128128u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128132u32;
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
pub fn block_0x00207904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2128136u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2128140u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2128184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207938));
    } else {
        emu.pc = 2128144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207910));
    }
}
#[inline(always)]
pub fn block_0x00207910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2128148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207914));
}
#[inline(always)]
pub fn block_0x00207914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128152u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 2020u32, 2128156u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2128160u32);
    emu.apc_no_count(1usize, 2128160u32, 81920u32, 2128164u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128168u32;
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
pub fn block_0x00207928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2128172u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2128176u32);
    emu.apc_no_count(1usize, 2128176u32, 81920u32, 2128180u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 0u32, 2128188u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2128240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207970));
    } else {
        emu.pc = 2128192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207940));
    }
}
#[inline(always)]
pub fn block_0x00207940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2128192u32, 8192u32, 2128196u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128204u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2128208u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2128212u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2128216u32);
    emu.apc_no_count(1usize, 2128216u32, 12288u32, 2128220u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128224u32;
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
pub fn block_0x00207960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2128228u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2128232u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2128244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207974));
    } else {
        emu.pc = 2128236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020796c));
    }
}
#[inline(always)]
pub fn block_0x0020796c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2128240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207914));
}
#[inline(always)]
pub fn block_0x00207970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2128244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207974));
}
#[inline(always)]
pub fn block_0x00207974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 18usize, 0u32, 2128248u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2128252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128000u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207880));
}
#[inline(always)]
pub fn block_0x0020797c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966880u32, 2128256u32);
    emu.sw_no_count(1usize, 2usize, 412u32, 2128260u32)?;
    emu.sw_no_count(8usize, 2usize, 408u32, 2128264u32)?;
    emu.sw_no_count(9usize, 2usize, 404u32, 2128268u32)?;
    emu.sw_no_count(18usize, 2usize, 400u32, 2128272u32)?;
    emu.sw_no_count(19usize, 2usize, 396u32, 2128276u32)?;
    emu.sw_no_count(20usize, 2usize, 392u32, 2128280u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2128764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b7c));
    } else {
        emu.pc = 2128284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020799c));
    }
}
#[inline(always)]
pub fn block_0x0020799c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2128288u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2128292u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2128764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b7c));
    } else {
        emu.pc = 2128296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002079a8));
    }
}
#[inline(always)]
pub fn block_0x002079a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 61u32, 2128300u32);
    emu.srr_no_count(14usize, 14usize, 13usize, 2128304u32);
    emu.ani_no_count(14usize, 14usize, 1u32, 2128308u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2128764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b7c));
    } else {
        emu.pc = 2128312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002079b8));
    }
}
#[inline(always)]
pub fn block_0x002079b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2128316u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2128320u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966876u32, 2128324u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2128328u32);
    emu.lw_no_count(13usize, 13usize, 0u32, 2128332u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2128764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b7c));
    } else {
        emu.pc = 2128336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002079d0));
    }
}
#[inline]
pub fn block_0x002079d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2128340u32);
    emu.sltiu_no_count(10usize, 12usize, 65u32, 2128344u32);
    emu.adi_no_count(13usize, 0usize, 65u32, 2128348u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2128352u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2128356u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2128360u32);
    emu.adi_no_count(10usize, 2usize, 132u32, 2128364u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2128368u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2128372u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128376u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2128380u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2128384u32);
    emu.apc_no_count(1usize, 2128384u32, 12288u32, 2128388u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 132u32, 2128396u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2128400u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2128404u32);
    emu.apc_no_count(1usize, 2128404u32, 12288u32, 2128408u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 133u32, 2128416u32);
    emu.lbu_no_count(11usize, 2usize, 132u32, 2128420u32);
    emu.lbu_no_count(12usize, 2usize, 134u32, 2128424u32);
    emu.lbu_no_count(8usize, 2usize, 135u32, 2128428u32);
    emu.sli_no_count(10usize, 10usize, 8u32, 2128432u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2128436u32);
    emu.lbu_no_count(9usize, 2usize, 136u32, 2128440u32);
    emu.lbu_no_count(19usize, 2usize, 137u32, 2128444u32);
    emu.lbu_no_count(20usize, 2usize, 138u32, 2128448u32);
    emu.sh_no_count(10usize, 2usize, 2u32, 2128452u32)?;
    emu.sb_no_count(12usize, 2usize, 4u32, 2128456u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2128460u32);
    emu.adi_no_count(10usize, 2usize, 268u32, 2128464u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2128468u32);
    emu.apc_no_count(1usize, 2128468u32, 12288u32, 2128472u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 9u32, 2128480u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2128484u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2128488u32);
    emu.apc_no_count(1usize, 2128488u32, 12288u32, 2128492u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 2usize, 5u32, 2128500u32);
    emu.sb_no_count(9usize, 2usize, 6u32, 2128504u32);
    emu.sb_no_count(19usize, 2usize, 7u32, 2128508u32);
    emu.sb_no_count(20usize, 2usize, 8u32, 2128512u32);
    emu.adi_no_count(10usize, 2usize, 132u32, 2128516u32);
    emu.adi_no_count(11usize, 2usize, 2u32, 2128520u32);
    emu.apc_no_count(1usize, 2128520u32, 4294950912u32, 2128524u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128528u32;
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
pub fn block_0x00207a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 2u32, 2128532u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2128536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966808u32, 2128540u32);
    emu.adi_no_count(10usize, 2usize, 268u32, 2128544u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2128548u32);
    emu.apc_no_count(1usize, 2128548u32, 12288u32, 2128552u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(8usize, 2usize, 200u32, 2128560u32);
    emu.adi_no_count(10usize, 2usize, 204u32, 2128564u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2128568u32);
    emu.adi_no_count(12usize, 2usize, 132u32, 2128572u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2128576u32);
    emu.apc_no_count(1usize, 2128576u32, 45056u32, 2128580u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128584u32;
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
pub fn block_0x00207ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 300u32, 2128588u32);
    emu.adi_no_count(12usize, 2usize, 164u32, 2128592u32);
    emu.adi_no_count(10usize, 2usize, 236u32, 2128596u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2128600u32);
    emu.apc_no_count(1usize, 2128600u32, 45056u32, 2128604u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 4294967294u32, 2128612u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2128616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2128624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207af0));
    } else {
        emu.pc = 2128620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207aec));
    }
}
#[inline(always)]
pub fn block_0x00207aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2128812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bac));
    } else {
        emu.pc = 2128624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207af0));
    }
}
#[inline(always)]
pub fn block_0x00207af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(20usize, 2usize, 196u32, 2128628u32);
    emu.lbu_no_count(19usize, 2usize, 332u32, 2128632u32);
    emu.sltiu_no_count(10usize, 9usize, 1u32, 2128636u32);
    emu.apc_no_count(1usize, 2128636u32, 12288u32, 2128640u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2128648u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2128652u32);
    emu.apc_no_count(1usize, 2128652u32, 12288u32, 2128656u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128660u32;
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
#[inline(always)]
pub fn block_0x00207b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2128664u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2128668u32);
    emu.adi_no_count(11usize, 2usize, 204u32, 2128672u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2128676u32);
    emu.apc_no_count(1usize, 2128676u32, 12288u32, 2128680u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 200u32, 2128688u32);
    emu.anr_no_count(10usize, 9usize, 10usize, 2128692u32);
    emu.apc_no_count(1usize, 2128692u32, 12288u32, 2128696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2128704u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2128708u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2128804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ba4));
    } else {
        emu.pc = 2128712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b48));
    }
}
#[inline]
pub fn block_0x00207b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 19usize, 20usize, 2128716u32);
    emu.sbr_no_count(11usize, 0usize, 8usize, 2128720u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2128724u32);
    emu.xrr_no_count(8usize, 10usize, 19usize, 2128728u32);
    emu.adi_no_count(10usize, 18usize, 4u32, 2128732u32);
    emu.adi_no_count(11usize, 2usize, 68u32, 2128736u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2128740u32);
    emu.apc_no_count(1usize, 2128740u32, 12288u32, 2128744u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128748u32;
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
pub fn block_0x00207b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2128752u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128756u32);
    emu.sb_no_count(8usize, 18usize, 68u32, 2128760u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2128764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207b80));
}
#[inline(always)]
pub fn block_0x00207b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2128768u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207b80));
}
#[inline]
pub fn block_0x00207b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2128772u32)?;
    emu.lw_no_count(1usize, 2usize, 412u32, 2128776u32)?;
    emu.lw_no_count(8usize, 2usize, 408u32, 2128780u32)?;
    emu.lw_no_count(9usize, 2usize, 404u32, 2128784u32)?;
    emu.lw_no_count(18usize, 2usize, 400u32, 2128788u32)?;
    emu.lw_no_count(19usize, 2usize, 396u32, 2128792u32)?;
    emu.lw_no_count(20usize, 2usize, 392u32, 2128796u32)?;
    emu.adi_no_count(2usize, 2usize, 416u32, 2128800u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128804u32;
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
pub fn block_0x00207ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2128808u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2128812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207b80));
}
#[inline]
pub fn block_0x00207bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 7u32, 2128816u32);
    emu.sw_no_count(10usize, 2usize, 336u32, 2128820u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128824u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966240u32, 2128828u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2128832u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966224u32, 2128836u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2128840u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966252u32, 2128844u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2128848u32);
    emu.adi_no_count(12usize, 2usize, 336u32, 2128852u32);
    emu.apc_no_count(1usize, 2128852u32, 102400u32, 2128856u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967120u32, 2128864u32);
    emu.sw_no_count(1usize, 2usize, 172u32, 2128868u32)?;
    emu.sw_no_count(8usize, 2usize, 168u32, 2128872u32)?;
    emu.sw_no_count(9usize, 2usize, 164u32, 2128876u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2128880u32)?;
    emu.sw_no_count(19usize, 2usize, 156u32, 2128884u32)?;
    emu.sw_no_count(20usize, 2usize, 152u32, 2128888u32)?;
    emu.sw_no_count(21usize, 2usize, 148u32, 2128892u32)?;
    emu.sw_no_count(22usize, 2usize, 144u32, 2128896u32)?;
    emu.sw_no_count(23usize, 2usize, 140u32, 2128900u32)?;
    emu.sw_no_count(24usize, 2usize, 136u32, 2128904u32)?;
    emu.sw_no_count(25usize, 2usize, 132u32, 2128908u32)?;
    emu.sw_no_count(26usize, 2usize, 128u32, 2128912u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2128916u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2128920u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2128924u32);
    let a = 0u32.wrapping_add(20480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128928u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965540u32, 2128932u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2128936u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2128944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c30));
    } else {
        emu.pc = 2128940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c2c));
    }
}
#[inline(always)]
pub fn block_0x00207c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2128944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207c30));
}
#[inline(always)]
pub fn block_0x00207c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2129204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d34));
    } else {
        emu.pc = 2128948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c34));
    }
}
#[inline(always)]
pub fn block_0x00207c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 20usize, 3u32, 2128952u32);
    emu.sli_no_count(11usize, 20usize, 6u32, 2128956u32);
    emu.sbr_no_count(19usize, 11usize, 10usize, 2128960u32);
    emu.apc_no_count(1usize, 2128960u32, 8192u32, 2128964u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128972u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2128976u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2128980u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2128984u32);
    emu.apc_no_count(1usize, 2128984u32, 12288u32, 2128988u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2129316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207da4));
    } else {
        emu.pc = 2128996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c64));
    }
}
#[inline]
pub fn block_0x00207c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 12u32, 2129000u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2129004u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2129008u32)?;
    emu.adi_no_count(19usize, 2usize, 80u32, 2129012u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2129016u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966136u32, 2129020u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2129024u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294966192u32, 2129028u32);
    emu.adi_no_count(23usize, 0usize, 3u32, 2129032u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2129036u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294965792u32, 2129040u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2129044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207cd4));
}
#[inline]
pub fn block_0x00207c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2129048u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2129052u32)?;
    emu.sli_no_count(11usize, 26usize, 3u32, 2129056u32);
    emu.sli_no_count(12usize, 26usize, 6u32, 2129060u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2129064u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2129068u32);
    emu.sw_no_count(25usize, 10usize, 0u32, 2129072u32)?;
    emu.sw_no_count(24usize, 10usize, 4u32, 2129076u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2129080u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2129084u32);
    emu.adi_no_count(12usize, 0usize, 48u32, 2129088u32);
    emu.apc_no_count(1usize, 2129088u32, 12288u32, 2129092u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 26usize, 1u32, 2129100u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2129104u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2129220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d44));
    } else {
        emu.pc = 2129108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207cd4));
    }
}
#[inline(always)]
pub fn block_0x00207cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2129112u32);
    emu.adi_no_count(13usize, 0usize, 17u32, 2129116u32);
    emu.adi_no_count(15usize, 0usize, 4u32, 2129120u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2129124u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2129128u32);
    emu.adi_no_count(14usize, 21usize, 0u32, 2129132u32);
    emu.apc_no_count(1usize, 2129132u32, 4294946816u32, 2129136u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 72u32, 2129144u32)?;
    emu.lw_no_count(24usize, 2usize, 76u32, 2129148u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2129248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d60));
    } else {
        emu.pc = 2129152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d00));
    }
}
#[inline(always)]
pub fn block_0x00207d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2129156u32);
    emu.adi_no_count(12usize, 0usize, 48u32, 2129160u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2129164u32);
    emu.apc_no_count(1usize, 2129164u32, 8192u32, 2129168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2129176u32)?;
    emu.lw_no_count(26usize, 2usize, 20u32, 2129180u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2129044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c94));
    } else {
        emu.pc = 2129184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d20));
    }
}
#[inline(always)]
pub fn block_0x00207d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2129188u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2129192u32);
    emu.apc_no_count(1usize, 2129192u32, 4294955008u32, 2129196u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2129204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207c94));
}
#[inline(always)]
pub fn block_0x00207d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2129208u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2129212u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2129216u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2129220u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2129220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d44));
}
#[inline(always)]
pub fn block_0x00207d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2129224u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2129228u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2129232u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2129236u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2129240u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2129244u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2129248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d6c));
}
#[inline(always)]
pub fn block_0x00207d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129252u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2129256u32)?;
    emu.sw_no_count(24usize, 8usize, 4u32, 2129260u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2129260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d6c));
}
#[inline]
pub fn block_0x00207d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 172u32, 2129264u32)?;
    emu.lw_no_count(8usize, 2usize, 168u32, 2129268u32)?;
    emu.lw_no_count(9usize, 2usize, 164u32, 2129272u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2129276u32)?;
    emu.lw_no_count(19usize, 2usize, 156u32, 2129280u32)?;
    emu.lw_no_count(20usize, 2usize, 152u32, 2129284u32)?;
    emu.lw_no_count(21usize, 2usize, 148u32, 2129288u32)?;
    emu.lw_no_count(22usize, 2usize, 144u32, 2129292u32)?;
    emu.lw_no_count(23usize, 2usize, 140u32, 2129296u32)?;
    emu.lw_no_count(24usize, 2usize, 136u32, 2129300u32)?;
    emu.lw_no_count(25usize, 2usize, 132u32, 2129304u32)?;
    emu.lw_no_count(26usize, 2usize, 128u32, 2129308u32)?;
    emu.adi_no_count(2usize, 2usize, 176u32, 2129312u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129316u32;
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
pub fn block_0x00207da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129320u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965776u32, 2129324u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2129328u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2129332u32);
    emu.apc_no_count(1usize, 2129332u32, 81920u32, 2129336u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129340u32;
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
pub fn block_0x00207dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2129344u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2129348u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2129352u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2129356u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2129360u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2129364u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2129368u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2129372u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2129376u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2129380u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2129384u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2129388u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2129392u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2129396u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129400u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2129404u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2129412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e04));
    } else {
        emu.pc = 2129408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e00));
    }
}
#[inline(always)]
pub fn block_0x00207e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2129412u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2129412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207e04));
}
#[inline(always)]
pub fn block_0x00207e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2129580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207eac));
    } else {
        emu.pc = 2129416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e08));
    }
}
#[inline(always)]
pub fn block_0x00207e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2129416u32, 8192u32, 2129420u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129428u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2129432u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2129436u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2129440u32);
    emu.apc_no_count(1usize, 2129440u32, 12288u32, 2129444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2129716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207f34));
    } else {
        emu.pc = 2129452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e2c));
    }
}
#[inline]
pub fn block_0x00207e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2129456u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2129460u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2129464u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2129468u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2129472u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2129476u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2129480u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2129484u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2129488u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294965792u32, 2129492u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2129496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207e74));
}
#[inline(always)]
pub fn block_0x00207e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2129500u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2129504u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2129508u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2129512u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2129516u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2129520u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2129596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ebc));
    } else {
        emu.pc = 2129524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e74));
    }
}
#[inline(always)]
pub fn block_0x00207e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 22usize, 20usize, 2129528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2129624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ed8));
    } else {
        emu.pc = 2129532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e7c));
    }
}
#[inline(always)]
pub fn block_0x00207e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2129536u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2129540u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2129544u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2129548u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2129552u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2129556u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2129496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e58));
    } else {
        emu.pc = 2129560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e98));
    }
}
#[inline(always)]
pub fn block_0x00207e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2129564u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2129568u32);
    emu.apc_no_count(1usize, 2129568u32, 81920u32, 2129572u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2129580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207e58));
}
#[inline(always)]
pub fn block_0x00207eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2129584u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2129588u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2129592u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2129596u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2129596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207ebc));
}
#[inline(always)]
pub fn block_0x00207ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2129600u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2129604u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2129608u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2129612u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2129616u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2129620u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2129624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207f04));
}
#[inline(always)]
pub fn block_0x00207ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2129628u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2129636u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2129640u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2129644u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2129648u32);
    emu.apc_no_count(1usize, 2129648u32, 12288u32, 2129652u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129660u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2129664u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2129668u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2129668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207f04));
}
#[inline]
pub fn block_0x00207f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2129672u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2129676u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2129680u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2129684u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2129688u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2129692u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2129696u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2129700u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2129704u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2129708u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2129712u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129716u32;
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
pub fn block_0x00207f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129720u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965776u32, 2129724u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2129728u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2129732u32);
    emu.apc_no_count(1usize, 2129732u32, 81920u32, 2129736u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
