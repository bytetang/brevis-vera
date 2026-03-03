pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2184288u32;
pub const PC_MAX: u32 = 2186668u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 116usize] = [
        block_0x00215460,
        block_0x00215468,
        block_0x00215470,
        block_0x00215478,
        block_0x00215480,
        block_0x00215488,
        block_0x002154bc,
        block_0x002154c4,
        block_0x002154f8,
        block_0x002155d8,
        block_0x002155fc,
        block_0x00215608,
        block_0x0021560c,
        block_0x00215630,
        block_0x00215658,
        block_0x0021565c,
        block_0x00215664,
        block_0x0021566c,
        block_0x00215674,
        block_0x0021567c,
        block_0x00215688,
        block_0x0021568c,
        block_0x002156b4,
        block_0x002156dc,
        block_0x002156e0,
        block_0x002156e8,
        block_0x002156f0,
        block_0x002156f4,
        block_0x002156fc,
        block_0x00215708,
        block_0x0021570c,
        block_0x00215724,
        block_0x0021574c,
        block_0x00215750,
        block_0x00215758,
        block_0x00215760,
        block_0x00215764,
        block_0x0021576c,
        block_0x00215780,
        block_0x0021578c,
        block_0x002157a8,
        block_0x002157b0,
        block_0x002157c8,
        block_0x002157d0,
        block_0x002157e8,
        block_0x002157ec,
        block_0x002157f4,
        block_0x00215808,
        block_0x00215814,
        block_0x00215830,
        block_0x00215838,
        block_0x00215850,
        block_0x00215858,
        block_0x00215870,
        block_0x00215874,
        block_0x0021587c,
        block_0x00215890,
        block_0x0021589c,
        block_0x002158b8,
        block_0x002158c0,
        block_0x002158d8,
        block_0x002158e0,
        block_0x002158f8,
        block_0x002158fc,
        block_0x00215904,
        block_0x00215918,
        block_0x00215924,
        block_0x00215940,
        block_0x00215948,
        block_0x00215960,
        block_0x00215968,
        block_0x00215980,
        block_0x00215984,
        block_0x0021598c,
        block_0x002159a0,
        block_0x002159ac,
        block_0x002159c8,
        block_0x002159d0,
        block_0x002159e8,
        block_0x002159f0,
        block_0x00215a08,
        block_0x00215a0c,
        block_0x00215a1c,
        block_0x00215a38,
        block_0x00215a4c,
        block_0x00215a64,
        block_0x00215a7c,
        block_0x00215acc,
        block_0x00215ae0,
        block_0x00215af0,
        block_0x00215b04,
        block_0x00215b08,
        block_0x00215b0c,
        block_0x00215b10,
        block_0x00215b18,
        block_0x00215b1c,
        block_0x00215b20,
        block_0x00215b28,
        block_0x00215b60,
        block_0x00215bc8,
        block_0x00215c24,
        block_0x00215c94,
        block_0x00215cc4,
        block_0x00215cf4,
        block_0x00215d08,
        block_0x00215d18,
        block_0x00215d28,
        block_0x00215d38,
        block_0x00215d48,
        block_0x00215d4c,
        block_0x00215d5c,
        block_0x00215d74,
        block_0x00215d78,
        block_0x00215d94,
        block_0x00215da4,
        block_0x00215dac,
    ];
    const IDX: [u16; 596usize] = [
        1u16, 0u16, 2u16, 0u16, 3u16, 0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 17u16,
        0u16, 18u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16, 21u16, 22u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 27u16, 28u16, 0u16,
        29u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 35u16, 0u16, 36u16,
        37u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 40u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16, 47u16, 0u16, 0u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 64u16, 0u16,
        65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        81u16, 82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16,
        0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16,
        93u16, 94u16, 0u16, 95u16, 96u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 108u16, 0u16,
        0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 112u16, 113u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16,
        0u16, 115u16, 0u16, 116u16,
    ];
    if pc < 2184288u32 || pc > 2186668u32 {
        return None;
    }
    let word_offset = ((pc - 2184288u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00215460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2184292u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2184304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215470));
    } else {
        emu.pc = 2184296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215468));
    }
}
#[inline(always)]
pub fn block_0x00215468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2184300u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2184304u32;
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
pub fn block_0x00215470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 127u32, 2184308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2184320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215480));
    } else {
        emu.pc = 2184312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215478));
    }
}
#[inline(always)]
pub fn block_0x00215478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2184316u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2184320u32;
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
pub fn block_0x00215480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 16u32, 2184324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2184380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002154bc));
    } else {
        emu.pc = 2184328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215488));
    }
}
#[inline]
pub fn block_0x00215488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2184332u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2184336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965454u32, 2184340u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2184344u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965534u32, 2184348u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2184352u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294965824u32, 2184356u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2184360u32);
    emu.adi_no_count(14usize, 0usize, 290u32, 2184364u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2184368u32);
    emu.adi_no_count(16usize, 0usize, 297u32, 2184372u32);
    emu.apc_no_count(6usize, 2184372u32, 0u32, 2184376u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2184380u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002154bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 17u32, 2184384u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2184440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002154f8));
    } else {
        emu.pc = 2184388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002154c4));
    }
}
#[inline]
pub fn block_0x002154c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2184392u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2184396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1472u32, 2184400u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2184404u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1560u32, 2184408u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2184412u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1768u32, 2184416u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2184420u32);
    emu.adi_no_count(14usize, 0usize, 208u32, 2184424u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2184428u32);
    emu.adi_no_count(16usize, 0usize, 486u32, 2184432u32);
    emu.apc_no_count(6usize, 2184432u32, 0u32, 2184436u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2184440u32;
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
#[inline(never)]
pub fn block_0x002154f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2184444u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(172032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2184448u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294791168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2184452u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294782976u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2184456u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294774784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2184460u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294770688u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2184464u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294766592u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2184468u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294049792u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2184472u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2184476u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967264u32, 2184480u32);
    emu.adi_no_count(12usize, 12usize, 1760u32, 2184484u32);
    emu.adi_no_count(13usize, 13usize, 4294965440u32, 2184488u32);
    emu.adi_no_count(14usize, 14usize, 336u32, 2184492u32);
    emu.anr_no_count(7usize, 10usize, 11usize, 2184496u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2184500u32);
    emu.adi_no_count(7usize, 15usize, 1040u32, 2184504u32);
    emu.adi_no_count(15usize, 15usize, 4294965248u32, 2184508u32);
    emu.adr_no_count(16usize, 10usize, 16usize, 2184512u32);
    emu.adi_no_count(17usize, 17usize, 4294966448u32, 2184516u32);
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2184520u32);
    emu.adi_no_count(6usize, 6usize, 496u32, 2184524u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2184528u32);
    emu.adi_no_count(11usize, 11usize, 30u32, 2184532u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2184536u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2184540u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2184544u32);
    emu.adr_no_count(17usize, 10usize, 17usize, 2184548u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2184552u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2184556u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2184560u32);
    let a = 0u32.wrapping_add(4294963200u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2184564u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1630u32, 2184568u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2184572u32);
    let a = 0u32.wrapping_add(4294254592u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2184576u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 688u32, 2184580u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2184584u32);
    let a = 0u32.wrapping_add(180224u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2184588u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965278u32, 2184592u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2184596u32);
    emu.sltiu_no_count(14usize, 14usize, 4294967282u32, 2184600u32);
    emu.sltiu_no_count(5usize, 7usize, 4294967281u32, 2184604u32);
    emu.anr_no_count(14usize, 14usize, 5usize, 2184608u32);
    emu.sltiu_no_count(17usize, 17usize, 4294967291u32, 2184612u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2184616u32);
    emu.sltiu_no_count(13usize, 13usize, 4294967290u32, 2184620u32);
    emu.sltru_no_count(12usize, 0usize, 12usize, 2184624u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2184628u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2184632u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2184636u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2184640u32);
    emu.sltiu_no_count(12usize, 16usize, 4294965790u32, 2184644u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2184648u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2184652u32);
    emu.anr_no_count(11usize, 15usize, 6usize, 2184656u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2184660u32);
    emu.add_memory_rw_events(56usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2184664u32;
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
pub fn block_0x002155d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967120u32, 2184668u32);
    emu.sw_no_count(1usize, 2usize, 172u32, 2184672u32)?;
    emu.sw_no_count(8usize, 2usize, 168u32, 2184676u32)?;
    emu.sw_no_count(9usize, 2usize, 164u32, 2184680u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2184684u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2184688u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2184692u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2184696u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2184820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215674));
    } else {
        emu.pc = 2184700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002155fc));
    }
}
#[inline(always)]
pub fn block_0x002155fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2184704u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2184708u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2185784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a38));
    } else {
        emu.pc = 2184712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215608));
    }
}
#[inline(always)]
pub fn block_0x00215608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2184812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021566c));
    } else {
        emu.pc = 2184716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021560c));
    }
}
#[inline]
pub fn block_0x0021560c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2184720u32);
    emu.sli_no_count(9usize, 9usize, 2u32, 2184724u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2184728u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966124u32, 2184732u32);
    emu.adr_no_count(11usize, 11usize, 9usize, 2184736u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2184740u32)?;
    emu.sli_no_count(14usize, 10usize, 2u32, 2184744u32);
    emu.adr_no_count(11usize, 8usize, 14usize, 2184748u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2184752u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2184752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215630));
}
#[inline]
pub fn block_0x00215630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2184756u32)?;
    emu.mulhu_no_count(17usize, 16usize, 13usize, 2184760u32);
    emu.mul_no_count(16usize, 16usize, 13usize, 2184764u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2184768u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2184772u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2184776u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2184780u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2184784u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2184788u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2184752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215630));
    } else {
        emu.pc = 2184792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215658));
    }
}
#[inline(always)]
pub fn block_0x00215658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2184812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021566c));
    } else {
        emu.pc = 2184796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021565c));
    }
}
#[inline(always)]
pub fn block_0x0021565c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2184800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a64));
    } else {
        emu.pc = 2184804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215664));
    }
}
#[inline(always)]
pub fn block_0x00215664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2184808u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2184812u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2184812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021566c));
}
#[inline(always)]
pub fn block_0x0021566c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2184816u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2184820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215a1c));
}
#[inline(always)]
pub fn block_0x00215674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 9usize, 7u32, 2184824u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2184948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156f4));
    } else {
        emu.pc = 2184828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021567c));
    }
}
#[inline(always)]
pub fn block_0x0021567c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2184832u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2184836u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2185784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a38));
    } else {
        emu.pc = 2184840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215688));
    }
}
#[inline(always)]
pub fn block_0x00215688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2184944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156f0));
    } else {
        emu.pc = 2184844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021568c));
    }
}
#[inline]
pub fn block_0x0021568c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2184848u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2184852u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2184856u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966124u32, 2184860u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2184864u32);
    emu.lw_no_count(14usize, 13usize, 0u32, 2184868u32)?;
    emu.sli_no_count(13usize, 10usize, 2u32, 2184872u32);
    emu.srr_no_count(14usize, 14usize, 12usize, 2184876u32);
    emu.adr_no_count(12usize, 8usize, 13usize, 2184880u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2184884u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2184884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002156b4));
}
#[inline]
pub fn block_0x002156b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2184888u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2184892u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2184896u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2184900u32);
    emu.sw_no_count(11usize, 15usize, 0u32, 2184904u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2184908u32);
    emu.sltru_no_count(11usize, 11usize, 16usize, 2184912u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2184916u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2184920u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2184884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156b4));
    } else {
        emu.pc = 2184924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156dc));
    }
}
#[inline(always)]
pub fn block_0x002156dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2184944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156f0));
    } else {
        emu.pc = 2184928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156e0));
    }
}
#[inline(always)]
pub fn block_0x002156e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2184932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a64));
    } else {
        emu.pc = 2184936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156e8));
    }
}
#[inline(always)]
pub fn block_0x002156e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 12usize, 0u32, 2184940u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2184944u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2184944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002156f0));
}
#[inline(always)]
pub fn block_0x002156f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2184948u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2184948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002156f4));
}
#[inline(always)]
pub fn block_0x002156f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 8u32, 2184952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215764));
    } else {
        emu.pc = 2184956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002156fc));
    }
}
#[inline(always)]
pub fn block_0x002156fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2184960u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2184964u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2185784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a38));
    } else {
        emu.pc = 2184968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215708));
    }
}
#[inline(always)]
pub fn block_0x00215708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2185056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215760));
    } else {
        emu.pc = 2184972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021570c));
    }
}
#[inline(always)]
pub fn block_0x0021570c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2184976u32);
    emu.sli_no_count(13usize, 10usize, 2u32, 2184980u32);
    let a = 0u32.wrapping_add(389120u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2184984u32;
    emu.update_insn_clock();
    emu.adr_no_count(11usize, 8usize, 13usize, 2184988u32);
    emu.adi_no_count(14usize, 14usize, 1505u32, 2184992u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2184996u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2184996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215724));
}
#[inline]
pub fn block_0x00215724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2185000u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2185004u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2185008u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2185012u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2185016u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2185020u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2185024u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2185028u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2185032u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2184996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215724));
    } else {
        emu.pc = 2185036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021574c));
    }
}
#[inline(always)]
pub fn block_0x0021574c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2185056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215760));
    } else {
        emu.pc = 2185040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215750));
    }
}
#[inline(always)]
pub fn block_0x00215750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2185044u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a64));
    } else {
        emu.pc = 2185048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215758));
    }
}
#[inline(always)]
pub fn block_0x00215758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2185052u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2185056u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2185056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215760));
}
#[inline(always)]
pub fn block_0x00215760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2185060u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215764));
}
#[inline(always)]
pub fn block_0x00215764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 16u32, 2185064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002157ec));
    } else {
        emu.pc = 2185068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021576c));
    }
}
#[inline(always)]
pub fn block_0x0021576c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2185072u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185076u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2185080u32);
    emu.apc_no_count(1usize, 2185080u32, 4294901760u32, 2185084u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2185092u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2185096u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002157a8));
    } else {
        emu.pc = 2185100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021578c));
    }
}
#[inline(always)]
pub fn block_0x0021578c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2185104u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966164u32, 2185108u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185112u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2185116u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2185120u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2185124u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2185128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185160u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002157c8));
}
#[inline(always)]
pub fn block_0x002157a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2185132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a4c));
    } else {
        emu.pc = 2185136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002157b0));
    }
}
#[inline(always)]
pub fn block_0x002157b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2185140u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966164u32, 2185144u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185148u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2185152u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2185156u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2185160u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2185160u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002157c8));
}
#[inline(always)]
pub fn block_0x002157c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2185160u32, 4294946816u32, 2185164u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002157d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2185172u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2185176u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185180u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2185184u32);
    emu.apc_no_count(1usize, 2185184u32, 4294901760u32, 2185188u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002157e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2185196u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002157ec));
}
#[inline(always)]
pub fn block_0x002157ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 32u32, 2185200u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215874));
    } else {
        emu.pc = 2185204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002157f4));
    }
}
#[inline(always)]
pub fn block_0x002157f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2185208u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185212u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2185216u32);
    emu.apc_no_count(1usize, 2185216u32, 4294901760u32, 2185220u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2185228u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2185232u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215830));
    } else {
        emu.pc = 2185236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215814));
    }
}
#[inline(always)]
pub fn block_0x00215814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2185240u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966172u32, 2185244u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185248u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2185252u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2185256u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2185260u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2185264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215850));
}
#[inline(always)]
pub fn block_0x00215830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2185268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a4c));
    } else {
        emu.pc = 2185272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215838));
    }
}
#[inline(always)]
pub fn block_0x00215838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2185276u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966172u32, 2185280u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185284u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2185288u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2185292u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2185296u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2185296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215850));
}
#[inline(always)]
pub fn block_0x00215850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2185296u32, 4294946816u32, 2185300u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2185308u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2185312u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185316u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2185320u32);
    emu.apc_no_count(1usize, 2185320u32, 4294901760u32, 2185324u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185328u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2185332u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215874));
}
#[inline(always)]
pub fn block_0x00215874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 64u32, 2185336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002158fc));
    } else {
        emu.pc = 2185340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021587c));
    }
}
#[inline(always)]
pub fn block_0x0021587c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2185344u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185348u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2185352u32);
    emu.apc_no_count(1usize, 2185352u32, 4294901760u32, 2185356u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185360u32;
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
pub fn block_0x00215890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2185364u32)?;
    emu.adi_no_count(10usize, 0usize, 5u32, 2185368u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002158b8));
    } else {
        emu.pc = 2185372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021589c));
    }
}
#[inline(always)]
pub fn block_0x0021589c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2185376u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966184u32, 2185380u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185384u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2185388u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2185392u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2185396u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2185400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002158d8));
}
#[inline(always)]
pub fn block_0x002158b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2185404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a4c));
    } else {
        emu.pc = 2185408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002158c0));
    }
}
#[inline(always)]
pub fn block_0x002158c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2185412u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966184u32, 2185416u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185420u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2185424u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2185428u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2185432u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2185432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002158d8));
}
#[inline(always)]
pub fn block_0x002158d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2185432u32, 4294946816u32, 2185436u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002158e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2185444u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2185448u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185452u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2185456u32);
    emu.apc_no_count(1usize, 2185456u32, 4294901760u32, 2185460u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002158f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2185468u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002158fc));
}
#[inline(always)]
pub fn block_0x002158fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 128u32, 2185472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215984));
    } else {
        emu.pc = 2185476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215904));
    }
}
#[inline(always)]
pub fn block_0x00215904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2185480u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185484u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2185488u32);
    emu.apc_no_count(1usize, 2185488u32, 4294901760u32, 2185492u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2185500u32)?;
    emu.adi_no_count(10usize, 0usize, 10u32, 2185504u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215940));
    } else {
        emu.pc = 2185508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215924));
    }
}
#[inline(always)]
pub fn block_0x00215924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2185512u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966204u32, 2185516u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185520u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2185524u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2185528u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2185532u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2185536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215960));
}
#[inline(always)]
pub fn block_0x00215940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2185540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a4c));
    } else {
        emu.pc = 2185544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215948));
    }
}
#[inline(always)]
pub fn block_0x00215948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2185548u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966204u32, 2185552u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185556u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2185560u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2185564u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2185568u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2185568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215960));
}
#[inline(always)]
pub fn block_0x00215960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2185568u32, 4294946816u32, 2185572u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2185580u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2185584u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185588u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2185592u32);
    emu.apc_no_count(1usize, 2185592u32, 4294901760u32, 2185596u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2185604u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215984));
}
#[inline(always)]
pub fn block_0x00215984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 256u32, 2185608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2185740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a0c));
    } else {
        emu.pc = 2185612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021598c));
    }
}
#[inline(always)]
pub fn block_0x0021598c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2185616u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185620u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2185624u32);
    emu.apc_no_count(1usize, 2185624u32, 4294901760u32, 2185628u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002159a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2185636u32)?;
    emu.adi_no_count(10usize, 0usize, 19u32, 2185640u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002159c8));
    } else {
        emu.pc = 2185644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002159ac));
    }
}
#[inline(always)]
pub fn block_0x002159ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2185648u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966244u32, 2185652u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185656u32);
    emu.adi_no_count(14usize, 0usize, 19u32, 2185660u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2185664u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2185668u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2185672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2185704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002159e8));
}
#[inline(always)]
pub fn block_0x002159c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2185676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2185804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215a4c));
    } else {
        emu.pc = 2185680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002159d0));
    }
}
#[inline(always)]
pub fn block_0x002159d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2185684u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966244u32, 2185688u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2185692u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2185696u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2185700u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2185704u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2185704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002159e8));
}
#[inline(always)]
pub fn block_0x002159e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2185704u32, 4294946816u32, 2185708u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185712u32;
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
pub fn block_0x002159f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2185716u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2185720u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2185724u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2185728u32);
    emu.apc_no_count(1usize, 2185728u32, 4294901760u32, 2185732u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215a08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2185740u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215a0c));
}
#[inline(always)]
pub fn block_0x00215a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2185744u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2185748u32);
    emu.apc_no_count(1usize, 2185748u32, 4294942720u32, 2185752u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2185760u32);
    emu.lw_no_count(1usize, 2usize, 172u32, 2185764u32)?;
    emu.lw_no_count(8usize, 2usize, 168u32, 2185768u32)?;
    emu.lw_no_count(9usize, 2usize, 164u32, 2185772u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2185776u32)?;
    emu.adi_no_count(2usize, 2usize, 176u32, 2185780u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185784u32;
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
pub fn block_0x00215a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2185788u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2185792u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2185796u32);
    emu.apc_no_count(1usize, 2185796u32, 8192u32, 2185800u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185804u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00215a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2185808u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2185812u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2185816u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2185820u32);
    emu.apc_no_count(1usize, 2185820u32, 8192u32, 2185824u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2185832u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2185836u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2185840u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2185844u32);
    emu.apc_no_count(1usize, 2185844u32, 4294942720u32, 2185848u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2185852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00215a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965904u32, 2185856u32);
    emu.sw_no_count(1usize, 2usize, 1388u32, 2185860u32)?;
    emu.sw_no_count(8usize, 2usize, 1384u32, 2185864u32)?;
    emu.sw_no_count(9usize, 2usize, 1380u32, 2185868u32)?;
    emu.sw_no_count(18usize, 2usize, 1376u32, 2185872u32)?;
    emu.sw_no_count(19usize, 2usize, 1372u32, 2185876u32)?;
    emu.sw_no_count(20usize, 2usize, 1368u32, 2185880u32)?;
    emu.sw_no_count(21usize, 2usize, 1364u32, 2185884u32)?;
    emu.sw_no_count(22usize, 2usize, 1360u32, 2185888u32)?;
    emu.sw_no_count(23usize, 2usize, 1356u32, 2185892u32)?;
    emu.sw_no_count(24usize, 2usize, 1352u32, 2185896u32)?;
    emu.sw_no_count(25usize, 2usize, 1348u32, 2185900u32)?;
    emu.sw_no_count(26usize, 2usize, 1344u32, 2185904u32)?;
    emu.sw_no_count(27usize, 2usize, 1340u32, 2185908u32)?;
    emu.adi_no_count(24usize, 13usize, 0u32, 2185912u32);
    emu.sw_no_count(12usize, 2usize, 20u32, 2185916u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2185920u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2185924u32)?;
    emu.orr_no_count(14usize, 12usize, 13usize, 2185928u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2189340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189340u32));
    } else {
        emu.pc = 2185932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215acc));
    }
}
#[inline(always)]
pub fn block_0x00215acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 10usize, 0u32, 2185936u32);
    emu.lw_no_count(18usize, 11usize, 8u32, 2185940u32)?;
    emu.lw_no_count(20usize, 11usize, 12u32, 2185944u32)?;
    emu.orr_no_count(10usize, 18usize, 20usize, 2185948u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2189368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189368u32));
    } else {
        emu.pc = 2185952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215ae0));
    }
}
#[inline(always)]
pub fn block_0x00215ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 16u32, 2185956u32)?;
    emu.lw_no_count(9usize, 11usize, 20u32, 2185960u32)?;
    emu.orr_no_count(10usize, 8usize, 9usize, 2185964u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2189396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189396u32));
    } else {
        emu.pc = 2185968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215af0));
    }
}
#[inline(always)]
pub fn block_0x00215af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 8usize, 2185972u32);
    emu.sltru_no_count(15usize, 10usize, 12usize, 2185976u32);
    emu.adr_no_count(14usize, 13usize, 9usize, 2185980u32);
    emu.adr_no_count(16usize, 14usize, 15usize, 2185984u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2185992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b08));
    } else {
        emu.pc = 2185988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b04));
    }
}
#[inline(always)]
pub fn block_0x00215b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2185992u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2185992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215b08));
}
#[inline(always)]
pub fn block_0x00215b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2189424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189424u32));
    } else {
        emu.pc = 2185996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b0c));
    }
}
#[inline(always)]
pub fn block_0x00215b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2186008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b18));
    } else {
        emu.pc = 2186000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b10));
    }
}
#[inline(always)]
pub fn block_0x00215b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 13usize, 20usize, 2186004u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2186008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2186012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215b1c));
}
#[inline(always)]
pub fn block_0x00215b18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 18usize, 2186012u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2186012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215b1c));
}
#[inline(always)]
pub fn block_0x00215b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2189452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189452u32));
    } else {
        emu.pc = 2186016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b20));
    }
}
#[inline(always)]
pub fn block_0x00215b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 16u32, 2186020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2189480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2189480u32));
    } else {
        emu.pc = 2186024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b28));
    }
}
#[inline]
pub fn block_0x00215b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(19usize, 11usize, 24u32, 2186028u32)?;
    emu.sltru_no_count(15usize, 10usize, 12usize, 2186032u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2186036u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2186040u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2186044u32;
    emu.update_insn_clock();
    emu.adr_no_count(15usize, 14usize, 15usize, 2186048u32);
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2186052u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 16usize, 1365u32, 2186056u32);
    emu.adi_no_count(16usize, 6usize, 819u32, 2186060u32);
    emu.adi_no_count(14usize, 14usize, 4294967055u32, 2186064u32);
    emu.sbr_no_count(5usize, 15usize, 5usize, 2186068u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2186072u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 257u32, 2186076u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2186184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215bc8));
    } else {
        emu.pc = 2186080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215b60));
    }
}
#[inline(never)]
pub fn block_0x00215b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2186084u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2186088u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186092u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2186096u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186100u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2186104u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186108u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2186112u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186116u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2186120u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186124u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2186128u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2186132u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2186136u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2186140u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2186144u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2186148u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2186152u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2186156u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2186160u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2186164u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2186168u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2186172u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2186176u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2186180u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2186184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2186276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215c24));
}
#[inline]
pub fn block_0x00215bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 5usize, 1u32, 2186188u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2186192u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2186196u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186200u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2186204u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186208u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2186212u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186216u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2186220u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2186224u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2186228u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2186232u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2186236u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2186240u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2186244u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2186248u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2186252u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2186256u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2186260u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2186264u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2186268u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2186272u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2186276u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2186276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215c24));
}
#[inline(never)]
pub fn block_0x00215c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 11usize, 26u32, 2186280u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2186284u32)?;
    emu.sbr_no_count(10usize, 19usize, 10usize, 2186288u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2186292u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2186296u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2186300u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2186304u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2186308u32);
    emu.adi_no_count(11usize, 14usize, 4294967295u32, 2186312u32);
    emu.anr_no_count(11usize, 11usize, 13usize, 2186316u32);
    emu.adi_no_count(22usize, 0usize, 2u32, 2186320u32);
    emu.sbr_no_count(13usize, 22usize, 14usize, 2186324u32);
    emu.sw_no_count(13usize, 2usize, 188u32, 2186328u32)?;
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2186332u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2186336u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2186340u32);
    emu.sw_no_count(12usize, 2usize, 28u32, 2186344u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2186348u32)?;
    emu.sltru_no_count(10usize, 13usize, 10usize, 2186352u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2186356u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2186360u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2186364u32);
    emu.sai_no_count(25usize, 10usize, 1040u32, 2186368u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2186372u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2186376u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2186380u32);
    emu.apc_no_count(1usize, 2186380u32, 4294901760u32, 2186384u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00215c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 20usize, 1u32, 2186392u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2186396u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2186400u32);
    emu.anr_no_count(11usize, 11usize, 20usize, 2186404u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2186408u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2186412u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2186416u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2186420u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2186424u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2186428u32);
    emu.apc_no_count(1usize, 2186428u32, 4294901760u32, 2186432u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00215cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 9usize, 1u32, 2186440u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2186444u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2186448u32);
    emu.anr_no_count(11usize, 11usize, 9usize, 2186452u32);
    emu.sw_no_count(10usize, 2usize, 516u32, 2186456u32)?;
    emu.sw_no_count(8usize, 2usize, 356u32, 2186460u32)?;
    emu.sw_no_count(11usize, 2usize, 360u32, 2186464u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2186468u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2186472u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2186476u32);
    emu.apc_no_count(1usize, 2186476u32, 4294901760u32, 2186480u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 524u32, 2186488u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2186492u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2186496u32);
    emu.apc_no_count(1usize, 2186496u32, 4294901760u32, 2186500u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2186508u32);
    emu.sw_no_count(10usize, 2usize, 680u32, 2186512u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2186516u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2186588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d5c));
    } else {
        emu.pc = 2186520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d18));
    }
}
#[inline(always)]
pub fn block_0x00215d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2186524u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186528u32);
    emu.apc_no_count(1usize, 2186528u32, 4294942720u32, 2186532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186536u32;
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
pub fn block_0x00215d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2186540u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186544u32);
    emu.apc_no_count(1usize, 2186544u32, 4294942720u32, 2186548u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186552u32;
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
pub fn block_0x00215d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2186556u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186560u32);
    emu.apc_no_count(1usize, 2186560u32, 4294942720u32, 2186564u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2186616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d78));
    } else {
        emu.pc = 2186572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d4c));
    }
}
#[inline(always)]
pub fn block_0x00215d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2186576u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2186580u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2186584u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2186588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2186668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215dac));
}
#[inline(always)]
pub fn block_0x00215d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 19usize, 2186592u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2186596u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2186600u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2186604u32);
    emu.apc_no_count(1usize, 2186604u32, 4294942720u32, 2186608u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2186572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d4c));
    } else {
        emu.pc = 2186616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00215d78));
    }
}
#[inline(always)]
pub fn block_0x00215d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 25usize, 2186620u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2186624u32);
    emu.sri_no_count(19usize, 10usize, 16u32, 2186628u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2186632u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186636u32);
    emu.apc_no_count(1usize, 2186636u32, 0u32, 2186640u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186644u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00215d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2186648u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186652u32);
    emu.apc_no_count(1usize, 2186652u32, 0u32, 2186656u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00215da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2186664u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2186668u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2186668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00215dac));
}
#[inline(always)]
pub fn block_0x00215dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2186668u32, 0u32, 2186672u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2186676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
