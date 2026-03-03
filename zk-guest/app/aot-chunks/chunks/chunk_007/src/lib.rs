pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2116348u32;
pub const PC_MAX: u32 = 2119320u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 116usize] = [
        block_0x00204afc,
        block_0x00204b04,
        block_0x00204b0c,
        block_0x00204b10,
        block_0x00204b24,
        block_0x00204b38,
        block_0x00204b88,
        block_0x00204b9c,
        block_0x00204ba0,
        block_0x00204bac,
        block_0x00204bfc,
        block_0x00204c10,
        block_0x00204c24,
        block_0x00204c30,
        block_0x00204c44,
        block_0x00204c4c,
        block_0x00204c58,
        block_0x00204c80,
        block_0x00204c88,
        block_0x00204c90,
        block_0x00204ca0,
        block_0x00204cc8,
        block_0x00204ccc,
        block_0x00204cdc,
        block_0x00204ce8,
        block_0x00204d28,
        block_0x00204d48,
        block_0x00204d54,
        block_0x00204d58,
        block_0x00204d70,
        block_0x00204d78,
        block_0x00204d80,
        block_0x00204d84,
        block_0x00204d90,
        block_0x00204d94,
        block_0x00204d9c,
        block_0x00204da0,
        block_0x00204dac,
        block_0x00204dbc,
        block_0x00204dc4,
        block_0x00204dcc,
        block_0x00204ddc,
        block_0x00204df0,
        block_0x00204df4,
        block_0x00204df8,
        block_0x00204dfc,
        block_0x00204e04,
        block_0x00204e08,
        block_0x00204e0c,
        block_0x00204e1c,
        block_0x00204e50,
        block_0x00204e64,
        block_0x00204e98,
        block_0x00204ed0,
        block_0x00204ee8,
        block_0x00204f74,
        block_0x00204f88,
        block_0x00204f94,
        block_0x00204fa8,
        block_0x00204fbc,
        block_0x00205004,
        block_0x00205018,
        block_0x00205020,
        block_0x00205174,
        block_0x00205184,
        block_0x00205194,
        block_0x002051a4,
        block_0x002051b4,
        block_0x002051c4,
        block_0x002051d4,
        block_0x002051e4,
        block_0x002051f0,
        block_0x00205240,
        block_0x00205278,
        block_0x002052b0,
        block_0x002052e8,
        block_0x00205320,
        block_0x00205358,
        block_0x00205390,
        block_0x002053c8,
        block_0x002053d4,
        block_0x002053f0,
        block_0x00205400,
        block_0x00205410,
        block_0x00205438,
        block_0x00205440,
        block_0x00205450,
        block_0x00205468,
        block_0x0020547c,
        block_0x00205484,
        block_0x00205488,
        block_0x002054a0,
        block_0x002054d0,
        block_0x002054d4,
        block_0x002054e8,
        block_0x00205510,
        block_0x00205524,
        block_0x00205528,
        block_0x00205548,
        block_0x00205558,
        block_0x00205560,
        block_0x00205568,
        block_0x00205570,
        block_0x00205574,
        block_0x00205590,
        block_0x0020559c,
        block_0x002055a4,
        block_0x002055a8,
        block_0x002055bc,
        block_0x002055d4,
        block_0x002055e4,
        block_0x00205618,
        block_0x00205630,
        block_0x00205664,
        block_0x0020566c,
        block_0x00205698,
    ];
    const IDX: [u16; 744usize] = [
        1u16, 0u16, 2u16, 0u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16,
        0u16, 8u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16,
        0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 31u16, 0u16, 32u16, 33u16, 0u16, 0u16, 34u16, 35u16, 0u16,
        36u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 39u16, 0u16, 40u16, 0u16,
        41u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 45u16,
        46u16, 0u16, 47u16, 48u16, 49u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 58u16,
        0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16,
        70u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16,
        0u16, 89u16, 0u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 101u16, 0u16, 102u16, 0u16, 103u16,
        104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 106u16, 0u16,
        107u16, 108u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        110u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 115u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16,
    ];
    if pc < 2116348u32 || pc > 2119320u32 {
        return None;
    }
    let word_offset = ((pc - 2116348u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00204afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2116348u32, 28672u32, 2116352u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2116356u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2116360u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116364u32;
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
pub fn block_0x00204b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116368u32;
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
pub fn block_0x00204b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2116372u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2116376u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2116380u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2116384u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2116388u32;
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
pub fn block_0x00204b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2116392u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2116396u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2116400u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2116404u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2116408u32;
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
pub fn block_0x00204b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2116412u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2116416u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2116420u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2116424u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116428u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2116432u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2116436u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116440u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2116444u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2116448u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116452u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2116456u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2116460u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116464u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2116468u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2116472u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2116476u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2116480u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2116484u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2116512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ba0));
    } else {
        emu.pc = 2116488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204b88));
    }
}
#[inline(always)]
pub fn block_0x00204b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2116492u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2116496u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2116500u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2116504u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2116624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c10));
    } else {
        emu.pc = 2116508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204b9c));
    }
}
#[inline(always)]
pub fn block_0x00204b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2116512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2116644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204c24));
}
#[inline(always)]
pub fn block_0x00204ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2116516u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2116520u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2116524u32;
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
pub fn block_0x00204bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2116528u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2116532u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2116536u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2116540u32)?;
    let a = 0u32.wrapping_add(4151074816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116544u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965826u32, 2116548u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2116552u32);
    let a = 0u32.wrapping_add(228253696u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116556u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 203u32, 2116560u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2116564u32);
    let a = 0u32.wrapping_add(1618087936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116568u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1443u32, 2116572u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2116576u32);
    let a = 0u32.wrapping_add(4257644544u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2116580u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 861u32, 2116584u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2116588u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2116592u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2116596u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2116600u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2116624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c10));
    } else {
        emu.pc = 2116604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204bfc));
    }
}
#[inline(always)]
pub fn block_0x00204bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2116608u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2116612u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2116616u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2116620u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2116644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c24));
    } else {
        emu.pc = 2116624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c10));
    }
}
#[inline(always)]
pub fn block_0x00204c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2116628u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2116632u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2116636u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2116640u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116644u32;
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
pub fn block_0x00204c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2116648u32)?;
    emu.apc_no_count(1usize, 2116648u32, 4096u32, 2116652u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2116660u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2116664u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2116668u32)?;
    emu.apc_no_count(1usize, 2116668u32, 0u32, 2116672u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2116676u32, 32768u32, 2116680u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 0u32, 2116688u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2116692u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2116808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204cc8));
    } else {
        emu.pc = 2116696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c58));
    }
}
#[inline]
pub fn block_0x00204c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2116700u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2116704u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2116708u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2116712u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2116716u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2116720u32)?;
    emu.lw_no_count(18usize, 8usize, 4u32, 2116724u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2116728u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2116732u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2116744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c88));
    } else {
        emu.pc = 2116736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c80));
    }
}
#[inline(always)]
pub fn block_0x00204c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2116740u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2116744u32;
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
pub fn block_0x00204c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2116748u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2116768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ca0));
    } else {
        emu.pc = 2116752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c90));
    }
}
#[inline(always)]
pub fn block_0x00204c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2116756u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2116760u32);
    emu.apc_no_count(1usize, 2116760u32, 4294955008u32, 2116764u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2116772u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2116776u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2116780u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2116784u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2116788u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2116792u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2116796u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2116800u32);
    emu.apc_no_count(6usize, 2116800u32, 4294955008u32, 2116804u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2116808u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116812u32;
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
pub fn block_0x00204ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2116816u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2116820u32)?;
    emu.apc_no_count(1usize, 2116820u32, 4096u32, 2116824u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2116832u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2116836u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116840u32;
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
pub fn block_0x00204ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2116844u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2116848u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2116852u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2116856u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2116860u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2116864u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2116868u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2116872u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2116876u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2116880u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2116884u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2116888u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2116892u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2116896u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2116900u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    } else {
        emu.pc = 2116904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d28));
    }
}
#[inline(always)]
pub fn block_0x00204d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2116908u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2116912u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2116916u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 304u32, 2116920u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2116924u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2116928u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2116932u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2116936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2116952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204d58));
}
#[inline(always)]
pub fn block_0x00204d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2116940u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2116944u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e0c));
    } else {
        emu.pc = 2116948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d54));
    }
}
#[inline(always)]
pub fn block_0x00204d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    } else {
        emu.pc = 2116952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d58));
    }
}
#[inline(always)]
pub fn block_0x00204d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2116956u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2116960u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2116964u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2116968u32);
    emu.apc_no_count(1usize, 2116968u32, 4096u32, 2116972u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 8u32, 2116980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d94));
    } else {
        emu.pc = 2116984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d78));
    }
}
#[inline(always)]
pub fn block_0x00204d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2116988u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2117128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e08));
    } else {
        emu.pc = 2116992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d80));
    }
}
#[inline(always)]
pub fn block_0x00204d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2117200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e50));
    } else {
        emu.pc = 2116996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d84));
    }
}
#[inline(always)]
pub fn block_0x00204d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 10usize, 2117000u32);
    emu.adr_no_count(9usize, 9usize, 10usize, 2117004u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2116952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d58));
    } else {
        emu.pc = 2117008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d90));
    }
}
#[inline(always)]
pub fn block_0x00204d90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2117012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2117148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204e1c));
}
#[inline(always)]
pub fn block_0x00204d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2117016u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2117112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df8));
    } else {
        emu.pc = 2117020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d9c));
    }
}
#[inline(always)]
pub fn block_0x00204d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2116936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d48));
    } else {
        emu.pc = 2117024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204da0));
    }
}
#[inline(always)]
pub fn block_0x00204da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2117028u32)?;
    emu.lbu_no_count(11usize, 18usize, 8u32, 2117032u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e0c));
    } else {
        emu.pc = 2117036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dac));
    }
}
#[inline(always)]
pub fn block_0x00204dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 18usize, 4u32, 2117040u32)?;
    emu.lw_no_count(11usize, 25usize, 0u32, 2117044u32)?;
    emu.lw_no_count(19usize, 18usize, 0u32, 2117048u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2117060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dc4));
    } else {
        emu.pc = 2117052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dbc));
    }
}
#[inline(always)]
pub fn block_0x00204dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2117056u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2117060u32;
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
pub fn block_0x00204dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 25usize, 4u32, 2117064u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2117084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ddc));
    } else {
        emu.pc = 2117068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dcc));
    }
}
#[inline(always)]
pub fn block_0x00204dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 25usize, 8u32, 2117072u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2117076u32);
    emu.apc_no_count(1usize, 2117076u32, 4294955008u32, 2117080u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2117088u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2117092u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2117096u32);
    emu.apc_no_count(1usize, 2117096u32, 4294955008u32, 2117100u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a != b {
        emu.pc = 2116952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d58));
    } else {
        emu.pc = 2117108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df4));
    }
}
#[inline(always)]
pub fn block_0x00204df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2117112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2117148u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204e1c));
}
#[inline(always)]
pub fn block_0x00204df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e0c));
    } else {
        emu.pc = 2117116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204dfc));
    }
}
#[inline(always)]
pub fn block_0x00204dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 9u32, 2117120u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2116948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204d54));
    } else {
        emu.pc = 2117124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e04));
    }
}
#[inline(always)]
pub fn block_0x00204e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2117128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2117132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204e0c));
}
#[inline(always)]
pub fn block_0x00204e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2117132u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2117132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204e0c));
}
#[inline(always)]
pub fn block_0x00204e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2117136u32)?;
    emu.ani_no_count(12usize, 11usize, 255u32, 2117140u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2117144u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2117220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e64));
    } else {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    }
}
#[inline]
pub fn block_0x00204e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2117152u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2117156u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2117160u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2117164u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2117168u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2117172u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2117176u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2117180u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2117184u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2117188u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2117192u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2117196u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117200u32;
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
pub fn block_0x00204e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2117204u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 312u32, 2117208u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2117212u32);
    emu.apc_no_count(1usize, 2117212u32, 73728u32, 2117216u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2117224u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2117228u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2117232u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2117236u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 180u32, 2117240u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2117244u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 328u32, 2117248u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2117252u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 344u32, 2117256u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2117260u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2117264u32);
    emu.apc_no_count(1usize, 2117264u32, 57344u32, 2117268u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2117276u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2117280u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2117284u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2117288u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2117292u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2117296u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2117300u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2117304u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2117308u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294967160u32, 2117312u32)?;
    emu.sw_no_count(0usize, 10usize, 4294967164u32, 2117316u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2117320u32);
    emu.sw_no_count(0usize, 10usize, 4294967160u32, 2117324u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2118640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002053f0));
    } else {
        emu.pc = 2117328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ed0));
    }
}
#[inline(always)]
pub fn block_0x00204ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967168u32, 2117336u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117340u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2117344u32);
    emu.apc_no_count(1usize, 2117344u32, 0u32, 2117348u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2117356u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2117360u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2117364u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2117368u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2117372u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2117376u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2117380u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2117384u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2117388u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2117392u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2117396u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2117400u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2117404u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2117408u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2117412u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2117416u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2117420u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2117424u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2117428u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2117432u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2117436u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2117440u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2117444u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2117448u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2117452u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2117456u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2117460u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2117464u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2117468u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2117472u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2117476u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2117480u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2117484u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2117488u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2117524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f94));
    } else {
        emu.pc = 2117492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f74));
    }
}
#[inline(always)]
pub fn block_0x00204f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2117496u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2117500u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2117504u32);
    emu.apc_no_count(1usize, 2117504u32, 0u32, 2117508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2117516u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2117520u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2117636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205004));
    } else {
        emu.pc = 2117524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f94));
    }
}
#[inline(always)]
pub fn block_0x00204f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2117528u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2117532u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2117536u32);
    emu.apc_no_count(1usize, 2117536u32, 4096u32, 2117540u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117544u32;
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
pub fn block_0x00204fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2117548u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2117552u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2117556u32);
    emu.apc_no_count(1usize, 2117556u32, 0u32, 2117560u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2117568u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2117572u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2117576u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2117580u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2117584u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2117588u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2117592u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2117596u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2117600u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2117604u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2117608u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2117612u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2117616u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2117620u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117624u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2117628u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2117632u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2117636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2117656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205018));
}
#[inline(always)]
pub fn block_0x00205004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2117640u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2117644u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2117648u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2117652u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2117656u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2117656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205018));
}
#[inline(always)]
pub fn block_0x00205018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2117656u32, 4096u32, 2117660u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2117668u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2117672u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2117676u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2117680u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2117684u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2117688u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2117692u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2117696u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2117700u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2117704u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2117708u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2117712u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2117716u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2117720u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2117724u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2117728u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2117732u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2117736u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2117740u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2117744u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2117748u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2117752u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2117756u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2117760u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2117764u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2117768u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2117772u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2117776u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2117780u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2117784u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2117788u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2117792u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2117796u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2117800u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2117804u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2117808u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2117812u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2117816u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2117820u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2117824u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2117828u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2117832u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2117836u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2117840u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2117844u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2117848u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2117852u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2117856u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2117860u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2117864u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2117868u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2117872u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2117876u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2117880u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2117884u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2117888u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2117892u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2117896u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2117900u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2117904u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2117908u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2117912u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2117916u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2117920u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2117924u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2117928u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2117932u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2117936u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2117940u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2117944u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2117948u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2117952u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2117956u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2117960u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2117964u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2117968u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2117972u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2117976u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2117980u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2117984u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2117988u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2117992u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2117996u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2118000u32);
    emu.add_memory_rw_events(85usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118008u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2118012u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2118016u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118024u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2118028u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2118032u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118040u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2118044u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2118048u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002051a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118056u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2118060u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2118064u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002051b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118072u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2118076u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2118080u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002051c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118088u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2118092u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2118096u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002051d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2118104u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2118108u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2118112u32);
    emu.add_memory_rw_events(4usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002051e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118120u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294967120u32, 2118124u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2118656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205400));
    } else {
        emu.pc = 2118128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002051f0));
    }
}
#[inline]
pub fn block_0x002051f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2118132u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2118136u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967120u32, 2118140u32);
    let a = 0u32.wrapping_add(2281701376u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2118144u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2118148u32)?;
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2118152u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2118156u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2118160u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2118164u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2118168u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2118172u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2118176u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2118180u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2118184u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2118188u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2118192u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2118196u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2118200u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118204u32);
    emu.add_memory_rw_events(20usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00205240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2118212u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118216u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118220u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118224u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118228u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118232u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118236u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118240u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118244u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118248u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118252u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118256u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2118260u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00205278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2118268u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118272u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118276u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118280u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118284u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118288u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118292u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118296u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118300u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118304u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118308u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118312u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2118316u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x002052b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2118324u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118328u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118332u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118336u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118340u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118344u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118348u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118352u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118356u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118360u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118364u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118368u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2118372u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x002052e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2118380u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118384u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118388u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118392u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118396u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118400u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118404u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118408u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118412u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118416u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118420u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118424u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2118428u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00205320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2118436u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118440u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118444u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118448u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118452u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118456u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118460u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118464u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118468u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118472u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118476u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118480u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2118484u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00205358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2118492u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2118496u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2118500u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118504u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118508u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2118512u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2118516u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118520u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2118524u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118528u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118532u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2118536u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2118540u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline]
pub fn block_0x00205390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2118548u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2118552u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2118556u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2118560u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2118564u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2118568u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2118572u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2118576u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2118580u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2118584u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2118588u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2118592u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2118596u32);
    emu.add_memory_rw_events(14usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002053c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2118604u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118608u32);
    emu.add_memory_rw_events(3usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x002053d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 223u32, 2118620u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2118624u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 392u32, 2118628u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2118632u32);
    emu.apc_no_count(1usize, 2118632u32, 40960u32, 2118636u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002053f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118644u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 360u32, 2118648u32);
    emu.apc_no_count(1usize, 2118648u32, 49152u32, 2118652u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118656u32;
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
pub fn block_0x00205400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 376u32, 2118664u32);
    emu.apc_no_count(1usize, 2118664u32, 49152u32, 2118668u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2118676u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2118680u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2118684u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2118688u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2118692u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2118696u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2118700u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2118704u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2118708u32);
    emu.add_memory_rw_events(10usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2118716u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2118952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205528));
    } else {
        emu.pc = 2118720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205440));
    }
}
#[inline(always)]
pub fn block_0x00205440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118724u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294967160u32, 2118728u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2118732u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2118984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205548));
    } else {
        emu.pc = 2118736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205450));
    }
}
#[inline(always)]
pub fn block_0x00205450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2118740u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294967160u32, 2118744u32);
    emu.lbu_no_count(19usize, 18usize, 112u32, 2118748u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2118752u32);
    emu.sbr_no_count(12usize, 10usize, 19usize, 2118756u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2118788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205484));
    } else {
        emu.pc = 2118760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205468));
    }
}
#[inline(always)]
pub fn block_0x00205468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 19usize, 2118764u32);
    emu.adi_no_count(10usize, 10usize, 48u32, 2118768u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2118772u32);
    emu.apc_no_count(1usize, 2118772u32, 0u32, 2118776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020547c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 8usize, 19usize, 2118784u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2118788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2118948u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205524));
}
#[inline(always)]
pub fn block_0x00205484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2118868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002054d4));
    } else {
        emu.pc = 2118792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205488));
    }
}
#[inline(always)]
pub fn block_0x00205488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(8usize, 8usize, 12usize, 2118796u32);
    emu.adr_no_count(20usize, 11usize, 12usize, 2118800u32);
    emu.adi_no_count(9usize, 18usize, 48u32, 2118804u32);
    emu.adr_no_count(10usize, 9usize, 19usize, 2118808u32);
    emu.apc_no_count(1usize, 2118808u32, 0u32, 2118812u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002054a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2118820u32)?;
    emu.lw_no_count(11usize, 18usize, 44u32, 2118824u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2118828u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2118832u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2118836u32);
    emu.sw_no_count(10usize, 18usize, 40u32, 2118840u32)?;
    emu.sw_no_count(11usize, 18usize, 44u32, 2118844u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2118848u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2118852u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2118856u32);
    emu.apc_no_count(1usize, 2118856u32, 4096u32, 2118860u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002054d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2118868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2118868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002054d4));
}
#[inline(always)]
pub fn block_0x002054d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 4294967232u32, 2118872u32);
    emu.ani_no_count(9usize, 8usize, 63u32, 2118876u32);
    emu.sri_no_count(12usize, 8usize, 6u32, 2118880u32);
    emu.adr_no_count(8usize, 11usize, 10usize, 2118884u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2118928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205510));
    } else {
        emu.pc = 2118888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002054e8));
    }
}
#[inline]
pub fn block_0x002054e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 40u32, 2118892u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2118896u32)?;
    emu.adr_no_count(14usize, 10usize, 12usize, 2118900u32);
    emu.sltru_no_count(10usize, 14usize, 10usize, 2118904u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2118908u32);
    emu.sw_no_count(14usize, 18usize, 40u32, 2118912u32)?;
    emu.sw_no_count(10usize, 18usize, 44u32, 2118916u32)?;
    emu.adi_no_count(10usize, 18usize, 8u32, 2118920u32);
    emu.apc_no_count(1usize, 2118920u32, 4096u32, 2118924u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 48u32, 2118932u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2118936u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2118940u32);
    emu.apc_no_count(1usize, 2118940u32, 0u32, 2118944u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 18usize, 112u32, 2118952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2118952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205528));
}
#[inline(always)]
pub fn block_0x00205528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2118956u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2118960u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2118964u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2118968u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2118972u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2118976u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2118980u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118984u32;
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
pub fn block_0x00205548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2118988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 408u32, 2118992u32);
    emu.apc_no_count(1usize, 2118992u32, 49152u32, 2118996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 240u32, 2119004u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 5usize, 0u32, 2119012u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119016u32;
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
pub fn block_0x00205568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 241u32, 2119020u32);
    emu.add_memory_rw_events(2usize);
    let syscall_id = emu.read_reg_snapshot(5);
    let arg2 = emu.read_reg_c(11);
    let arg1 = emu.read_reg_b(10);
    match emu.execute_syscall(syscall_id, arg1, arg2) {
        Ok((return_value, new_next_pc, extra_cycles, should_halt)) => {
            emu.write_reg_no_count(5, return_value);
            emu.pc = new_next_pc;
            emu.update_insn_clock();
            if extra_cycles > 0 {
                emu.clk = emu.clk.wrapping_add(extra_cycles);
            }
            if should_halt {
                emu.check_chunk_boundary_fast();
                return Ok(crate::NextStep::Halt);
            }
            emu.check_chunk_boundary_fast();
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        Err(e) => {
            return Err(e);
        }
    }
}
#[inline(always)]
pub fn block_0x00205570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119028u32;
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
pub fn block_0x00205574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2119032u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2119036u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2119040u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2119044u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2119048u32);
    emu.apc_no_count(1usize, 2119048u32, 0u32, 2119052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(20u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2119060u32);
    emu.apc_no_count(1usize, 2119060u32, 0u32, 2119064u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119068u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020559c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2119068u32, 0u32, 2119072u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2119076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002055a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055a8));
    }
}
#[inline(always)]
pub fn block_0x002055a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 0u32, 2119084u32);
    emu.adr_no_count(13usize, 12usize, 10usize, 2119088u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2119092u32);
    emu.sb_no_count(11usize, 13usize, 4294967295u32, 2119096u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055bc));
    }
}
#[inline(always)]
pub fn block_0x002055bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 1u32, 2119104u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2119108u32);
    emu.sb_no_count(11usize, 13usize, 4294967294u32, 2119112u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2119116u32);
    emu.sb_no_count(11usize, 13usize, 4294967293u32, 2119120u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055d4));
    }
}
#[inline(always)]
pub fn block_0x002055d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 10usize, 3u32, 2119128u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2119132u32);
    emu.sb_no_count(11usize, 13usize, 4294967292u32, 2119136u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002055e4));
    }
}
#[inline]
pub fn block_0x002055e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2119144u32);
    emu.ani_no_count(14usize, 13usize, 3u32, 2119148u32);
    emu.adr_no_count(13usize, 10usize, 14usize, 2119152u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2119156u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2119160u32);
    emu.ani_no_count(11usize, 11usize, 255u32, 2119164u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2119168u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 257u32, 2119172u32);
    emu.mul_no_count(11usize, 11usize, 14usize, 2119176u32);
    emu.sw_no_count(11usize, 13usize, 0u32, 2119180u32)?;
    emu.adr_no_count(14usize, 13usize, 12usize, 2119184u32);
    emu.sw_no_count(11usize, 14usize, 4294967292u32, 2119188u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205618));
    }
}
#[inline(always)]
pub fn block_0x00205618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 4u32, 2119196u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2119200u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967284u32, 2119204u32)?;
    emu.adi_no_count(15usize, 0usize, 25u32, 2119208u32);
    emu.sw_no_count(11usize, 14usize, 4294967288u32, 2119212u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205630));
    }
}
#[inline]
pub fn block_0x00205630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 12u32, 2119220u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2119224u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2119228u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2119232u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967268u32, 2119236u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967272u32, 2119240u32)?;
    emu.sw_no_count(11usize, 14usize, 4294967276u32, 2119244u32)?;
    emu.ani_no_count(15usize, 13usize, 4u32, 2119248u32);
    emu.ori_no_count(15usize, 15usize, 24u32, 2119252u32);
    emu.sbr_no_count(12usize, 12usize, 15usize, 2119256u32);
    emu.adi_no_count(16usize, 0usize, 32u32, 2119260u32);
    emu.sw_no_count(11usize, 14usize, 4294967280u32, 2119264u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    } else {
        emu.pc = 2119268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205664));
    }
}
#[inline(always)]
pub fn block_0x00205664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 15usize, 2119272u32);
    emu.adi_no_count(14usize, 0usize, 31u32, 2119276u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2119276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020566c));
}
#[inline]
pub fn block_0x0020566c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 13usize, 0u32, 2119280u32)?;
    emu.sw_no_count(11usize, 13usize, 4u32, 2119284u32)?;
    emu.sw_no_count(11usize, 13usize, 8u32, 2119288u32)?;
    emu.sw_no_count(11usize, 13usize, 12u32, 2119292u32)?;
    emu.sw_no_count(11usize, 13usize, 16u32, 2119296u32)?;
    emu.sw_no_count(11usize, 13usize, 20u32, 2119300u32)?;
    emu.sw_no_count(11usize, 13usize, 24u32, 2119304u32)?;
    emu.sw_no_count(11usize, 13usize, 28u32, 2119308u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967264u32, 2119312u32);
    emu.adi_no_count(13usize, 13usize, 32u32, 2119316u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2119276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020566c));
    } else {
        emu.pc = 2119320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205698));
    }
}
#[inline(always)]
pub fn block_0x00205698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2119324u32;
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
