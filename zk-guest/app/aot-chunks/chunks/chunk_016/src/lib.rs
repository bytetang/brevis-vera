pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2155164u32;
pub const PC_MAX: u32 = 2158060u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 136usize] = [
        block_0x0020e29c,
        block_0x0020e2dc,
        block_0x0020e2e0,
        block_0x0020e2f8,
        block_0x0020e300,
        block_0x0020e30c,
        block_0x0020e320,
        block_0x0020e324,
        block_0x0020e330,
        block_0x0020e34c,
        block_0x0020e49c,
        block_0x0020e5f0,
        block_0x0020e5f4,
        block_0x0020e600,
        block_0x0020e60c,
        block_0x0020e61c,
        block_0x0020e630,
        block_0x0020e640,
        block_0x0020e644,
        block_0x0020e64c,
        block_0x0020e678,
        block_0x0020e684,
        block_0x0020e6d8,
        block_0x0020e6f8,
        block_0x0020e6fc,
        block_0x0020e700,
        block_0x0020e708,
        block_0x0020e714,
        block_0x0020e724,
        block_0x0020e72c,
        block_0x0020e738,
        block_0x0020e73c,
        block_0x0020e744,
        block_0x0020e754,
        block_0x0020e768,
        block_0x0020e794,
        block_0x0020e79c,
        block_0x0020e7a0,
        block_0x0020e7a4,
        block_0x0020e7ac,
        block_0x0020e7b4,
        block_0x0020e7bc,
        block_0x0020e7d4,
        block_0x0020e7dc,
        block_0x0020e7e4,
        block_0x0020e7ec,
        block_0x0020e7f4,
        block_0x0020e7fc,
        block_0x0020e804,
        block_0x0020e80c,
        block_0x0020e818,
        block_0x0020e828,
        block_0x0020e830,
        block_0x0020e840,
        block_0x0020e848,
        block_0x0020e860,
        block_0x0020e868,
        block_0x0020e86c,
        block_0x0020e878,
        block_0x0020e888,
        block_0x0020e890,
        block_0x0020e898,
        block_0x0020e8a4,
        block_0x0020e8b8,
        block_0x0020e8c0,
        block_0x0020e8cc,
        block_0x0020e8dc,
        block_0x0020e8e4,
        block_0x0020e8f4,
        block_0x0020e8fc,
        block_0x0020e918,
        block_0x0020e92c,
        block_0x0020e93c,
        block_0x0020e944,
        block_0x0020e968,
        block_0x0020e970,
        block_0x0020e978,
        block_0x0020e988,
        block_0x0020e9b4,
        block_0x0020e9b8,
        block_0x0020e9c4,
        block_0x0020e9d4,
        block_0x0020e9e0,
        block_0x0020e9f0,
        block_0x0020e9f8,
        block_0x0020ea34,
        block_0x0020ea38,
        block_0x0020ea4c,
        block_0x0020ea5c,
        block_0x0020ea64,
        block_0x0020ea88,
        block_0x0020ea90,
        block_0x0020ea98,
        block_0x0020eaa8,
        block_0x0020ead4,
        block_0x0020ead8,
        block_0x0020eae4,
        block_0x0020eaf4,
        block_0x0020eafc,
        block_0x0020eb04,
        block_0x0020eb40,
        block_0x0020eb44,
        block_0x0020eb58,
        block_0x0020eb68,
        block_0x0020eb70,
        block_0x0020eb78,
        block_0x0020eb8c,
        block_0x0020eba0,
        block_0x0020eba4,
        block_0x0020ebc4,
        block_0x0020ebdc,
        block_0x0020ebf8,
        block_0x0020ec0c,
        block_0x0020ec1c,
        block_0x0020ec20,
        block_0x0020ec34,
        block_0x0020ec44,
        block_0x0020ec4c,
        block_0x0020ec70,
        block_0x0020ec78,
        block_0x0020ec80,
        block_0x0020ec88,
        block_0x0020ecbc,
        block_0x0020ecc0,
        block_0x0020eccc,
        block_0x0020ecdc,
        block_0x0020ece8,
        block_0x0020ecf4,
        block_0x0020ed10,
        block_0x0020ed18,
        block_0x0020ed44,
        block_0x0020ed88,
        block_0x0020ed8c,
        block_0x0020edc8,
        block_0x0020eddc,
        block_0x0020edec,
    ];
    const IDX: [u16; 725usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 9u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16,
        19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        21u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 26u16, 0u16, 27u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 31u16, 32u16, 0u16,
        33u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 38u16, 39u16, 0u16,
        40u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        44u16, 0u16, 45u16, 0u16, 46u16, 0u16, 47u16, 0u16, 48u16, 0u16, 49u16, 0u16,
        50u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 58u16,
        0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16,
        0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16,
        0u16, 97u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 100u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16,
        113u16, 0u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16,
        0u16, 0u16, 117u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        119u16, 0u16, 120u16, 0u16, 121u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16, 125u16,
        0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 127u16, 0u16, 0u16, 128u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 129u16, 0u16, 130u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 131u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 132u16, 133u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 134u16, 0u16,
        0u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 136u16,
    ];
    if pc < 2155164u32 || pc > 2158060u32 {
        return None;
    }
    let word_offset = ((pc - 2155164u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020e29c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2155168u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2155172u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2155176u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2155180u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2155184u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2155188u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2155192u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2155196u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2155200u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2155204u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2155208u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2155212u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2155216u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2155220u32)?;
    emu.adi_no_count(20usize, 12usize, 0u32, 2155224u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2f8));
    } else {
        emu.pc = 2155228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2dc));
    }
}
#[inline(always)]
pub fn block_0x0020e2dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2155232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2e0));
}
#[inline(always)]
pub fn block_0x0020e2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2155236u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967032u32, 2155240u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2155244u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2155248u32);
    emu.apc_no_count(1usize, 2155248u32, 0u32, 2155252u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155256u32;
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
pub fn block_0x0020e2f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2155260u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2156032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e600));
    } else {
        emu.pc = 2155264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e300));
    }
}
#[inline(always)]
pub fn block_0x0020e300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2155268u32);
    emu.apc_no_count(1usize, 2155268u32, 4294930432u32, 2155272u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155276u32;
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
pub fn block_0x0020e30c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2155280u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2155284u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2155288u32);
    emu.apc_no_count(1usize, 2155288u32, 4294918144u32, 2155292u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2155232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2e0));
    } else {
        emu.pc = 2155300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e324));
    }
}
#[inline(always)]
pub fn block_0x0020e324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2155304u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2155308u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2156044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e60c));
    } else {
        emu.pc = 2155312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e330));
    }
}
#[inline(always)]
pub fn block_0x0020e330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 16u32, 2155316u32)?;
    emu.adi_no_count(13usize, 0usize, 0u32, 2155320u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2155324u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2155328u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967280u32, 2155332u32);
    emu.anr_no_count(10usize, 20usize, 10usize, 2155336u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2155340u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2155340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e34c));
}
#[inline(never)]
pub fn block_0x0020e34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 84u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 9usize, 11usize, 2155344u32);
    emu.lbu_no_count(26usize, 27usize, 0u32, 2155348u32);
    emu.lbu_no_count(25usize, 27usize, 1u32, 2155352u32);
    emu.lbu_no_count(24usize, 27usize, 2u32, 2155356u32);
    emu.lbu_no_count(23usize, 27usize, 3u32, 2155360u32);
    emu.lbu_no_count(22usize, 27usize, 4u32, 2155364u32);
    emu.lbu_no_count(21usize, 27usize, 5u32, 2155368u32);
    emu.lbu_no_count(19usize, 27usize, 6u32, 2155372u32);
    emu.lbu_no_count(31usize, 27usize, 7u32, 2155376u32);
    emu.lbu_no_count(30usize, 27usize, 8u32, 2155380u32);
    emu.lbu_no_count(29usize, 27usize, 9u32, 2155384u32);
    emu.lbu_no_count(28usize, 27usize, 10u32, 2155388u32);
    emu.lbu_no_count(7usize, 27usize, 11u32, 2155392u32);
    emu.lbu_no_count(6usize, 27usize, 12u32, 2155396u32);
    emu.lbu_no_count(5usize, 27usize, 13u32, 2155400u32);
    emu.lbu_no_count(17usize, 27usize, 14u32, 2155404u32);
    emu.lbu_no_count(15usize, 27usize, 15u32, 2155408u32);
    emu.xri_no_count(16usize, 26usize, 4294967295u32, 2155412u32);
    emu.xri_no_count(1usize, 25usize, 4294967295u32, 2155416u32);
    emu.xri_no_count(8usize, 24usize, 4294967295u32, 2155420u32);
    emu.xri_no_count(10usize, 23usize, 4294967295u32, 2155424u32);
    emu.xri_no_count(14usize, 22usize, 4294967295u32, 2155428u32);
    emu.xri_no_count(12usize, 21usize, 4294967295u32, 2155432u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2155436u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2155440u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2155444u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2155448u32);
    emu.adr_no_count(16usize, 1usize, 16usize, 2155452u32);
    emu.xri_no_count(1usize, 31usize, 4294967295u32, 2155456u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2155460u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2155464u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2155468u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2155472u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2155476u32);
    emu.xri_no_count(8usize, 30usize, 4294967295u32, 2155480u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2155484u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2155488u32);
    emu.sri_no_count(14usize, 14usize, 31u32, 2155492u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2155496u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2155500u32);
    emu.xri_no_count(14usize, 7usize, 4294967295u32, 2155504u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2155508u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2155512u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2155516u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2155520u32);
    emu.adr_no_count(8usize, 1usize, 8usize, 2155524u32);
    emu.xri_no_count(1usize, 6usize, 4294967295u32, 2155528u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2155532u32);
    emu.sli_no_count(1usize, 1usize, 24u32, 2155536u32);
    emu.sri_no_count(14usize, 14usize, 31u32, 2155540u32);
    emu.sri_no_count(1usize, 1usize, 31u32, 2155544u32);
    emu.adr_no_count(14usize, 14usize, 1usize, 2155548u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2155552u32);
    emu.xri_no_count(16usize, 19usize, 4294967295u32, 2155556u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2155560u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2155564u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2155568u32);
    emu.xri_no_count(16usize, 29usize, 4294967295u32, 2155572u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2155576u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2155580u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2155584u32);
    emu.xri_no_count(8usize, 5usize, 4294967295u32, 2155588u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2155592u32);
    emu.sri_no_count(8usize, 8usize, 31u32, 2155596u32);
    emu.adr_no_count(14usize, 14usize, 8usize, 2155600u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2155604u32);
    emu.xri_no_count(12usize, 28usize, 4294967295u32, 2155608u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2155612u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2155616u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2155620u32);
    emu.xri_no_count(16usize, 17usize, 4294967295u32, 2155624u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2155628u32);
    emu.sri_no_count(16usize, 16usize, 31u32, 2155632u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2155636u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2155640u32);
    emu.xri_no_count(12usize, 15usize, 4294967295u32, 2155644u32);
    emu.sli_no_count(12usize, 12usize, 24u32, 2155648u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2155652u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2155656u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2155660u32);
    emu.ani_no_count(10usize, 10usize, 255u32, 2155664u32);
    emu.adr_no_count(16usize, 18usize, 11usize, 2155668u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2155672u32);
    emu.add_memory_rw_events(83usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e61c));
    } else {
        emu.pc = 2155676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e49c));
    }
}
#[inline(never)]
pub fn block_0x0020e49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 26usize, 4294967199u32, 2155680u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155684u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155688u32);
    emu.xrr_no_count(26usize, 10usize, 26usize, 2155692u32);
    emu.adi_no_count(10usize, 25usize, 4294967199u32, 2155696u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155700u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155704u32);
    emu.xrr_no_count(25usize, 10usize, 25usize, 2155708u32);
    emu.adi_no_count(10usize, 24usize, 4294967199u32, 2155712u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155716u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155720u32);
    emu.xrr_no_count(24usize, 10usize, 24usize, 2155724u32);
    emu.adi_no_count(10usize, 23usize, 4294967199u32, 2155728u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155732u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155736u32);
    emu.xrr_no_count(23usize, 10usize, 23usize, 2155740u32);
    emu.adi_no_count(10usize, 22usize, 4294967199u32, 2155744u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155748u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155752u32);
    emu.xrr_no_count(22usize, 10usize, 22usize, 2155756u32);
    emu.adi_no_count(10usize, 21usize, 4294967199u32, 2155760u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155764u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155768u32);
    emu.xrr_no_count(27usize, 10usize, 21usize, 2155772u32);
    emu.adi_no_count(10usize, 19usize, 4294967199u32, 2155776u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155780u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155784u32);
    emu.xrr_no_count(19usize, 10usize, 19usize, 2155788u32);
    emu.adi_no_count(10usize, 31usize, 4294967199u32, 2155792u32);
    emu.sltiu_no_count(10usize, 10usize, 26u32, 2155796u32);
    emu.sli_no_count(10usize, 10usize, 5u32, 2155800u32);
    emu.xrr_no_count(10usize, 10usize, 31usize, 2155804u32);
    emu.adi_no_count(12usize, 30usize, 4294967199u32, 2155808u32);
    emu.sltiu_no_count(12usize, 12usize, 26u32, 2155812u32);
    emu.sli_no_count(12usize, 12usize, 5u32, 2155816u32);
    emu.xrr_no_count(12usize, 12usize, 30usize, 2155820u32);
    emu.adi_no_count(14usize, 29usize, 4294967199u32, 2155824u32);
    emu.sltiu_no_count(14usize, 14usize, 26u32, 2155828u32);
    emu.sli_no_count(14usize, 14usize, 5u32, 2155832u32);
    emu.xrr_no_count(14usize, 14usize, 29usize, 2155836u32);
    emu.adi_no_count(29usize, 28usize, 4294967199u32, 2155840u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155844u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155848u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2155852u32);
    emu.adi_no_count(29usize, 7usize, 4294967199u32, 2155856u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155860u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155864u32);
    emu.xrr_no_count(7usize, 29usize, 7usize, 2155868u32);
    emu.adi_no_count(29usize, 6usize, 4294967199u32, 2155872u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155876u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155880u32);
    emu.xrr_no_count(6usize, 29usize, 6usize, 2155884u32);
    emu.adi_no_count(29usize, 5usize, 4294967199u32, 2155888u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155892u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155896u32);
    emu.xrr_no_count(5usize, 29usize, 5usize, 2155900u32);
    emu.adi_no_count(29usize, 17usize, 4294967199u32, 2155904u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155908u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155912u32);
    emu.xrr_no_count(17usize, 29usize, 17usize, 2155916u32);
    emu.adi_no_count(29usize, 15usize, 4294967199u32, 2155920u32);
    emu.adi_no_count(13usize, 13usize, 4294967280u32, 2155924u32);
    emu.sltiu_no_count(29usize, 29usize, 26u32, 2155928u32);
    emu.sli_no_count(29usize, 29usize, 5u32, 2155932u32);
    emu.xrr_no_count(15usize, 29usize, 15usize, 2155936u32);
    emu.adr_no_count(21usize, 20usize, 13usize, 2155940u32);
    emu.sb_no_count(26usize, 16usize, 0u32, 2155944u32);
    emu.sb_no_count(25usize, 16usize, 1u32, 2155948u32);
    emu.sb_no_count(24usize, 16usize, 2u32, 2155952u32);
    emu.sb_no_count(23usize, 16usize, 3u32, 2155956u32);
    emu.sb_no_count(22usize, 16usize, 4u32, 2155960u32);
    emu.sb_no_count(27usize, 16usize, 5u32, 2155964u32);
    emu.sb_no_count(19usize, 16usize, 6u32, 2155968u32);
    emu.sb_no_count(10usize, 16usize, 7u32, 2155972u32);
    emu.sb_no_count(12usize, 16usize, 8u32, 2155976u32);
    emu.sb_no_count(14usize, 16usize, 9u32, 2155980u32);
    emu.sb_no_count(28usize, 16usize, 10u32, 2155984u32);
    emu.sb_no_count(7usize, 16usize, 11u32, 2155988u32);
    emu.sb_no_count(6usize, 16usize, 12u32, 2155992u32);
    emu.sb_no_count(5usize, 16usize, 13u32, 2155996u32);
    emu.sb_no_count(17usize, 16usize, 14u32, 2156000u32);
    emu.sb_no_count(15usize, 16usize, 15u32, 2156004u32);
    emu.adi_no_count(11usize, 11usize, 16u32, 2156008u32);
    emu.adi_no_count(10usize, 0usize, 15u32, 2156012u32);
    emu.add_memory_rw_events(84usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2155340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e34c));
    } else {
        emu.pc = 2156016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5f0));
    }
}
#[inline(always)]
pub fn block_0x0020e5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2156080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e630));
    } else {
        emu.pc = 2156020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5f4));
    }
}
#[inline(always)]
pub fn block_0x0020e5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 16u32, 2156024u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2156028u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2156032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e678));
}
#[inline(always)]
pub fn block_0x0020e600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2156036u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2156040u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2156044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e678));
}
#[inline(always)]
pub fn block_0x0020e60c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2156048u32);
    emu.adi_no_count(21usize, 20usize, 0u32, 2156052u32);
    emu.adi_no_count(16usize, 18usize, 0u32, 2156056u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2156060u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e640));
}
#[inline(always)]
pub fn block_0x0020e61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 20usize, 13usize, 2156064u32);
    emu.adi_no_count(9usize, 27usize, 0u32, 2156068u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2156072u32);
    emu.lw_no_count(8usize, 2usize, 16u32, 2156076u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2156080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e640));
}
#[inline(always)]
pub fn block_0x0020e630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 9usize, 11usize, 2156084u32);
    emu.adr_no_count(16usize, 18usize, 11usize, 2156088u32);
    emu.lw_no_count(8usize, 2usize, 16u32, 2156092u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2156096u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2156096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e640));
}
#[inline(always)]
pub fn block_0x0020e640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 19usize, 21usize, 2156100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e644));
}
#[inline(always)]
pub fn block_0x0020e644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 9usize, 0u32, 2156104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6d8));
    } else {
        emu.pc = 2156108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e64c));
    }
}
#[inline]
pub fn block_0x0020e64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 255u32, 2156112u32);
    emu.adi_no_count(19usize, 19usize, 1u32, 2156116u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2156120u32);
    emu.adi_no_count(9usize, 9usize, 1u32, 2156124u32);
    emu.adi_no_count(11usize, 10usize, 4294967199u32, 2156128u32);
    emu.sltiu_no_count(11usize, 11usize, 26u32, 2156132u32);
    emu.sli_no_count(11usize, 11usize, 5u32, 2156136u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2156140u32);
    emu.sb_no_count(10usize, 16usize, 0u32, 2156144u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2156148u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2156100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e644));
    } else {
        emu.pc = 2156152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e678));
    }
}
#[inline(always)]
pub fn block_0x0020e678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 20u32, 2156156u32)?;
    emu.sw_no_count(18usize, 2usize, 24u32, 2156160u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2156164u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2156164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e684));
}
#[inline]
pub fn block_0x0020e684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2156168u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2156172u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2156176u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2156180u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2156184u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2156188u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2156192u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2156196u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2156200u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2156204u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2156208u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2156212u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2156216u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2156220u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2156224u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2156228u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2156232u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2156236u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2156240u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2156244u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156248u32;
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
pub fn block_0x0020e6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 20u32, 2156252u32)?;
    emu.sw_no_count(18usize, 2usize, 24u32, 2156256u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2156260u32)?;
    emu.adr_no_count(21usize, 9usize, 21usize, 2156264u32);
    emu.adi_no_count(22usize, 0usize, 128u32, 2156268u32);
    emu.adi_no_count(23usize, 0usize, 224u32, 2156272u32);
    emu.adi_no_count(24usize, 0usize, 240u32, 2156276u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2156280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e708));
}
#[inline(always)]
pub fn block_0x0020e6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(25usize, 10usize, 0u32, 2156284u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6fc));
}
#[inline(always)]
pub fn block_0x0020e6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 19usize, 2156288u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline(always)]
pub fn block_0x0020e700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 28u32, 2156292u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2156164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e684));
    } else {
        emu.pc = 2156296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e708));
    }
}
#[inline(always)]
pub fn block_0x0020e708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 9usize, 0u32, 2156300u32);
    emu.ani_no_count(11usize, 10usize, 255u32, 2156304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e744));
    } else {
        emu.pc = 2156308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e714));
    }
}
#[inline(always)]
pub fn block_0x0020e714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 1u32, 2156312u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2156316u32);
    emu.apc_no_count(1usize, 2156316u32, 4096u32, 2156320u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2156328u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2156508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7dc));
    } else {
        emu.pc = 2156332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e72c));
    }
}
#[inline(always)]
pub fn block_0x0020e72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 40u32, 2156336u32)?;
    emu.lw_no_count(27usize, 2usize, 32u32, 2156340u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2156448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7a0));
    } else {
        emu.pc = 2156344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e738));
    }
}
#[inline(always)]
pub fn block_0x0020e738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2156460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7ac));
    } else {
        emu.pc = 2156348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e73c));
    }
}
#[inline(always)]
pub fn block_0x0020e73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2156352u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e818));
}
#[inline(always)]
pub fn block_0x0020e744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 9usize, 1u32, 2156360u32);
    emu.ani_no_count(10usize, 11usize, 31u32, 2156364u32);
    emu.ani_no_count(12usize, 12usize, 63u32, 2156368u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2156476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7bc));
    } else {
        emu.pc = 2156372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e754));
    }
}
#[inline(always)]
pub fn block_0x0020e754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 9usize, 2u32, 2156376u32);
    emu.sli_no_count(12usize, 12usize, 6u32, 2156380u32);
    emu.ani_no_count(13usize, 13usize, 63u32, 2156384u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2156388u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2156616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e848));
    } else {
        emu.pc = 2156392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e768));
    }
}
#[inline]
pub fn block_0x0020e768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 3u32, 2156396u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2156400u32);
    emu.sli_no_count(10usize, 10usize, 29u32, 2156404u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2156408u32);
    emu.sli_no_count(12usize, 12usize, 6u32, 2156412u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2156416u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2156420u32);
    emu.orr_no_count(11usize, 11usize, 10usize, 2156424u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2156428u32);
    emu.apc_no_count(1usize, 2156428u32, 4096u32, 2156432u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2156440u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2156332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e72c));
    } else {
        emu.pc = 2156444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e79c));
    }
}
#[inline(always)]
pub fn block_0x0020e79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7dc));
}
#[inline(always)]
pub fn block_0x0020e7a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2156540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7fc));
    } else {
        emu.pc = 2156452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7a4));
    }
}
#[inline(always)]
pub fn block_0x0020e7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2156456u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8cc));
}
#[inline(always)]
pub fn block_0x0020e7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2156464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e80c));
    } else {
        emu.pc = 2156468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7b4));
    }
}
#[inline(always)]
pub fn block_0x0020e7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2156472u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e818));
}
#[inline(always)]
pub fn block_0x0020e7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 2u32, 2156480u32);
    emu.sli_no_count(10usize, 10usize, 6u32, 2156484u32);
    emu.orr_no_count(11usize, 10usize, 12usize, 2156488u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2156492u32);
    emu.apc_no_count(1usize, 2156492u32, 4096u32, 2156496u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2156504u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2156332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e72c));
    } else {
        emu.pc = 2156508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7dc));
    }
}
#[inline(always)]
pub fn block_0x0020e7dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 32u32, 2156512u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7ec));
    } else {
        emu.pc = 2156516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7e4));
    }
}
#[inline(always)]
pub fn block_0x0020e7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2156520u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e878));
}
#[inline(always)]
pub fn block_0x0020e7ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2156528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e86c));
    } else {
        emu.pc = 2156532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7f4));
    }
}
#[inline(always)]
pub fn block_0x0020e7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2156536u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e878));
}
#[inline(always)]
pub fn block_0x0020e7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2156544u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8c0));
    } else {
        emu.pc = 2156548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e804));
    }
}
#[inline(always)]
pub fn block_0x0020e804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2156552u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8cc));
}
#[inline(always)]
pub fn block_0x0020e80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2156560u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2156564u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2156568u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2156568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e818));
}
#[inline(always)]
pub fn block_0x0020e818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2156572u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2156576u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2156580u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2156824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e918));
    } else {
        emu.pc = 2156584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e828));
    }
}
#[inline(always)]
pub fn block_0x0020e828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 18usize, 10usize, 2156588u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2156860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e93c));
    } else {
        emu.pc = 2156592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e830));
    }
}
#[inline(always)]
pub fn block_0x0020e830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(27usize, 18usize, 0u32, 2156596u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2156600u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2156604u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e968));
    } else {
        emu.pc = 2156608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e840));
    }
}
#[inline(always)]
pub fn block_0x0020e840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2156612u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9c4));
}
#[inline(always)]
pub fn block_0x0020e848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 3u32, 2156620u32);
    emu.sli_no_count(10usize, 10usize, 12u32, 2156624u32);
    emu.orr_no_count(11usize, 12usize, 10usize, 2156628u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2156632u32);
    emu.apc_no_count(1usize, 2156632u32, 4096u32, 2156636u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156640u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020e860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2156644u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2156332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e72c));
    } else {
        emu.pc = 2156648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e868));
    }
}
#[inline(always)]
pub fn block_0x0020e868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7dc));
}
#[inline(always)]
pub fn block_0x0020e86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2156656u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2156660u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2156664u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2156664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e878));
}
#[inline(always)]
pub fn block_0x0020e878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2156668u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2156672u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2156676u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2157560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebf8));
    } else {
        emu.pc = 2156680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e888));
    }
}
#[inline(always)]
pub fn block_0x0020e888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 10usize, 2156684u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6f8));
    } else {
        emu.pc = 2156688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e890));
    }
}
#[inline(always)]
pub fn block_0x0020e890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2156692u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2156708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8a4));
    } else {
        emu.pc = 2156696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e898));
    }
}
#[inline(always)]
pub fn block_0x0020e898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2156700u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2156704u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2156708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb78));
}
#[inline(always)]
pub fn block_0x0020e8a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 16u32, 2156712u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2156716u32);
    emu.ani_no_count(11usize, 25usize, 63u32, 2156720u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2156724u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2156796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8fc));
    } else {
        emu.pc = 2156728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8b8));
    }
}
#[inline(always)]
pub fn block_0x0020e8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 12u32, 2156732u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eba4));
}
#[inline(always)]
pub fn block_0x0020e8c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2156740u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2156744u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2156748u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2156748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8cc));
}
#[inline(always)]
pub fn block_0x0020e8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2156752u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2156756u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2156760u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2157600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec20));
    } else {
        emu.pc = 2156764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8dc));
    }
}
#[inline(always)]
pub fn block_0x0020e8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 18usize, 10usize, 2156768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2157636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec44));
    } else {
        emu.pc = 2156772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8e4));
    }
}
#[inline(always)]
pub fn block_0x0020e8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(27usize, 18usize, 0u32, 2156776u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2156780u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2156784u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2157680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec70));
    } else {
        emu.pc = 2156788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8f4));
    }
}
#[inline(always)]
pub fn block_0x0020e8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2156792u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eccc));
}
#[inline(always)]
pub fn block_0x0020e8fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 18u32, 2156800u32);
    emu.sli_no_count(25usize, 25usize, 14u32, 2156804u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2156808u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2156812u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2156816u32);
    emu.sri_no_count(14usize, 25usize, 26u32, 2156820u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2156824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebdc));
}
#[inline(always)]
pub fn block_0x0020e918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2156828u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2156832u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2156836u32);
    emu.apc_no_count(1usize, 2156836u32, 4294963200u32, 2156840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020e92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2156848u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2156852u32)?;
    emu.adr_no_count(18usize, 18usize, 10usize, 2156856u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a < b {
        emu.pc = 2156592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e830));
    } else {
        emu.pc = 2156860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e93c));
    }
}
#[inline(always)]
pub fn block_0x0020e93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2156864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e978));
    } else {
        emu.pc = 2156868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e944));
    }
}
#[inline]
pub fn block_0x0020e944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 6u32, 2156872u32);
    emu.ani_no_count(11usize, 27usize, 63u32, 2156876u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2156880u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2156884u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2156888u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2156892u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2156896u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2156900u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e840));
    } else {
        emu.pc = 2156904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e968));
    }
}
#[inline(always)]
pub fn block_0x0020e968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2156908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2156984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b8));
    } else {
        emu.pc = 2156912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e970));
    }
}
#[inline(always)]
pub fn block_0x0020e970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2156916u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156996u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9c4));
}
#[inline(always)]
pub fn block_0x0020e978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 16u32, 2156924u32);
    emu.sli_no_count(11usize, 27usize, 20u32, 2156928u32);
    emu.ani_no_count(10usize, 27usize, 63u32, 2156932u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2157048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f8));
    } else {
        emu.pc = 2156936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e988));
    }
}
#[inline]
pub fn block_0x0020e988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 12u32, 2156940u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2156944u32);
    emu.adi_no_count(10usize, 10usize, 128u32, 2156948u32);
    emu.ori_no_count(12usize, 12usize, 224u32, 2156952u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2156956u32);
    emu.sb_no_count(12usize, 18usize, 0u32, 2156960u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2156964u32);
    emu.sb_no_count(10usize, 18usize, 2u32, 2156968u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2156972u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2156976u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e968));
    } else {
        emu.pc = 2156980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b4));
    }
}
#[inline(always)]
pub fn block_0x0020e9b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e840));
}
#[inline(always)]
pub fn block_0x0020e9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2156988u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2156992u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2156996u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2156996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9c4));
}
#[inline(always)]
pub fn block_0x0020e9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2157000u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2157004u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2157008u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2157112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea38));
    } else {
        emu.pc = 2157012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9d4));
    }
}
#[inline(always)]
pub fn block_0x0020e9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2157016u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2157020u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2157148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea5c));
    } else {
        emu.pc = 2157024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e0));
    }
}
#[inline(always)]
pub fn block_0x0020e9e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(25usize, 10usize, 0u32, 2157028u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157032u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157036u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2157192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea88));
    } else {
        emu.pc = 2157040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f0));
    }
}
#[inline(always)]
pub fn block_0x0020e9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2157044u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157048u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eae4));
}
#[inline]
pub fn block_0x0020e9f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 27usize, 18u32, 2157052u32);
    emu.sli_no_count(27usize, 27usize, 14u32, 2157056u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2157060u32);
    emu.adi_no_count(10usize, 10usize, 128u32, 2157064u32);
    emu.adi_no_count(12usize, 12usize, 240u32, 2157068u32);
    emu.sri_no_count(13usize, 27usize, 26u32, 2157072u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157076u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2157080u32);
    emu.sb_no_count(12usize, 18usize, 0u32, 2157084u32);
    emu.sb_no_count(13usize, 18usize, 1u32, 2157088u32);
    emu.sb_no_count(11usize, 18usize, 2u32, 2157092u32);
    emu.sb_no_count(10usize, 18usize, 3u32, 2157096u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157100u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157104u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e840));
    } else {
        emu.pc = 2157108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea34));
    }
}
#[inline(always)]
pub fn block_0x0020ea34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e968));
}
#[inline(always)]
pub fn block_0x0020ea38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2157116u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2157120u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2157124u32);
    emu.apc_no_count(1usize, 2157124u32, 4294963200u32, 2157128u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ea4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2157136u32)?;
    emu.lw_no_count(18usize, 2usize, 24u32, 2157140u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2157144u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2157024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e0));
    } else {
        emu.pc = 2157148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea5c));
    }
}
#[inline(always)]
pub fn block_0x0020ea5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2157152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2157208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea98));
    } else {
        emu.pc = 2157156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea64));
    }
}
#[inline]
pub fn block_0x0020ea64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2157160u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2157164u32);
    emu.ori_no_count(11usize, 11usize, 192u32, 2157168u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157172u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2157176u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157180u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157184u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157188u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2157040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f0));
    } else {
        emu.pc = 2157192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea88));
    }
}
#[inline(always)]
pub fn block_0x0020ea88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 26usize, 11u32, 2157196u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead8));
    } else {
        emu.pc = 2157200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea90));
    }
}
#[inline(always)]
pub fn block_0x0020ea90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2157204u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eae4));
}
#[inline(always)]
pub fn block_0x0020ea98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 16u32, 2157212u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2157216u32);
    emu.ani_no_count(11usize, 25usize, 63u32, 2157220u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2157316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb04));
    } else {
        emu.pc = 2157224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaa8));
    }
}
#[inline]
pub fn block_0x0020eaa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 12u32, 2157228u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157232u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157236u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2157240u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157244u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2157248u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157252u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2157256u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157260u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157264u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2157192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea88));
    } else {
        emu.pc = 2157268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead4));
    }
}
#[inline(always)]
pub fn block_0x0020ead4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157040u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9f0));
}
#[inline(always)]
pub fn block_0x0020ead8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 26usize, 16u32, 2157276u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2157280u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2157284u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2157284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eae4));
}
#[inline(always)]
pub fn block_0x0020eae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2157288u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2157292u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2157296u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2157380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb44));
    } else {
        emu.pc = 2157300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaf4));
    }
}
#[inline(always)]
pub fn block_0x0020eaf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 10usize, 2157304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2157416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb68));
    } else {
        emu.pc = 2157308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eafc));
    }
}
#[inline(always)]
pub fn block_0x0020eafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(26usize, 10usize, 0u32, 2157312u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6fc));
}
#[inline]
pub fn block_0x0020eb04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 18u32, 2157320u32);
    emu.sli_no_count(25usize, 25usize, 14u32, 2157324u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157328u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157332u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2157336u32);
    emu.sri_no_count(14usize, 25usize, 26u32, 2157340u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157344u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2157348u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2157352u32);
    emu.sb_no_count(14usize, 10usize, 1u32, 2157356u32);
    emu.sb_no_count(12usize, 10usize, 2u32, 2157360u32);
    emu.sb_no_count(11usize, 10usize, 3u32, 2157364u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157368u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157372u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2157040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f0));
    } else {
        emu.pc = 2157376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb40));
    }
}
#[inline(always)]
pub fn block_0x0020eb40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea88));
}
#[inline(always)]
pub fn block_0x0020eb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2157384u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2157388u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2157392u32);
    emu.apc_no_count(1usize, 2157392u32, 4294963200u32, 2157396u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157400u32;
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
pub fn block_0x0020eb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2157404u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2157408u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2157412u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2157308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eafc));
    } else {
        emu.pc = 2157416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb68));
    }
}
#[inline(always)]
pub fn block_0x0020eb68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 26usize, 11u32, 2157420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2157452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb8c));
    } else {
        emu.pc = 2157424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb70));
    }
}
#[inline(always)]
pub fn block_0x0020eb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 26usize, 6u32, 2157428u32);
    emu.ani_no_count(12usize, 26usize, 63u32, 2157432u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb78));
}
#[inline(always)]
pub fn block_0x0020eb78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ori_no_count(11usize, 11usize, 192u32, 2157436u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157440u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2157444u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157448u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2157452u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6fc));
}
#[inline(always)]
pub fn block_0x0020eb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 16u32, 2157456u32);
    emu.sli_no_count(12usize, 26usize, 20u32, 2157460u32);
    emu.ani_no_count(11usize, 26usize, 63u32, 2157464u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157468u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2157508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebc4));
    } else {
        emu.pc = 2157472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba0));
    }
}
#[inline(always)]
pub fn block_0x0020eba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 12u32, 2157476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eba4));
}
#[inline(always)]
pub fn block_0x0020eba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 12usize, 26u32, 2157480u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157484u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2157488u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157492u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2157496u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157500u32);
    emu.sb_no_count(11usize, 10usize, 2u32, 2157504u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2157508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline(always)]
pub fn block_0x0020ebc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 26usize, 18u32, 2157512u32);
    emu.sli_no_count(26usize, 26usize, 14u32, 2157516u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157520u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157524u32);
    emu.adi_no_count(13usize, 13usize, 240u32, 2157528u32);
    emu.sri_no_count(14usize, 26usize, 26u32, 2157532u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2157532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebdc));
}
#[inline(always)]
pub fn block_0x0020ebdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 128u32, 2157536u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2157540u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2157544u32);
    emu.sb_no_count(14usize, 10usize, 1u32, 2157548u32);
    emu.sb_no_count(12usize, 10usize, 2u32, 2157552u32);
    emu.sb_no_count(11usize, 10usize, 3u32, 2157556u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2157560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline(always)]
pub fn block_0x0020ebf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2157564u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2157568u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2157572u32);
    emu.apc_no_count(1usize, 2157572u32, 4294963200u32, 2157576u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ec0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2157584u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2157588u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2157592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e890));
    } else {
        emu.pc = 2157596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec1c));
    }
}
#[inline(always)]
pub fn block_0x0020ec1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157600u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156280u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e6f8));
}
#[inline(always)]
pub fn block_0x0020ec20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2157604u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2157608u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2157612u32);
    emu.apc_no_count(1usize, 2157612u32, 4294963200u32, 2157616u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ec34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2157624u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2157628u32)?;
    emu.adr_no_count(18usize, 18usize, 10usize, 2157632u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a < b {
        emu.pc = 2156772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8e4));
    } else {
        emu.pc = 2157636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec44));
    }
}
#[inline(always)]
pub fn block_0x0020ec44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 11u32, 2157640u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec80));
    } else {
        emu.pc = 2157644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec4c));
    }
}
#[inline]
pub fn block_0x0020ec4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 6u32, 2157648u32);
    emu.ani_no_count(11usize, 27usize, 63u32, 2157652u32);
    emu.ori_no_count(10usize, 10usize, 192u32, 2157656u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157660u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2157664u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2157668u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157672u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157676u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8f4));
    } else {
        emu.pc = 2157680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec70));
    }
}
#[inline(always)]
pub fn block_0x0020ec70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 11u32, 2157684u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecc0));
    } else {
        emu.pc = 2157688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec78));
    }
}
#[inline(always)]
pub fn block_0x0020ec78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2157692u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eccc));
}
#[inline(always)]
pub fn block_0x0020ec80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 16u32, 2157700u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed44));
    } else {
        emu.pc = 2157704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec88));
    }
}
#[inline]
pub fn block_0x0020ec88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 12u32, 2157708u32);
    emu.sli_no_count(11usize, 27usize, 20u32, 2157712u32);
    emu.ani_no_count(12usize, 27usize, 63u32, 2157716u32);
    emu.ori_no_count(10usize, 10usize, 224u32, 2157720u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2157724u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157728u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157732u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2157736u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2157740u32);
    emu.sb_no_count(12usize, 18usize, 2u32, 2157744u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157748u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157752u32)?;
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8f4));
    } else {
        emu.pc = 2157756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecbc));
    }
}
#[inline(always)]
pub fn block_0x0020ecbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157680u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec70));
}
#[inline(always)]
pub fn block_0x0020ecc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 25usize, 16u32, 2157764u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2157768u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2157772u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2157772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eccc));
}
#[inline(always)]
pub fn block_0x0020eccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2157776u32)?;
    emu.sbr_no_count(11usize, 10usize, 19usize, 2157780u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2157784u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2158024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edc8));
    } else {
        emu.pc = 2157788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecdc));
    }
}
#[inline(always)]
pub fn block_0x0020ecdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 24u32, 2157792u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2157796u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6f8));
    } else {
        emu.pc = 2157800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece8));
    }
}
#[inline(always)]
pub fn block_0x0020ece8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 11u32, 2157804u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157808u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2157840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed10));
    } else {
        emu.pc = 2157812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecf4));
    }
}
#[inline(always)]
pub fn block_0x0020ecf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 6u32, 2157816u32);
    emu.ani_no_count(12usize, 25usize, 63u32, 2157820u32);
    emu.ori_no_count(11usize, 11usize, 192u32, 2157824u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157828u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2157832u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157836u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2157840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline(always)]
pub fn block_0x0020ed10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 16u32, 2157844u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2157964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed8c));
    } else {
        emu.pc = 2157848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed18));
    }
}
#[inline]
pub fn block_0x0020ed18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 12u32, 2157852u32);
    emu.sli_no_count(12usize, 25usize, 20u32, 2157856u32);
    emu.ani_no_count(13usize, 25usize, 63u32, 2157860u32);
    emu.ori_no_count(11usize, 11usize, 224u32, 2157864u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157868u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2157872u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157876u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2157880u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2157884u32);
    emu.sb_no_count(13usize, 10usize, 2u32, 2157888u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2157892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline]
pub fn block_0x0020ed44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 27usize, 18u32, 2157896u32);
    emu.sli_no_count(11usize, 27usize, 14u32, 2157900u32);
    emu.sli_no_count(12usize, 27usize, 20u32, 2157904u32);
    emu.ani_no_count(13usize, 27usize, 63u32, 2157908u32);
    emu.adi_no_count(10usize, 10usize, 240u32, 2157912u32);
    emu.sri_no_count(11usize, 11usize, 26u32, 2157916u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157920u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2157924u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2157928u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2157932u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2157936u32);
    emu.sb_no_count(11usize, 18usize, 1u32, 2157940u32);
    emu.sb_no_count(12usize, 18usize, 2u32, 2157944u32);
    emu.sb_no_count(13usize, 18usize, 3u32, 2157948u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2157952u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2157956u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2157680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec70));
    } else {
        emu.pc = 2157960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed88));
    }
}
#[inline(always)]
pub fn block_0x0020ed88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e8f4));
}
#[inline]
pub fn block_0x0020ed8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 18u32, 2157968u32);
    emu.sli_no_count(12usize, 25usize, 14u32, 2157972u32);
    emu.sli_no_count(13usize, 25usize, 20u32, 2157976u32);
    emu.ani_no_count(14usize, 25usize, 63u32, 2157980u32);
    emu.adi_no_count(11usize, 11usize, 240u32, 2157984u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2157988u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2157992u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2157996u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2158000u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2158004u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2158008u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2158012u32);
    emu.sb_no_count(13usize, 10usize, 2u32, 2158016u32);
    emu.sb_no_count(14usize, 10usize, 3u32, 2158020u32);
    emu.add_memory_rw_events(15usize);
    let return_addr = 2158024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e700));
}
#[inline(always)]
pub fn block_0x0020edc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2158028u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2158032u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2158036u32);
    emu.apc_no_count(1usize, 2158036u32, 4294963200u32, 2158040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020eddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2158048u32)?;
    emu.lw_no_count(18usize, 2usize, 24u32, 2158052u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2158056u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2156280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e6f8));
    } else {
        emu.pc = 2158060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edec));
    }
}
#[inline(always)]
pub fn block_0x0020edec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ece8));
}
