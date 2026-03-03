pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2241512u32;
pub const PC_MAX: u32 = 2243892u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 116usize] = [
        block_0x002233e8,
        block_0x002233f0,
        block_0x002233f8,
        block_0x00223400,
        block_0x00223408,
        block_0x00223410,
        block_0x00223444,
        block_0x0022344c,
        block_0x00223480,
        block_0x00223560,
        block_0x00223584,
        block_0x00223590,
        block_0x00223594,
        block_0x002235b8,
        block_0x002235e0,
        block_0x002235e4,
        block_0x002235ec,
        block_0x002235f4,
        block_0x002235fc,
        block_0x00223604,
        block_0x00223610,
        block_0x00223614,
        block_0x0022363c,
        block_0x00223664,
        block_0x00223668,
        block_0x00223670,
        block_0x00223678,
        block_0x0022367c,
        block_0x00223684,
        block_0x00223690,
        block_0x00223694,
        block_0x002236ac,
        block_0x002236d4,
        block_0x002236d8,
        block_0x002236e0,
        block_0x002236e8,
        block_0x002236ec,
        block_0x002236f4,
        block_0x00223708,
        block_0x00223714,
        block_0x00223730,
        block_0x00223738,
        block_0x00223750,
        block_0x00223758,
        block_0x00223770,
        block_0x00223774,
        block_0x0022377c,
        block_0x00223790,
        block_0x0022379c,
        block_0x002237b8,
        block_0x002237c0,
        block_0x002237d8,
        block_0x002237e0,
        block_0x002237f8,
        block_0x002237fc,
        block_0x00223804,
        block_0x00223818,
        block_0x00223824,
        block_0x00223840,
        block_0x00223848,
        block_0x00223860,
        block_0x00223868,
        block_0x00223880,
        block_0x00223884,
        block_0x0022388c,
        block_0x002238a0,
        block_0x002238ac,
        block_0x002238c8,
        block_0x002238d0,
        block_0x002238e8,
        block_0x002238f0,
        block_0x00223908,
        block_0x0022390c,
        block_0x00223914,
        block_0x00223928,
        block_0x00223934,
        block_0x00223950,
        block_0x00223958,
        block_0x00223970,
        block_0x00223978,
        block_0x00223990,
        block_0x00223994,
        block_0x002239a4,
        block_0x002239c0,
        block_0x002239d4,
        block_0x002239ec,
        block_0x00223a04,
        block_0x00223a54,
        block_0x00223a68,
        block_0x00223a78,
        block_0x00223a8c,
        block_0x00223a90,
        block_0x00223a94,
        block_0x00223a98,
        block_0x00223aa0,
        block_0x00223aa4,
        block_0x00223aa8,
        block_0x00223ab0,
        block_0x00223ae8,
        block_0x00223b50,
        block_0x00223bac,
        block_0x00223c1c,
        block_0x00223c4c,
        block_0x00223c7c,
        block_0x00223c90,
        block_0x00223ca0,
        block_0x00223cb0,
        block_0x00223cc0,
        block_0x00223cd0,
        block_0x00223cd4,
        block_0x00223ce4,
        block_0x00223cfc,
        block_0x00223d00,
        block_0x00223d1c,
        block_0x00223d2c,
        block_0x00223d34,
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
    if pc < 2241512u32 || pc > 2243892u32 {
        return None;
    }
    let word_offset = ((pc - 2241512u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002233e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 32u32, 2241516u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2241528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002233f8));
    } else {
        emu.pc = 2241520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002233f0));
    }
}
#[inline(always)]
pub fn block_0x002233f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2241524u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241528u32;
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
pub fn block_0x002233f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 127u32, 2241532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2241544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223408));
    } else {
        emu.pc = 2241536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223400));
    }
}
#[inline(always)]
pub fn block_0x00223400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2241540u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241544u32;
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
pub fn block_0x00223408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 16u32, 2241548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2241604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223444));
    } else {
        emu.pc = 2241552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223410));
    }
}
#[inline]
pub fn block_0x00223410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2241556u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2241560u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965582u32, 2241564u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2241568u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965662u32, 2241572u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2241576u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294965952u32, 2241580u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2241584u32);
    emu.adi_no_count(14usize, 0usize, 290u32, 2241588u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2241592u32);
    emu.adi_no_count(16usize, 0usize, 297u32, 2241596u32);
    emu.apc_no_count(6usize, 2241596u32, 0u32, 2241600u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2241604u32;
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
pub fn block_0x00223444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 17u32, 2241608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2241664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223480));
    } else {
        emu.pc = 2241612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022344c));
    }
}
#[inline]
pub fn block_0x0022344c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2241616u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2241620u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1600u32, 2241624u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2241628u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1688u32, 2241632u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2241636u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1896u32, 2241640u32);
    emu.adi_no_count(12usize, 0usize, 44u32, 2241644u32);
    emu.adi_no_count(14usize, 0usize, 208u32, 2241648u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2241652u32);
    emu.adi_no_count(16usize, 0usize, 486u32, 2241656u32);
    emu.apc_no_count(6usize, 2241656u32, 0u32, 2241660u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2241664u32;
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
pub fn block_0x00223480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2241668u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(172032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2241672u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294791168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2241676u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294782976u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2241680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294774784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2241684u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294770688u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2241688u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294766592u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2241692u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4294049792u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2241696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2241700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967264u32, 2241704u32);
    emu.adi_no_count(12usize, 12usize, 1760u32, 2241708u32);
    emu.adi_no_count(13usize, 13usize, 4294965440u32, 2241712u32);
    emu.adi_no_count(14usize, 14usize, 336u32, 2241716u32);
    emu.anr_no_count(7usize, 10usize, 11usize, 2241720u32);
    emu.xrr_no_count(12usize, 7usize, 12usize, 2241724u32);
    emu.adi_no_count(7usize, 15usize, 1040u32, 2241728u32);
    emu.adi_no_count(15usize, 15usize, 4294965248u32, 2241732u32);
    emu.adr_no_count(16usize, 10usize, 16usize, 2241736u32);
    emu.adi_no_count(17usize, 17usize, 4294966448u32, 2241740u32);
    emu.adi_no_count(5usize, 5usize, 4294967040u32, 2241744u32);
    emu.adi_no_count(6usize, 6usize, 496u32, 2241748u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2241752u32);
    emu.adi_no_count(11usize, 11usize, 30u32, 2241756u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2241760u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2241764u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2241768u32);
    emu.adr_no_count(17usize, 10usize, 17usize, 2241772u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2241776u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2241780u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2241784u32);
    let a = 0u32.wrapping_add(4294963200u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2241788u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1630u32, 2241792u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2241796u32);
    let a = 0u32.wrapping_add(4294254592u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2241800u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 688u32, 2241804u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2241808u32);
    let a = 0u32.wrapping_add(180224u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2241812u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294965278u32, 2241816u32);
    emu.xrr_no_count(10usize, 10usize, 5usize, 2241820u32);
    emu.sltiu_no_count(14usize, 14usize, 4294967282u32, 2241824u32);
    emu.sltiu_no_count(5usize, 7usize, 4294967281u32, 2241828u32);
    emu.anr_no_count(14usize, 14usize, 5usize, 2241832u32);
    emu.sltiu_no_count(17usize, 17usize, 4294967291u32, 2241836u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2241840u32);
    emu.sltiu_no_count(13usize, 13usize, 4294967290u32, 2241844u32);
    emu.sltru_no_count(12usize, 0usize, 12usize, 2241848u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2241852u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2241856u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2241860u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2241864u32);
    emu.sltiu_no_count(12usize, 16usize, 4294965790u32, 2241868u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2241872u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2241876u32);
    emu.anr_no_count(11usize, 15usize, 6usize, 2241880u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2241884u32);
    emu.add_memory_rw_events(56usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2241888u32;
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
pub fn block_0x00223560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967120u32, 2241892u32);
    emu.sw_no_count(1usize, 2usize, 172u32, 2241896u32)?;
    emu.sw_no_count(8usize, 2usize, 168u32, 2241900u32)?;
    emu.sw_no_count(9usize, 2usize, 164u32, 2241904u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2241908u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2241912u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2241916u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2241920u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2242044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235fc));
    } else {
        emu.pc = 2241924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223584));
    }
}
#[inline(always)]
pub fn block_0x00223584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2241928u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2241932u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2243008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239c0));
    } else {
        emu.pc = 2241936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223590));
    }
}
#[inline(always)]
pub fn block_0x00223590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235f4));
    } else {
        emu.pc = 2241940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223594));
    }
}
#[inline]
pub fn block_0x00223594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2241944u32);
    emu.sli_no_count(9usize, 9usize, 2u32, 2241948u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2241952u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966252u32, 2241956u32);
    emu.adr_no_count(11usize, 11usize, 9usize, 2241960u32);
    emu.lw_no_count(13usize, 11usize, 0u32, 2241964u32)?;
    emu.sli_no_count(14usize, 10usize, 2u32, 2241968u32);
    emu.adr_no_count(11usize, 8usize, 14usize, 2241972u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2241976u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2241976u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002235b8));
}
#[inline]
pub fn block_0x002235b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2241980u32)?;
    emu.mulhu_no_count(17usize, 16usize, 13usize, 2241984u32);
    emu.mul_no_count(16usize, 16usize, 13usize, 2241988u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2241992u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2241996u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2242000u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2242004u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2242008u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2242012u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2241976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235b8));
    } else {
        emu.pc = 2242016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235e0));
    }
}
#[inline(always)]
pub fn block_0x002235e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235f4));
    } else {
        emu.pc = 2242020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235e4));
    }
}
#[inline(always)]
pub fn block_0x002235e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2242024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2243052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239ec));
    } else {
        emu.pc = 2242028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002235ec));
    }
}
#[inline(always)]
pub fn block_0x002235ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2242032u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2242036u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2242036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002235f4));
}
#[inline(always)]
pub fn block_0x002235f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2242040u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2242044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002239a4));
}
#[inline(always)]
pub fn block_0x002235fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 9usize, 7u32, 2242048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2242172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022367c));
    } else {
        emu.pc = 2242052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223604));
    }
}
#[inline(always)]
pub fn block_0x00223604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2242056u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2242060u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2243008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239c0));
    } else {
        emu.pc = 2242064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223610));
    }
}
#[inline(always)]
pub fn block_0x00223610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223678));
    } else {
        emu.pc = 2242068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223614));
    }
}
#[inline]
pub fn block_0x00223614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2242072u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2242076u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2242080u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966252u32, 2242084u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2242088u32);
    emu.lw_no_count(14usize, 13usize, 0u32, 2242092u32)?;
    emu.sli_no_count(13usize, 10usize, 2u32, 2242096u32);
    emu.srr_no_count(14usize, 14usize, 12usize, 2242100u32);
    emu.adr_no_count(12usize, 8usize, 13usize, 2242104u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2242108u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2242108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022363c));
}
#[inline]
pub fn block_0x0022363c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2242112u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2242116u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2242120u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2242124u32);
    emu.sw_no_count(11usize, 15usize, 0u32, 2242128u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2242132u32);
    emu.sltru_no_count(11usize, 11usize, 16usize, 2242136u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2242140u32);
    emu.adr_no_count(11usize, 17usize, 11usize, 2242144u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2242108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022363c));
    } else {
        emu.pc = 2242148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223664));
    }
}
#[inline(always)]
pub fn block_0x00223664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223678));
    } else {
        emu.pc = 2242152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223668));
    }
}
#[inline(always)]
pub fn block_0x00223668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2242156u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2243052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239ec));
    } else {
        emu.pc = 2242160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223670));
    }
}
#[inline(always)]
pub fn block_0x00223670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 12usize, 0u32, 2242164u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2242168u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2242168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223678));
}
#[inline(always)]
pub fn block_0x00223678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2242172u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022367c));
}
#[inline(always)]
pub fn block_0x0022367c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 8u32, 2242176u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236ec));
    } else {
        emu.pc = 2242180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223684));
    }
}
#[inline(always)]
pub fn block_0x00223684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 160u32, 2242184u32)?;
    emu.adi_no_count(11usize, 0usize, 41u32, 2242188u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2243008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239c0));
    } else {
        emu.pc = 2242192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223690));
    }
}
#[inline(always)]
pub fn block_0x00223690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236e8));
    } else {
        emu.pc = 2242196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223694));
    }
}
#[inline(always)]
pub fn block_0x00223694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2242200u32);
    emu.sli_no_count(13usize, 10usize, 2u32, 2242204u32);
    let a = 0u32.wrapping_add(389120u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2242208u32;
    emu.update_insn_clock();
    emu.adr_no_count(11usize, 8usize, 13usize, 2242212u32);
    emu.adi_no_count(14usize, 14usize, 1505u32, 2242216u32);
    emu.adi_no_count(15usize, 8usize, 0u32, 2242220u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002236ac));
}
#[inline]
pub fn block_0x002236ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2242224u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2242228u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2242232u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2242236u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2242240u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2242244u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2242248u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2242252u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2242256u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2242220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236ac));
    } else {
        emu.pc = 2242260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236d4));
    }
}
#[inline(always)]
pub fn block_0x002236d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2242280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236e8));
    } else {
        emu.pc = 2242264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236d8));
    }
}
#[inline(always)]
pub fn block_0x002236d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2242268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2243052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239ec));
    } else {
        emu.pc = 2242272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236e0));
    }
}
#[inline(always)]
pub fn block_0x002236e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 11usize, 0u32, 2242276u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2242280u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2242280u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002236e8));
}
#[inline(always)]
pub fn block_0x002236e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 160u32, 2242284u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002236ec));
}
#[inline(always)]
pub fn block_0x002236ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 16u32, 2242288u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223774));
    } else {
        emu.pc = 2242292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002236f4));
    }
}
#[inline(always)]
pub fn block_0x002236f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2242296u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242300u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2242304u32);
    emu.apc_no_count(1usize, 2242304u32, 4294864896u32, 2242308u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242312u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2242316u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2242320u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2242352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223730));
    } else {
        emu.pc = 2242324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223714));
    }
}
#[inline(always)]
pub fn block_0x00223714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2242328u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966292u32, 2242332u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242336u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2242340u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2242344u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2242348u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2242352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223750));
}
#[inline(always)]
pub fn block_0x00223730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2242356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2243028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239d4));
    } else {
        emu.pc = 2242360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223738));
    }
}
#[inline(always)]
pub fn block_0x00223738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2242364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966292u32, 2242368u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242372u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2242376u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2242380u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2242384u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223750));
}
#[inline(always)]
pub fn block_0x00223750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2242384u32, 4294946816u32, 2242388u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2242396u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2242400u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242404u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2242408u32);
    emu.apc_no_count(1usize, 2242408u32, 4294864896u32, 2242412u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2242420u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223774));
}
#[inline(always)]
pub fn block_0x00223774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 32u32, 2242424u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002237fc));
    } else {
        emu.pc = 2242428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022377c));
    }
}
#[inline(always)]
pub fn block_0x0022377c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2242432u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242436u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2242440u32);
    emu.apc_no_count(1usize, 2242440u32, 4294864896u32, 2242444u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2242452u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2242456u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2242488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002237b8));
    } else {
        emu.pc = 2242460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022379c));
    }
}
#[inline(always)]
pub fn block_0x0022379c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2242464u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966300u32, 2242468u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242472u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2242476u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2242480u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2242484u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2242488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002237d8));
}
#[inline(always)]
pub fn block_0x002237b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2242492u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2243028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239d4));
    } else {
        emu.pc = 2242496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002237c0));
    }
}
#[inline(always)]
pub fn block_0x002237c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2242500u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966300u32, 2242504u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242508u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2242512u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2242516u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2242520u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002237d8));
}
#[inline(always)]
pub fn block_0x002237d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2242520u32, 4294946816u32, 2242524u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242528u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002237e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2242532u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2242536u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242540u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2242544u32);
    emu.apc_no_count(1usize, 2242544u32, 4294864896u32, 2242548u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002237f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2242556u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002237fc));
}
#[inline(always)]
pub fn block_0x002237fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 64u32, 2242560u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223884));
    } else {
        emu.pc = 2242564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223804));
    }
}
#[inline(always)]
pub fn block_0x00223804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2242568u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242572u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2242576u32);
    emu.apc_no_count(1usize, 2242576u32, 4294864896u32, 2242580u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242584u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2242588u32)?;
    emu.adi_no_count(10usize, 0usize, 5u32, 2242592u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2242624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223840));
    } else {
        emu.pc = 2242596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223824));
    }
}
#[inline(always)]
pub fn block_0x00223824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2242600u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966312u32, 2242604u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242608u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2242612u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2242616u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2242620u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2242624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223860));
}
#[inline(always)]
pub fn block_0x00223840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2242628u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2243028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239d4));
    } else {
        emu.pc = 2242632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223848));
    }
}
#[inline(always)]
pub fn block_0x00223848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2242636u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966312u32, 2242640u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242644u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2242648u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2242652u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2242656u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223860));
}
#[inline(always)]
pub fn block_0x00223860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2242656u32, 4294946816u32, 2242660u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242664u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00223868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2242668u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2242672u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242676u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2242680u32);
    emu.apc_no_count(1usize, 2242680u32, 4294864896u32, 2242684u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2242692u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223884));
}
#[inline(always)]
pub fn block_0x00223884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 128u32, 2242696u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022390c));
    } else {
        emu.pc = 2242700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022388c));
    }
}
#[inline(always)]
pub fn block_0x0022388c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2242704u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242708u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2242712u32);
    emu.apc_no_count(1usize, 2242712u32, 4294864896u32, 2242716u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002238a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2242724u32)?;
    emu.adi_no_count(10usize, 0usize, 10u32, 2242728u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2242760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002238c8));
    } else {
        emu.pc = 2242732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002238ac));
    }
}
#[inline(always)]
pub fn block_0x002238ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2242736u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966332u32, 2242740u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242744u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2242748u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2242752u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2242756u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2242760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002238e8));
}
#[inline(always)]
pub fn block_0x002238c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2242764u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2243028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239d4));
    } else {
        emu.pc = 2242768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002238d0));
    }
}
#[inline(always)]
pub fn block_0x002238d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2242772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966332u32, 2242776u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242780u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2242784u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2242788u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2242792u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002238e8));
}
#[inline(always)]
pub fn block_0x002238e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2242792u32, 4294946816u32, 2242796u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002238f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2242804u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2242808u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242812u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2242816u32);
    emu.apc_no_count(1usize, 2242816u32, 4294864896u32, 2242820u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2242828u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022390c));
}
#[inline(always)]
pub fn block_0x0022390c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 256u32, 2242832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2242964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223994));
    } else {
        emu.pc = 2242836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223914));
    }
}
#[inline(always)]
pub fn block_0x00223914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2242840u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242844u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2242848u32);
    emu.apc_no_count(1usize, 2242848u32, 4294864896u32, 2242852u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 8usize, 160u32, 2242860u32)?;
    emu.adi_no_count(10usize, 0usize, 19u32, 2242864u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2242896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223950));
    } else {
        emu.pc = 2242868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223934));
    }
}
#[inline(always)]
pub fn block_0x00223934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2242872u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966372u32, 2242876u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242880u32);
    emu.adi_no_count(14usize, 0usize, 19u32, 2242884u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2242888u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2242892u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2242896u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2242928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223970));
}
#[inline(always)]
pub fn block_0x00223950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 41u32, 2242900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2243028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002239d4));
    } else {
        emu.pc = 2242904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223958));
    }
}
#[inline(always)]
pub fn block_0x00223958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2242908u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966372u32, 2242912u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2242916u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2242920u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2242924u32);
    emu.adi_no_count(14usize, 15usize, 0u32, 2242928u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2242928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223970));
}
#[inline(always)]
pub fn block_0x00223970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2242928u32, 4294942720u32, 2242932u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2242940u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2242944u32);
    emu.adi_no_count(12usize, 0usize, 160u32, 2242948u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2242952u32);
    emu.apc_no_count(1usize, 2242952u32, 4294864896u32, 2242956u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 160u32, 2242964u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2242964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223994));
}
#[inline(always)]
pub fn block_0x00223994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2242968u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2242972u32);
    emu.apc_no_count(1usize, 2242972u32, 4294942720u32, 2242976u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2242980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002239a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2242984u32);
    emu.lw_no_count(1usize, 2usize, 172u32, 2242988u32)?;
    emu.lw_no_count(8usize, 2usize, 168u32, 2242992u32)?;
    emu.lw_no_count(9usize, 2usize, 164u32, 2242996u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2243000u32)?;
    emu.adi_no_count(2usize, 2usize, 176u32, 2243004u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243008u32;
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
pub fn block_0x002239c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2243012u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2243016u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2243020u32);
    emu.apc_no_count(1usize, 2243020u32, 8192u32, 2243024u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243028u32;
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
pub fn block_0x002239d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2243032u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2243036u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2243040u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2243044u32);
    emu.apc_no_count(1usize, 2243044u32, 8192u32, 2243048u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243052u32;
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
pub fn block_0x002239ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2243056u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2243060u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2243064u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2243068u32);
    emu.apc_no_count(1usize, 2243068u32, 4294942720u32, 2243072u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(48u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00223a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965904u32, 2243080u32);
    emu.sw_no_count(1usize, 2usize, 1388u32, 2243084u32)?;
    emu.sw_no_count(8usize, 2usize, 1384u32, 2243088u32)?;
    emu.sw_no_count(9usize, 2usize, 1380u32, 2243092u32)?;
    emu.sw_no_count(18usize, 2usize, 1376u32, 2243096u32)?;
    emu.sw_no_count(19usize, 2usize, 1372u32, 2243100u32)?;
    emu.sw_no_count(20usize, 2usize, 1368u32, 2243104u32)?;
    emu.sw_no_count(21usize, 2usize, 1364u32, 2243108u32)?;
    emu.sw_no_count(22usize, 2usize, 1360u32, 2243112u32)?;
    emu.sw_no_count(23usize, 2usize, 1356u32, 2243116u32)?;
    emu.sw_no_count(24usize, 2usize, 1352u32, 2243120u32)?;
    emu.sw_no_count(25usize, 2usize, 1348u32, 2243124u32)?;
    emu.sw_no_count(26usize, 2usize, 1344u32, 2243128u32)?;
    emu.sw_no_count(27usize, 2usize, 1340u32, 2243132u32)?;
    emu.adi_no_count(24usize, 13usize, 0u32, 2243136u32);
    emu.sw_no_count(12usize, 2usize, 20u32, 2243140u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2243144u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2243148u32)?;
    emu.orr_no_count(14usize, 12usize, 13usize, 2243152u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2246564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246564u32));
    } else {
        emu.pc = 2243156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a54));
    }
}
#[inline(always)]
pub fn block_0x00223a54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 10usize, 0u32, 2243160u32);
    emu.lw_no_count(18usize, 11usize, 8u32, 2243164u32)?;
    emu.lw_no_count(20usize, 11usize, 12u32, 2243168u32)?;
    emu.orr_no_count(10usize, 18usize, 20usize, 2243172u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2246592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246592u32));
    } else {
        emu.pc = 2243176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a68));
    }
}
#[inline(always)]
pub fn block_0x00223a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 16u32, 2243180u32)?;
    emu.lw_no_count(9usize, 11usize, 20u32, 2243184u32)?;
    emu.orr_no_count(10usize, 8usize, 9usize, 2243188u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2246620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246620u32));
    } else {
        emu.pc = 2243192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a78));
    }
}
#[inline(always)]
pub fn block_0x00223a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 8usize, 2243196u32);
    emu.sltru_no_count(15usize, 10usize, 12usize, 2243200u32);
    emu.adr_no_count(14usize, 13usize, 9usize, 2243204u32);
    emu.adr_no_count(16usize, 14usize, 15usize, 2243208u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2243216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a90));
    } else {
        emu.pc = 2243212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a8c));
    }
}
#[inline(always)]
pub fn block_0x00223a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2243216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2243216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223a90));
}
#[inline(always)]
pub fn block_0x00223a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2246648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246648u32));
    } else {
        emu.pc = 2243220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a94));
    }
}
#[inline(always)]
pub fn block_0x00223a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2243232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223aa0));
    } else {
        emu.pc = 2243224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223a98));
    }
}
#[inline(always)]
pub fn block_0x00223a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 13usize, 20usize, 2243228u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2243232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2243236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223aa4));
}
#[inline(always)]
pub fn block_0x00223aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 18usize, 2243236u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2243236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223aa4));
}
#[inline(always)]
pub fn block_0x00223aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2246676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246676u32));
    } else {
        emu.pc = 2243240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223aa8));
    }
}
#[inline(always)]
pub fn block_0x00223aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 16u32, 2243244u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2246704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246704u32));
    } else {
        emu.pc = 2243248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ab0));
    }
}
#[inline]
pub fn block_0x00223ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(19usize, 11usize, 24u32, 2243252u32)?;
    emu.sltru_no_count(15usize, 10usize, 12usize, 2243256u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2243260u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2243264u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2243268u32;
    emu.update_insn_clock();
    emu.adr_no_count(15usize, 14usize, 15usize, 2243272u32);
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2243276u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 16usize, 1365u32, 2243280u32);
    emu.adi_no_count(16usize, 6usize, 819u32, 2243284u32);
    emu.adi_no_count(14usize, 14usize, 4294967055u32, 2243288u32);
    emu.sbr_no_count(5usize, 15usize, 5usize, 2243292u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2243296u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 257u32, 2243300u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2243408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223b50));
    } else {
        emu.pc = 2243304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ae8));
    }
}
#[inline(never)]
pub fn block_0x00223ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2243308u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2243312u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243316u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2243320u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243324u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2243328u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243332u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2243336u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243340u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2243344u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243348u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2243352u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2243356u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2243360u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2243364u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2243368u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2243372u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2243376u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2243380u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2243384u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2243388u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2243392u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2243396u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2243400u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2243404u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2243408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2243500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223bac));
}
#[inline]
pub fn block_0x00223b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 5usize, 1u32, 2243412u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2243416u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2243420u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243424u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2243428u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243432u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2243436u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243440u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2243444u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2243448u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2243452u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2243456u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2243460u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2243464u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2243468u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2243472u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2243476u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2243480u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2243484u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2243488u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2243492u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2243496u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2243500u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2243500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223bac));
}
#[inline(never)]
pub fn block_0x00223bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 11usize, 26u32, 2243504u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2243508u32)?;
    emu.sbr_no_count(10usize, 19usize, 10usize, 2243512u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2243516u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2243520u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2243524u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2243528u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2243532u32);
    emu.adi_no_count(11usize, 14usize, 4294967295u32, 2243536u32);
    emu.anr_no_count(11usize, 11usize, 13usize, 2243540u32);
    emu.adi_no_count(22usize, 0usize, 2u32, 2243544u32);
    emu.sbr_no_count(13usize, 22usize, 14usize, 2243548u32);
    emu.sw_no_count(13usize, 2usize, 188u32, 2243552u32)?;
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2243556u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2243560u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2243564u32);
    emu.sw_no_count(12usize, 2usize, 28u32, 2243568u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2243572u32)?;
    emu.sltru_no_count(10usize, 13usize, 10usize, 2243576u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2243580u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2243584u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2243588u32);
    emu.sai_no_count(25usize, 10usize, 1040u32, 2243592u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2243596u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2243600u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2243604u32);
    emu.apc_no_count(1usize, 2243604u32, 4294860800u32, 2243608u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243612u32;
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
#[inline]
pub fn block_0x00223c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 20usize, 1u32, 2243616u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2243620u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2243624u32);
    emu.anr_no_count(11usize, 11usize, 20usize, 2243628u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2243632u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2243636u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2243640u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2243644u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2243648u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2243652u32);
    emu.apc_no_count(1usize, 2243652u32, 4294860800u32, 2243656u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00223c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 9usize, 1u32, 2243664u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2243668u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2243672u32);
    emu.anr_no_count(11usize, 11usize, 9usize, 2243676u32);
    emu.sw_no_count(10usize, 2usize, 516u32, 2243680u32)?;
    emu.sw_no_count(8usize, 2usize, 356u32, 2243684u32)?;
    emu.sw_no_count(11usize, 2usize, 360u32, 2243688u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2243692u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2243696u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2243700u32);
    emu.apc_no_count(1usize, 2243700u32, 4294860800u32, 2243704u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243708u32;
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
pub fn block_0x00223c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 524u32, 2243712u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2243716u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2243720u32);
    emu.apc_no_count(1usize, 2243720u32, 4294860800u32, 2243724u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2243732u32);
    emu.sw_no_count(10usize, 2usize, 680u32, 2243736u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2243740u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2243812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ce4));
    } else {
        emu.pc = 2243744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ca0));
    }
}
#[inline(always)]
pub fn block_0x00223ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2243748u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243752u32);
    emu.apc_no_count(1usize, 2243752u32, 4294942720u32, 2243756u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2243764u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243768u32);
    emu.apc_no_count(1usize, 2243768u32, 4294942720u32, 2243772u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2243780u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243784u32);
    emu.apc_no_count(1usize, 2243784u32, 4294942720u32, 2243788u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243792u32;
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
pub fn block_0x00223cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2243840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d00));
    } else {
        emu.pc = 2243796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223cd4));
    }
}
#[inline(always)]
pub fn block_0x00223cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2243800u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2243804u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2243808u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2243812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2243892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223d34));
}
#[inline(always)]
pub fn block_0x00223ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 19usize, 2243816u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2243820u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2243824u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2243828u32);
    emu.apc_no_count(1usize, 2243828u32, 4294942720u32, 2243832u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2243796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223cd4));
    } else {
        emu.pc = 2243840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d00));
    }
}
#[inline(always)]
pub fn block_0x00223d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 25usize, 2243844u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2243848u32);
    emu.sri_no_count(19usize, 10usize, 16u32, 2243852u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2243856u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243860u32);
    emu.apc_no_count(1usize, 2243860u32, 0u32, 2243864u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243868u32;
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
pub fn block_0x00223d1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2243872u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243876u32);
    emu.apc_no_count(1usize, 2243876u32, 0u32, 2243880u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243884u32;
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
pub fn block_0x00223d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2243888u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2243892u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2243892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223d34));
}
#[inline(always)]
pub fn block_0x00223d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2243892u32, 0u32, 2243896u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243900u32;
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
