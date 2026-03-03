pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2239652u32;
pub const PC_MAX: u32 = 2241496u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 123usize] = [
        block_0x00222ca4,
        block_0x00222cbc,
        block_0x00222cf0,
        block_0x00222cf4,
        block_0x00222cfc,
        block_0x00222d14,
        block_0x00222d20,
        block_0x00222d2c,
        block_0x00222d30,
        block_0x00222d34,
        block_0x00222d4c,
        block_0x00222d50,
        block_0x00222d5c,
        block_0x00222d78,
        block_0x00222d8c,
        block_0x00222da0,
        block_0x00222db8,
        block_0x00222dc4,
        block_0x00222dd4,
        block_0x00222ddc,
        block_0x00222e08,
        block_0x00222e14,
        block_0x00222e1c,
        block_0x00222e58,
        block_0x00222e64,
        block_0x00222eb0,
        block_0x00222ebc,
        block_0x00222ee4,
        block_0x00222eec,
        block_0x00222f0c,
        block_0x00222f28,
        block_0x00222f40,
        block_0x00222f44,
        block_0x00222f68,
        block_0x00222f70,
        block_0x00222f78,
        block_0x00222f90,
        block_0x00222f94,
        block_0x00222fb8,
        block_0x00222fc0,
        block_0x00222fc8,
        block_0x00222fe8,
        block_0x00222ff0,
        block_0x00223008,
        block_0x00223018,
        block_0x00223030,
        block_0x0022306c,
        block_0x002230e0,
        block_0x002230e8,
        block_0x002230f4,
        block_0x00223100,
        block_0x00223104,
        block_0x00223120,
        block_0x00223128,
        block_0x0022312c,
        block_0x00223134,
        block_0x0022313c,
        block_0x00223148,
        block_0x0022314c,
        block_0x00223160,
        block_0x00223164,
        block_0x00223168,
        block_0x00223170,
        block_0x00223180,
        block_0x00223184,
        block_0x0022318c,
        block_0x00223198,
        block_0x002231a0,
        block_0x002231a8,
        block_0x002231ac,
        block_0x002231b4,
        block_0x002231c0,
        block_0x002231c8,
        block_0x002231d0,
        block_0x002231d4,
        block_0x002231d8,
        block_0x002231dc,
        block_0x002231e0,
        block_0x002231e8,
        block_0x002231ec,
        block_0x002231f4,
        block_0x002231f8,
        block_0x00223200,
        block_0x0022320c,
        block_0x00223214,
        block_0x0022321c,
        block_0x00223228,
        block_0x0022322c,
        block_0x00223234,
        block_0x0022323c,
        block_0x00223244,
        block_0x00223250,
        block_0x00223258,
        block_0x00223264,
        block_0x0022326c,
        block_0x0022327c,
        block_0x00223284,
        block_0x0022328c,
        block_0x00223294,
        block_0x00223298,
        block_0x002232b0,
        block_0x002232f0,
        block_0x00223304,
        block_0x00223318,
        block_0x0022331c,
        block_0x00223320,
        block_0x00223324,
        block_0x00223328,
        block_0x00223338,
        block_0x0022333c,
        block_0x00223340,
        block_0x00223348,
        block_0x00223350,
        block_0x00223360,
        block_0x0022336c,
        block_0x00223370,
        block_0x00223374,
        block_0x00223390,
        block_0x00223398,
        block_0x002233a0,
        block_0x002233a8,
        block_0x002233c0,
        block_0x002233d8,
    ];
    const IDX: [u16; 462usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 9u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16,
        0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 39u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 58u16,
        59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 62u16, 0u16, 63u16, 0u16, 0u16,
        0u16, 64u16, 65u16, 0u16, 66u16, 0u16, 0u16, 67u16, 0u16, 68u16, 0u16, 69u16,
        70u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 75u16, 76u16,
        77u16, 78u16, 0u16, 79u16, 80u16, 0u16, 81u16, 82u16, 0u16, 83u16, 0u16, 0u16,
        84u16, 0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 87u16, 88u16, 0u16, 89u16, 0u16,
        90u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 94u16, 0u16,
        95u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 99u16, 100u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 106u16, 107u16, 108u16, 0u16,
        0u16, 0u16, 109u16, 110u16, 111u16, 0u16, 112u16, 0u16, 113u16, 0u16, 0u16, 0u16,
        114u16, 0u16, 0u16, 115u16, 116u16, 117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        118u16, 0u16, 119u16, 0u16, 120u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16,
    ];
    if pc < 2239652u32 || pc > 2241496u32 {
        return None;
    }
    let word_offset = ((pc - 2239652u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00222ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2239656u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2239660u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2239664u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2239668u32);
    emu.apc_no_count(6usize, 2239668u32, 4294963200u32, 2239672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2239676u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00222cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2239680u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2239684u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2239688u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2239692u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2239696u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2239700u32)?;
    emu.lw_no_count(19usize, 11usize, 4u32, 2239704u32)?;
    emu.lw_no_count(8usize, 11usize, 0u32, 2239708u32)?;
    emu.lw_no_count(18usize, 19usize, 16u32, 2239712u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2239716u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2239720u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2239724u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2239728u32;
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
pub fn block_0x00222cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222cfc));
    } else {
        emu.pc = 2239732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222cf4));
    }
}
#[inline(always)]
pub fn block_0x00222cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2239736u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239740u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222d5c));
}
#[inline(always)]
pub fn block_0x00222cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2239744u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2239748u32);
    emu.adi_no_count(12usize, 0usize, 257u32, 2239752u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2239756u32);
    emu.apc_no_count(1usize, 2239756u32, 4294959104u32, 2239760u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 25u32, 2239768u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2239772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2239796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222d34));
    } else {
        emu.pc = 2239776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222d20));
    }
}
#[inline(always)]
pub fn block_0x00222d20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2239780u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2239784u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2239788u32;
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
pub fn block_0x00222d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222cf4));
    } else {
        emu.pc = 2239792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222d30));
    }
}
#[inline(always)]
pub fn block_0x00222d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2239796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239824u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222d50));
}
#[inline(always)]
pub fn block_0x00222d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 24u32, 2239800u32);
    emu.lw_no_count(13usize, 19usize, 12u32, 2239804u32)?;
    emu.sbr_no_count(12usize, 10usize, 11usize, 2239808u32);
    emu.adr_no_count(11usize, 9usize, 11usize, 2239812u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2239816u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2239820u32;
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
pub fn block_0x00222d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222cf4));
    } else {
        emu.pc = 2239824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222d50));
    }
}
#[inline(always)]
pub fn block_0x00222d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2239828u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2239832u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(18usize);
    let return_addr = 2239836u32;
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
pub fn block_0x00222d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2239840u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2239844u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2239848u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2239852u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2239856u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2239860u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239864u32;
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
pub fn block_0x00222d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2239868u32);
    emu.lbu_no_count(12usize, 11usize, 11u32, 2239872u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2239876u32)?;
    emu.ani_no_count(12usize, 12usize, 24u32, 2239880u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2239940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222dc4));
    } else {
        emu.pc = 2239884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222d8c));
    }
}
#[inline(always)]
pub fn block_0x00222d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2239888u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2239892u32)?;
    emu.adi_no_count(10usize, 0usize, 128u32, 2239896u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2239900u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2239956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222dd4));
    } else {
        emu.pc = 2239904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222da0));
    }
}
#[inline(always)]
pub fn block_0x00222da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 2usize, 8u32, 2239908u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2239912u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2239916u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2239920u32);
    emu.apc_no_count(1usize, 2239920u32, 4294963200u32, 2239924u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2239932u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2239936u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239940u32;
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
pub fn block_0x00222dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2239944u32)?;
    emu.lw_no_count(10usize, 13usize, 0u32, 2239948u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2239952u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2239956u32;
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
pub fn block_0x00222dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 11u32, 2239960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2240020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222e14));
    } else {
        emu.pc = 2239964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ddc));
    }
}
#[inline]
pub fn block_0x00222ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 6u32, 2239968u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2239972u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2239976u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2239980u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2239984u32);
    emu.sb_no_count(11usize, 2usize, 9u32, 2239988u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2239992u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2239996u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2240000u32);
    emu.apc_no_count(1usize, 2240000u32, 4294963200u32, 2240004u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240008u32;
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
#[inline(always)]
pub fn block_0x00222e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2240012u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2240016u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240020u32;
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
pub fn block_0x00222e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 16u32, 2240024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2240100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222e64));
    } else {
        emu.pc = 2240028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222e1c));
    }
}
#[inline]
pub fn block_0x00222e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 12u32, 2240032u32);
    emu.sli_no_count(12usize, 11usize, 20u32, 2240036u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2240040u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2240044u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2240048u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2240052u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2240056u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2240060u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2240064u32);
    emu.sb_no_count(11usize, 2usize, 10u32, 2240068u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2240072u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2240076u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2240080u32);
    emu.apc_no_count(1usize, 2240080u32, 4294963200u32, 2240084u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240088u32;
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
pub fn block_0x00222e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2240092u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2240096u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240100u32;
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
pub fn block_0x00222e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 18u32, 2240104u32);
    emu.sli_no_count(12usize, 11usize, 14u32, 2240108u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2240112u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2240116u32);
    emu.adi_no_count(10usize, 10usize, 240u32, 2240120u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2240124u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2240128u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2240132u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2240136u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2240140u32);
    emu.sb_no_count(10usize, 2usize, 8u32, 2240144u32);
    emu.sb_no_count(12usize, 2usize, 9u32, 2240148u32);
    emu.sb_no_count(14usize, 2usize, 10u32, 2240152u32);
    emu.sb_no_count(11usize, 2usize, 11u32, 2240156u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2240160u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2240164u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2240168u32);
    emu.apc_no_count(1usize, 2240168u32, 4294963200u32, 2240172u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2240180u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2240184u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240188u32;
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
pub fn block_0x00222ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2240192u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2240196u32)?;
    emu.sw_no_count(8usize, 2usize, 136u32, 2240200u32)?;
    emu.sw_no_count(9usize, 2usize, 132u32, 2240204u32)?;
    emu.sw_no_count(18usize, 2usize, 128u32, 2240208u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2240212u32);
    emu.lw_no_count(12usize, 11usize, 8u32, 2240216u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2240220u32)?;
    emu.sli_no_count(10usize, 12usize, 6u32, 2240224u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2240296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f28));
    } else {
        emu.pc = 2240228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ee4));
    }
}
#[inline(always)]
pub fn block_0x00222ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2240232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2240376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f78));
    } else {
        emu.pc = 2240236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222eec));
    }
}
#[inline(always)]
pub fn block_0x00222eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2240240u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2240244u32)?;
    emu.adi_no_count(9usize, 2usize, 0u32, 2240248u32);
    emu.adi_no_count(12usize, 2usize, 0u32, 2240252u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2240256u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2240260u32);
    emu.apc_no_count(1usize, 2240260u32, 4294942720u32, 2240264u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2240272u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2240276u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2240280u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2240284u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2240288u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2240292u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2240296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222fe8));
}
#[inline(always)]
pub fn block_0x00222f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2240300u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2240304u32)?;
    emu.lw_no_count(10usize, 11usize, 4u32, 2240308u32)?;
    emu.adi_no_count(11usize, 2usize, 127u32, 2240312u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2240316u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2240320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222f68));
}
#[inline(always)]
pub fn block_0x00222f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 87u32, 2240324u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2240324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222f44));
}
#[inline]
pub fn block_0x00222f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 13usize, 4u32, 2240328u32);
    emu.sli_no_count(16usize, 10usize, 28u32, 2240332u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2240336u32);
    emu.sb_no_count(14usize, 11usize, 0u32, 2240340u32);
    emu.orr_no_count(13usize, 13usize, 16usize, 2240344u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2240348u32);
    emu.orr_no_count(14usize, 13usize, 10usize, 2240352u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2240356u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2240456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222fc8));
    } else {
        emu.pc = 2240360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f68));
    }
}
#[inline(always)]
pub fn block_0x00222f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(14usize, 13usize, 15u32, 2240364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2240320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f40));
    } else {
        emu.pc = 2240368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f70));
    }
}
#[inline(always)]
pub fn block_0x00222f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 48u32, 2240372u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2240376u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222f44));
}
#[inline(always)]
pub fn block_0x00222f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2240380u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2240384u32)?;
    emu.lw_no_count(10usize, 11usize, 4u32, 2240388u32)?;
    emu.adi_no_count(11usize, 2usize, 127u32, 2240392u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2240396u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2240400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222fb8));
}
#[inline(always)]
pub fn block_0x00222f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 55u32, 2240404u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2240404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222f94));
}
#[inline]
pub fn block_0x00222f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 13usize, 4u32, 2240408u32);
    emu.sli_no_count(16usize, 10usize, 28u32, 2240412u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2240416u32);
    emu.sb_no_count(14usize, 11usize, 0u32, 2240420u32);
    emu.orr_no_count(13usize, 13usize, 16usize, 2240424u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2240428u32);
    emu.orr_no_count(14usize, 13usize, 10usize, 2240432u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2240436u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2240456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222fc8));
    } else {
        emu.pc = 2240440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222fb8));
    }
}
#[inline(always)]
pub fn block_0x00222fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(14usize, 13usize, 15u32, 2240444u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2240400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222f90));
    } else {
        emu.pc = 2240448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222fc0));
    }
}
#[inline(always)]
pub fn block_0x00222fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 48u32, 2240452u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2240456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222f94));
}
#[inline(always)]
pub fn block_0x00222fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2240460u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2240464u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2240468u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2240472u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2240476u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2240480u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2240484u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2240488u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2240488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222fe8));
}
#[inline(always)]
pub fn block_0x00222fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2240488u32, 4294959104u32, 2240492u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240496u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00222ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2240500u32)?;
    emu.lw_no_count(8usize, 2usize, 136u32, 2240504u32)?;
    emu.lw_no_count(9usize, 2usize, 132u32, 2240508u32)?;
    emu.lw_no_count(18usize, 2usize, 128u32, 2240512u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2240516u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2240520u32;
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
pub fn block_0x00223008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2240524u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2240528u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2240532u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2240536u32;
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
pub fn block_0x00223018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2240540u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2240544u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2240548u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2240552u32);
    emu.apc_no_count(6usize, 2240552u32, 4294963200u32, 2240556u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2240560u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00223030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2240564u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2240568u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2240572u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2240576u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2240580u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2240584u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2240588u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2240592u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2240596u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2240600u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2240604u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2240608u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2240612u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2240616u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2241132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022326c));
    } else {
        emu.pc = 2240620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022306c));
    }
}
#[inline(never)]
pub fn block_0x0022306c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2240624u32);
    emu.adi_no_count(22usize, 12usize, 4294967289u32, 2240628u32);
    emu.adi_no_count(25usize, 11usize, 3u32, 2240632u32);
    emu.adi_no_count(14usize, 11usize, 4u32, 2240636u32);
    emu.sbr_no_count(15usize, 0usize, 12usize, 2240640u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2240644u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1312u32, 2240648u32);
    emu.adi_no_count(17usize, 0usize, 4u32, 2240652u32);
    emu.adi_no_count(5usize, 0usize, 240u32, 2240656u32);
    emu.adi_no_count(6usize, 0usize, 48u32, 2240660u32);
    emu.adi_no_count(7usize, 0usize, 4294967231u32, 2240664u32);
    emu.adi_no_count(28usize, 0usize, 4294967232u32, 2240668u32);
    emu.adi_no_count(29usize, 0usize, 244u32, 2240672u32);
    emu.adi_no_count(30usize, 0usize, 4294967183u32, 2240676u32);
    emu.adi_no_count(31usize, 0usize, 2u32, 2240680u32);
    emu.adi_no_count(8usize, 0usize, 3u32, 2240684u32);
    emu.adi_no_count(9usize, 0usize, 224u32, 2240688u32);
    emu.adi_no_count(18usize, 0usize, 160u32, 2240692u32);
    emu.adi_no_count(19usize, 0usize, 237u32, 2240696u32);
    emu.adi_no_count(20usize, 0usize, 4294967199u32, 2240700u32);
    emu.adi_no_count(21usize, 0usize, 12u32, 2240704u32);
    emu.sltru_no_count(24usize, 12usize, 22usize, 2240708u32);
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2240712u32);
    emu.anr_no_count(24usize, 24usize, 22usize, 2240716u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2240720u32;
    emu.update_insn_clock();
    emu.ani_no_count(25usize, 25usize, 4294967292u32, 2240724u32);
    emu.sbr_no_count(25usize, 25usize, 11usize, 2240728u32);
    emu.adi_no_count(26usize, 22usize, 128u32, 2240732u32);
    emu.add_memory_rw_events(29usize);
    let return_addr = 2240736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2240744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002230e8));
}
#[inline(always)]
pub fn block_0x002230e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2240740u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2241132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022326c));
    } else {
        emu.pc = 2240744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002230e8));
    }
}
#[inline(always)]
pub fn block_0x002230e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 11usize, 13usize, 2240748u32);
    emu.lb_no_count(22usize, 22usize, 0u32, 2240752u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2240844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022314c));
    } else {
        emu.pc = 2240756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002230f4));
    }
}
#[inline(always)]
pub fn block_0x002230f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(22usize, 25usize, 13usize, 2240760u32);
    emu.ani_no_count(22usize, 22usize, 3u32, 2240764u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2240736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002230e0));
    } else {
        emu.pc = 2240768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223100));
    }
}
#[inline(always)]
pub fn block_0x00223100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2240808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223128));
    } else {
        emu.pc = 2240772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223104));
    }
}
#[inline(always)]
pub fn block_0x00223104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 11usize, 13usize, 2240776u32);
    emu.adr_no_count(27usize, 14usize, 13usize, 2240780u32);
    emu.lw_no_count(22usize, 22usize, 0u32, 2240784u32)?;
    emu.lw_no_count(27usize, 27usize, 0u32, 2240788u32)?;
    emu.orr_no_count(22usize, 27usize, 22usize, 2240792u32);
    emu.anr_no_count(22usize, 22usize, 26usize, 2240796u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2240808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223128));
    } else {
        emu.pc = 2240800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223120));
    }
}
#[inline(always)]
pub fn block_0x00223120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2240804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2240772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223104));
    } else {
        emu.pc = 2240808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223128));
    }
}
#[inline(always)]
pub fn block_0x00223128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2240984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231d8));
    } else {
        emu.pc = 2240812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022312c));
    }
}
#[inline(always)]
pub fn block_0x0022312c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(22usize, 0usize, 13usize, 2240816u32);
    emu.adr_no_count(13usize, 11usize, 13usize, 2240820u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2240820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223134));
}
#[inline(always)]
pub fn block_0x00223134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(27usize, 13usize, 0u32, 2240824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(27usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2240980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231d4));
    } else {
        emu.pc = 2240828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022313c));
    }
}
#[inline(always)]
pub fn block_0x0022313c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2240832u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2240836u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2240820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223134));
    } else {
        emu.pc = 2240840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223148));
    }
}
#[inline(always)]
pub fn block_0x00223148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2240844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022326c));
}
#[inline(always)]
pub fn block_0x0022314c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(27usize, 22usize, 255u32, 2240848u32);
    emu.adr_no_count(22usize, 16usize, 27usize, 2240852u32);
    emu.lbu_no_count(1usize, 22usize, 0u32, 2240856u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2240860u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2240940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231ac));
    } else {
        emu.pc = 2240864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223160));
    }
}
#[inline(always)]
pub fn block_0x00223160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2240900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223184));
    } else {
        emu.pc = 2240868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223164));
    }
}
#[inline(always)]
pub fn block_0x00223164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a != b {
        emu.pc = 2241156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223284));
    } else {
        emu.pc = 2240872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223168));
    }
}
#[inline(always)]
pub fn block_0x00223168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2240876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2240880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223170));
    }
}
#[inline(always)]
pub fn block_0x00223170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 1usize, 2240884u32);
    emu.lb_no_count(23usize, 27usize, 0u32, 2240888u32);
    emu.adi_no_count(27usize, 0usize, 1u32, 2240892u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223264));
    } else {
        emu.pc = 2240896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223180));
    }
}
#[inline(always)]
pub fn block_0x00223180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2240900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223298));
}
#[inline(always)]
pub fn block_0x00223184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2240904u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2240908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022318c));
    }
}
#[inline(always)]
pub fn block_0x0022318c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(1usize, 11usize, 1usize, 2240912u32);
    emu.lbu_no_count(1usize, 1usize, 0u32, 2240916u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2240992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231e0));
    } else {
        emu.pc = 2240920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223198));
    }
}
#[inline(always)]
pub fn block_0x00223198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 1usize, 24u32, 2240924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2241016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231f8));
    } else {
        emu.pc = 2240928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231a0));
    }
}
#[inline(always)]
pub fn block_0x002231a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2240932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223214));
    } else {
        emu.pc = 2240936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231a8));
    }
}
#[inline(always)]
pub fn block_0x002231a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2240940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223284));
}
#[inline(always)]
pub fn block_0x002231ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 1u32, 2240944u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2240948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231b4));
    }
}
#[inline(always)]
pub fn block_0x002231b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(1usize, 11usize, 1usize, 2240952u32);
    emu.lbu_no_count(1usize, 1usize, 0u32, 2240956u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2241004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231ec));
    } else {
        emu.pc = 2240960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231c0));
    }
}
#[inline(always)]
pub fn block_0x002231c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(1usize, 1usize, 24u32, 2240964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2241068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022322c));
    } else {
        emu.pc = 2240968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231c8));
    }
}
#[inline(always)]
pub fn block_0x002231c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2240972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(30usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022323c));
    } else {
        emu.pc = 2240976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231d0));
    }
}
#[inline(always)]
pub fn block_0x002231d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2240980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223284));
}
#[inline(always)]
pub fn block_0x002231d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 22usize, 2240984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2240984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002231d8));
}
#[inline(always)]
pub fn block_0x002231d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2240744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002230e8));
    } else {
        emu.pc = 2240988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231dc));
    }
}
#[inline(always)]
pub fn block_0x002231dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2240992u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022326c));
}
#[inline(always)]
pub fn block_0x002231e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 1usize, 224u32, 2240996u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2241044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223214));
    } else {
        emu.pc = 2241000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231e8));
    }
}
#[inline(always)]
pub fn block_0x002231e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2241004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223284));
}
#[inline(always)]
pub fn block_0x002231ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 1usize, 4294967152u32, 2241008u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2241084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022323c));
    } else {
        emu.pc = 2241012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002231f4));
    }
}
#[inline(always)]
pub fn block_0x002231f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2241016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241156u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223284));
}
#[inline(always)]
pub fn block_0x002231f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 27usize, 4294967071u32, 2241020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2241036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022320c));
    } else {
        emu.pc = 2241024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223200));
    }
}
#[inline(always)]
pub fn block_0x00223200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 27usize, 254u32, 2241028u32);
    emu.adi_no_count(27usize, 0usize, 238u32, 2241032u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2241156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223284));
    } else {
        emu.pc = 2241036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022320c));
    }
}
#[inline(always)]
pub fn block_0x0022320c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2241040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223284));
    } else {
        emu.pc = 2241044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223214));
    }
}
#[inline(always)]
pub fn block_0x00223214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 2u32, 2241048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2241052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022321c));
    }
}
#[inline(always)]
pub fn block_0x0022321c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 11usize, 1usize, 2241056u32);
    emu.lb_no_count(23usize, 23usize, 0u32, 2241060u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223264));
    } else {
        emu.pc = 2241064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223228));
    }
}
#[inline(always)]
pub fn block_0x00223228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2241068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022328c));
}
#[inline(always)]
pub fn block_0x0022322c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 27usize, 4294967055u32, 2241072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a < b {
        emu.pc = 2241156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223284));
    } else {
        emu.pc = 2241076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223234));
    }
}
#[inline(always)]
pub fn block_0x00223234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(23usize, 1usize, 1048u32, 2241080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223284));
    } else {
        emu.pc = 2241084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022323c));
    }
}
#[inline(always)]
pub fn block_0x0022323c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 13usize, 2u32, 2241088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2241092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223244));
    }
}
#[inline(always)]
pub fn block_0x00223244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 27usize, 2241096u32);
    emu.lb_no_count(23usize, 27usize, 0u32, 2241100u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2241164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022328c));
    } else {
        emu.pc = 2241104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223250));
    }
}
#[inline(always)]
pub fn block_0x00223250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(1usize, 13usize, 3u32, 2241108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a >= b {
        emu.pc = 2241148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022327c));
    } else {
        emu.pc = 2241112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223258));
    }
}
#[inline(always)]
pub fn block_0x00223258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 11usize, 1usize, 2241116u32);
    emu.lb_no_count(23usize, 23usize, 0u32, 2241120u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(23usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223294));
    } else {
        emu.pc = 2241124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223264));
    }
}
#[inline(always)]
pub fn block_0x00223264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 1usize, 1u32, 2241128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2240744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002230e8));
    } else {
        emu.pc = 2241132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022326c));
    }
}
#[inline(always)]
pub fn block_0x0022326c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2241136u32);
    emu.sw_no_count(11usize, 10usize, 4u32, 2241140u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2241144u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2241148u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002232b0));
}
#[inline(always)]
pub fn block_0x0022327c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2241152u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2241156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223298));
}
#[inline(always)]
pub fn block_0x00223284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 1u32, 2241160u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2241164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223298));
}
#[inline(always)]
pub fn block_0x0022328c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 2u32, 2241168u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2241172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223298));
}
#[inline(always)]
pub fn block_0x00223294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 3u32, 2241176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2241176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223298));
}
#[inline(always)]
pub fn block_0x00223298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 27usize, 255u32, 2241180u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2241184u32);
    emu.orr_no_count(11usize, 22usize, 11usize, 2241188u32);
    emu.sw_no_count(13usize, 10usize, 4u32, 2241192u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2241196u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2241200u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2241200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002232b0));
}
#[inline]
pub fn block_0x002232b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 10usize, 0u32, 2241204u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2241208u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2241212u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2241216u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2241220u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2241224u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2241228u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2241232u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2241236u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2241240u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2241244u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2241248u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2241252u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2241256u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2241260u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241264u32;
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
pub fn block_0x002232f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2241268u32);
    emu.sri_no_count(6usize, 10usize, 8u32, 2241272u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2241276u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2241280u32);
    emu.ani_no_count(7usize, 10usize, 255u32, 2241284u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2241284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223304));
}
#[inline(always)]
pub fn block_0x00223304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 11usize, 0u32, 2241288u32);
    emu.lbu_no_count(28usize, 11usize, 1u32, 2241292u32);
    emu.adi_no_count(11usize, 11usize, 2u32, 2241296u32);
    emu.adr_no_count(5usize, 17usize, 28usize, 2241300u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2241340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022333c));
    } else {
        emu.pc = 2241304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223318));
    }
}
#[inline(always)]
pub fn block_0x00223318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2241448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002233a8));
    } else {
        emu.pc = 2241308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022331c));
    }
}
#[inline(always)]
pub fn block_0x0022331c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2241472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002233c0));
    } else {
        emu.pc = 2241312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223320));
    }
}
#[inline(always)]
pub fn block_0x00223320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 13usize, 17usize, 2241316u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2241316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223324));
}
#[inline(always)]
pub fn block_0x00223324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2241344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223340));
    } else {
        emu.pc = 2241320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223328));
    }
}
#[inline(always)]
pub fn block_0x00223328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(29usize, 17usize, 0u32, 2241324u32);
    emu.adi_no_count(17usize, 17usize, 1u32, 2241328u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2241332u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2241316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223324));
    } else {
        emu.pc = 2241336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223338));
    }
}
#[inline(always)]
pub fn block_0x00223338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2241340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002233a0));
}
#[inline(always)]
pub fn block_0x0022333c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2241352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223348));
    } else {
        emu.pc = 2241344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223340));
    }
}
#[inline(always)]
pub fn block_0x00223340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 5usize, 0u32, 2241348u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2241284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223304));
    } else {
        emu.pc = 2241352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223348));
    }
}
#[inline(always)]
pub fn block_0x00223348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 15usize, 16usize, 2241356u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2241360u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2241360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223350));
}
#[inline(always)]
pub fn block_0x00223350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 15usize, 0u32, 2241364u32);
    emu.adi_no_count(13usize, 15usize, 1u32, 2241368u32);
    emu.ani_no_count(12usize, 14usize, 255u32, 2241372u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2241392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223370));
    } else {
        emu.pc = 2241376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223360));
    }
}
#[inline(always)]
pub fn block_0x00223360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2241380u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2241384u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2241424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223390));
    } else {
        emu.pc = 2241388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022336c));
    }
}
#[inline(always)]
pub fn block_0x0022336c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2241392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2241432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223398));
}
#[inline(always)]
pub fn block_0x00223370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2241496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002233d8));
    } else {
        emu.pc = 2241396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223374));
    }
}
#[inline(always)]
pub fn block_0x00223374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 15usize, 1u32, 2241400u32);
    emu.adi_no_count(15usize, 15usize, 2u32, 2241404u32);
    emu.ani_no_count(12usize, 12usize, 127u32, 2241408u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2241412u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2241416u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2241420u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2241432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223398));
    } else {
        emu.pc = 2241424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223390));
    }
}
#[inline(always)]
pub fn block_0x00223390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(11usize, 11usize, 1u32, 2241428u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2241360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223350));
    } else {
        emu.pc = 2241432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223398));
    }
}
#[inline(always)]
pub fn block_0x00223398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2241436u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241440u32;
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
pub fn block_0x002233a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 0usize, 1u32, 2241444u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241448u32;
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
pub fn block_0x002233a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2241452u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2241456u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2241460u32);
    emu.adi_no_count(11usize, 5usize, 0u32, 2241464u32);
    emu.apc_no_count(1usize, 2241464u32, 8192u32, 2241468u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002233c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2241476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2241480u32);
    emu.adi_no_count(10usize, 5usize, 0u32, 2241484u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2241488u32);
    emu.apc_no_count(1usize, 2241488u32, 8192u32, 2241492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002233d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2241500u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1584u32, 2241504u32);
    emu.apc_no_count(1usize, 2241504u32, 4294950912u32, 2241508u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
