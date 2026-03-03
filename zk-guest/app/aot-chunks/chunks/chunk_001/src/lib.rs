pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2101548u32;
pub const PC_MAX: u32 = 2103840u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 135usize] = [
        block_0x0020112c,
        block_0x00201148,
        block_0x0020114c,
        block_0x00201154,
        block_0x0020115c,
        block_0x00201164,
        block_0x00201168,
        block_0x00201170,
        block_0x00201174,
        block_0x0020117c,
        block_0x00201188,
        block_0x0020118c,
        block_0x00201190,
        block_0x00201194,
        block_0x002011b8,
        block_0x002011bc,
        block_0x002011d4,
        block_0x002011e8,
        block_0x002011ec,
        block_0x002011f8,
        block_0x00201234,
        block_0x00201238,
        block_0x00201258,
        block_0x00201274,
        block_0x00201288,
        block_0x00201294,
        block_0x002012a4,
        block_0x002012ac,
        block_0x002012bc,
        block_0x002012c0,
        block_0x002012e0,
        block_0x002012f4,
        block_0x002012fc,
        block_0x00201300,
        block_0x0020130c,
        block_0x00201320,
        block_0x00201334,
        block_0x00201338,
        block_0x00201354,
        block_0x00201360,
        block_0x00201368,
        block_0x00201370,
        block_0x00201380,
        block_0x00201390,
        block_0x0020139c,
        block_0x002013a0,
        block_0x002013ac,
        block_0x002013b4,
        block_0x002013b8,
        block_0x002013c0,
        block_0x002013dc,
        block_0x002013ec,
        block_0x002013f4,
        block_0x002013f8,
        block_0x00201404,
        block_0x00201410,
        block_0x00201424,
        block_0x00201428,
        block_0x00201440,
        block_0x00201458,
        block_0x00201474,
        block_0x00201484,
        block_0x00201494,
        block_0x002014ac,
        block_0x002014c4,
        block_0x002014d8,
        block_0x002014f0,
        block_0x00201504,
        block_0x0020151c,
        block_0x00201524,
        block_0x00201540,
        block_0x00201550,
        block_0x00201560,
        block_0x00201564,
        block_0x00201578,
        block_0x0020158c,
        block_0x002015a4,
        block_0x002015b8,
        block_0x002015c0,
        block_0x002015c4,
        block_0x002015cc,
        block_0x002015d8,
        block_0x002015dc,
        block_0x002015e0,
        block_0x002015e4,
        block_0x002015e8,
        block_0x002015ec,
        block_0x00201650,
        block_0x0020165c,
        block_0x00201660,
        block_0x00201670,
        block_0x0020167c,
        block_0x002016a0,
        block_0x00201704,
        block_0x00201710,
        block_0x00201714,
        block_0x00201724,
        block_0x00201738,
        block_0x00201750,
        block_0x0020176c,
        block_0x00201784,
        block_0x0020179c,
        block_0x002017b0,
        block_0x002017d0,
        block_0x002017e8,
        block_0x002017ec,
        block_0x002017f8,
        block_0x00201814,
        block_0x00201818,
        block_0x00201844,
        block_0x00201858,
        block_0x00201894,
        block_0x002018a8,
        block_0x002018ac,
        block_0x002018b4,
        block_0x002018c0,
        block_0x002018c4,
        block_0x002018e0,
        block_0x002018e8,
        block_0x002018f0,
        block_0x002018fc,
        block_0x00201900,
        block_0x00201914,
        block_0x00201918,
        block_0x0020192c,
        block_0x00201944,
        block_0x00201970,
        block_0x00201988,
        block_0x002019a0,
        block_0x002019bc,
        block_0x002019c8,
        block_0x002019e4,
        block_0x002019ec,
        block_0x00201a04,
        block_0x00201a20,
    ];
    const IDX: [u16; 574usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 4u16, 0u16, 5u16,
        0u16, 6u16, 7u16, 0u16, 8u16, 9u16, 0u16, 10u16, 0u16, 0u16, 11u16, 12u16, 13u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 20u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16,
        0u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 33u16,
        34u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16,
        37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 40u16, 0u16,
        41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16,
        45u16, 46u16, 0u16, 0u16, 47u16, 0u16, 48u16, 49u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 54u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16,
        0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 79u16, 80u16, 0u16, 81u16,
        0u16, 0u16, 82u16, 83u16, 84u16, 85u16, 86u16, 87u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 89u16, 90u16, 0u16,
        0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16, 0u16, 107u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16,
        0u16, 0u16, 0u16, 113u16, 114u16, 0u16, 115u16, 0u16, 0u16, 116u16, 117u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 0u16, 120u16, 0u16, 0u16,
        121u16, 122u16, 0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 0u16,
        125u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 129u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 130u16, 0u16, 0u16,
        131u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 132u16, 0u16, 133u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 134u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16,
    ];
    if pc < 2101548u32 || pc > 2103840u32 {
        return None;
    }
    let word_offset = ((pc - 2101548u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020112c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 56u32, 2101552u32)?;
    emu.lw_no_count(26usize, 2usize, 44u32, 2101556u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2101560u32)?;
    emu.lw_no_count(27usize, 2usize, 48u32, 2101564u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2101568u32)?;
    emu.adi_no_count(12usize, 0usize, 270u32, 2101572u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b8));
    } else {
        emu.pc = 2101576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201148));
    }
}
#[inline(always)]
pub fn block_0x00201148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e4));
}
#[inline(always)]
pub fn block_0x0020114c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2101584u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e4));
    } else {
        emu.pc = 2101588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201154));
    }
}
#[inline(always)]
pub fn block_0x00201154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 8u32, 2101592u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201168));
}
#[inline(always)]
pub fn block_0x0020115c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2101600u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e4));
    } else {
        emu.pc = 2101604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201164));
    }
}
#[inline(always)]
pub fn block_0x00201164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2101608u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201168));
}
#[inline(always)]
pub fn block_0x00201168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 60u32, 2101612u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b8));
    } else {
        emu.pc = 2101616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201170));
    }
}
#[inline(always)]
pub fn block_0x00201170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e4));
}
#[inline(always)]
pub fn block_0x00201174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2101624u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b4));
    } else {
        emu.pc = 2101628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020117c));
    }
}
#[inline(always)]
pub fn block_0x0020117c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 180u32, 2101632u32);
    emu.lw_no_count(21usize, 2usize, 60u32, 2101636u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b8));
    } else {
        emu.pc = 2101640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201188));
    }
}
#[inline(always)]
pub fn block_0x00201188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013b8));
    } else {
        emu.pc = 2101644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020118c));
    }
}
#[inline(always)]
pub fn block_0x0020118c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e4));
}
#[inline(always)]
pub fn block_0x00201190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 1u32, 2101652u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201194));
}
#[inline]
pub fn block_0x00201194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 2usize, 20u32, 2101656u32)?;
    emu.lw_no_count(7usize, 2usize, 16u32, 2101660u32)?;
    emu.adi_no_count(15usize, 0usize, 0u32, 2101664u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2101668u32);
    emu.sw_no_count(24usize, 2usize, 188u32, 2101672u32)?;
    emu.sw_no_count(17usize, 2usize, 192u32, 2101676u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2101680u32)?;
    emu.adi_no_count(11usize, 17usize, 0u32, 2101684u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2101688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002011bc));
}
#[inline(always)]
pub fn block_0x002011b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2101876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201274));
    } else {
        emu.pc = 2101692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011bc));
    }
}
#[inline(always)]
pub fn block_0x002011bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2101696u32);
    emu.mul_no_count(10usize, 21usize, 6usize, 2101700u32);
    emu.adi_no_count(21usize, 21usize, 1u32, 2101704u32);
    emu.divu_no_count(10usize, 10usize, 5usize, 2101708u32);
    emu.mul_no_count(27usize, 10usize, 7usize, 2101712u32);
    emu.adi_no_count(18usize, 9usize, 0u32, 2101716u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2101716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002011d4));
}
#[inline(always)]
pub fn block_0x002011d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(10usize, 24usize, 9usize, 2101720u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2101724u32);
    emu.adr_no_count(25usize, 10usize, 27usize, 2101728u32);
    emu.adi_no_count(13usize, 25usize, 4u32, 2101732u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2102420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201494));
    } else {
        emu.pc = 2101736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011e8));
    }
}
#[inline(always)]
pub fn block_0x002011e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2102444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002014ac));
    } else {
        emu.pc = 2101740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011ec));
    }
}
#[inline(always)]
pub fn block_0x002011ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2101744u32)?;
    emu.sbr_no_count(10usize, 10usize, 15usize, 2101748u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2101816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201238));
    } else {
        emu.pc = 2101752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011f8));
    }
}
#[inline]
pub fn block_0x002011f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(25usize, 26usize, 25usize, 2101756u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2101760u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2101764u32);
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2101768u32);
    emu.lbu_no_count(12usize, 25usize, 0u32, 2101772u32);
    emu.lbu_no_count(13usize, 25usize, 1u32, 2101776u32);
    emu.lbu_no_count(14usize, 25usize, 2u32, 2101780u32);
    emu.lbu_no_count(16usize, 25usize, 3u32, 2101784u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2101788u32);
    emu.sb_no_count(13usize, 10usize, 1u32, 2101792u32);
    emu.sb_no_count(14usize, 10usize, 2u32, 2101796u32);
    emu.sb_no_count(16usize, 10usize, 3u32, 2101800u32);
    emu.sw_no_count(15usize, 2usize, 196u32, 2101804u32)?;
    emu.adr_no_count(24usize, 24usize, 20usize, 2101808u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2101716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002011d4));
    } else {
        emu.pc = 2101812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201234));
    }
}
#[inline(always)]
pub fn block_0x00201234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2101816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002011b8));
}
#[inline(always)]
pub fn block_0x00201238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 188u32, 2101820u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2101824u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2101828u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2101832u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2101836u32);
    emu.sw_no_count(17usize, 2usize, 8u32, 2101840u32)?;
    emu.apc_no_count(1usize, 2101840u32, 12288u32, 2101844u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101848u32;
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
#[inline(always)]
pub fn block_0x00201258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 16u32, 2101852u32)?;
    emu.lw_no_count(6usize, 2usize, 20u32, 2101856u32)?;
    emu.lw_no_count(5usize, 2usize, 24u32, 2101860u32)?;
    emu.lw_no_count(17usize, 2usize, 8u32, 2101864u32)?;
    emu.lw_no_count(11usize, 2usize, 192u32, 2101868u32)?;
    emu.lw_no_count(15usize, 2usize, 196u32, 2101872u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2101876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002011f8));
}
#[inline(always)]
pub fn block_0x00201274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 2usize, 8u32, 2101880u32)?;
    emu.adi_no_count(10usize, 2usize, 188u32, 2101884u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2101888u32);
    emu.apc_no_count(1usize, 2101888u32, 0u32, 2101892u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101896u32;
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
#[inline(always)]
pub fn block_0x00201288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 196u32, 2101900u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2101904u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e0));
    } else {
        emu.pc = 2101908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201294));
    }
}
#[inline(always)]
pub fn block_0x00201294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2101912u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2101916u32)?;
    emu.apc_no_count(1usize, 2101916u32, 151552u32, 2101920u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002012a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 56u32, 2101928u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e4));
    } else {
        emu.pc = 2101932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012ac));
    }
}
#[inline(always)]
pub fn block_0x002012ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 44u32, 2101936u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2101940u32)?;
    emu.lw_no_count(27usize, 2usize, 48u32, 2101944u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013ac));
}
#[inline(always)]
pub fn block_0x002012bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2101952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002012c0));
}
#[inline(always)]
pub fn block_0x002012c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2101956u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2101960u32)?;
    emu.sw_no_count(25usize, 2usize, 188u32, 2101964u32)?;
    emu.sw_no_count(20usize, 2usize, 192u32, 2101968u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2101972u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2101976u32)?;
    emu.adi_no_count(25usize, 0usize, 0u32, 2101980u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2102112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201360));
    } else {
        emu.pc = 2101984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012e0));
    }
}
#[inline(always)]
pub fn block_0x002012e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(10usize, 10usize, 11usize, 2101988u32);
    emu.lw_no_count(27usize, 2usize, 16u32, 2101992u32)?;
    emu.sli_no_count(27usize, 27usize, 2u32, 2101996u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2102000u32);
    emu.adr_no_count(27usize, 10usize, 27usize, 2102004u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2102004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002012f4));
}
#[inline(always)]
pub fn block_0x002012f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 24usize, 27usize, 2102008u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2103576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201918));
    } else {
        emu.pc = 2102012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012fc));
    }
}
#[inline(always)]
pub fn block_0x002012fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2103596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020192c));
    } else {
        emu.pc = 2102016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201300));
    }
}
#[inline(always)]
pub fn block_0x00201300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2102020u32)?;
    emu.sbr_no_count(10usize, 10usize, 25usize, 2102024u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2102072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201338));
    } else {
        emu.pc = 2102028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020130c));
    }
}
#[inline(always)]
pub fn block_0x0020130c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 26usize, 27usize, 2102032u32);
    emu.adr_no_count(10usize, 20usize, 25usize, 2102036u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2102040u32);
    emu.apc_no_count(1usize, 2102040u32, 36864u32, 2102044u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102048u32;
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
pub fn block_0x00201320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(25usize, 25usize, 24usize, 2102052u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2102056u32);
    emu.sw_no_count(25usize, 2usize, 196u32, 2102060u32)?;
    emu.adr_no_count(27usize, 27usize, 18usize, 2102064u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2102004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002012f4));
    } else {
        emu.pc = 2102068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201334));
    }
}
#[inline(always)]
pub fn block_0x00201334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102072u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201368));
}
#[inline(always)]
pub fn block_0x00201338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 188u32, 2102076u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2102080u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2102084u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2102088u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2102092u32);
    emu.apc_no_count(1usize, 2102092u32, 12288u32, 2102096u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 192u32, 2102104u32)?;
    emu.lw_no_count(25usize, 2usize, 196u32, 2102108u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2102112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020130c));
}
#[inline(always)]
pub fn block_0x00201360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2102116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2102120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201370));
}
#[inline(always)]
pub fn block_0x00201368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 192u32, 2102124u32)?;
    emu.lw_no_count(27usize, 2usize, 48u32, 2102128u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201370));
}
#[inline(always)]
pub fn block_0x00201370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 188u32, 2102132u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2102136u32);
    emu.apc_no_count(1usize, 2102136u32, 0u32, 2102140u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 196u32, 2102148u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2102152u32)?;
    emu.lw_no_count(11usize, 2usize, 60u32, 2102156u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100440u32));
    } else {
        emu.pc = 2102160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201390));
    }
}
#[inline(always)]
pub fn block_0x00201390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2102164u32)?;
    emu.apc_no_count(1usize, 2102164u32, 151552u32, 2102168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102172u32;
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
pub fn block_0x0020139c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100440u32));
    } else {
        emu.pc = 2102176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013a0));
    }
}
#[inline(always)]
pub fn block_0x002013a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 56u32, 2102180u32)?;
    emu.lw_no_count(26usize, 2usize, 44u32, 2102184u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2102188u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2102188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013ac));
}
#[inline(always)]
pub fn block_0x002013ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 36u32, 2102192u32)?;
    emu.adi_no_count(12usize, 0usize, 270u32, 2102196u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2102196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013b4));
}
#[inline(always)]
pub fn block_0x002013b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 60u32, 2102200u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002013b8));
}
#[inline(always)]
pub fn block_0x002013b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 8usize, 1u32, 2102204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2100524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100524u32));
    } else {
        emu.pc = 2102208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013c0));
    }
}
#[inline(always)]
pub fn block_0x002013c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 8usize, 3u32, 2102212u32);
    emu.sli_no_count(11usize, 8usize, 6u32, 2102216u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2102220u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2102224u32);
    emu.lw_no_count(11usize, 10usize, 40u32, 2102228u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2102232u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e4));
    } else {
        emu.pc = 2102236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013dc));
    }
}
#[inline(always)]
pub fn block_0x002013dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 36u32, 2102240u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2102244u32);
    emu.apc_no_count(1usize, 2102244u32, 151552u32, 2102248u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002013ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 270u32, 2102256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100524u32));
    } else {
        emu.pc = 2102260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002013f4));
    }
}
#[inline(always)]
pub fn block_0x002013f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e4));
}
#[inline(always)]
pub fn block_0x002013f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966522u32, 2102272u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2102276u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2102276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201404));
}
#[inline(always)]
pub fn block_0x00201404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2102280u32);
    emu.apc_no_count(1usize, 2102280u32, 151552u32, 2102284u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(24usize, 10usize, 1u32, 2102292u32);
    emu.lw_no_count(11usize, 2usize, 140u32, 2102296u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2102300u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 0usize, 1u32, 2102304u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2100004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100004u32));
    } else {
        emu.pc = 2102308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201424));
    }
}
#[inline(always)]
pub fn block_0x00201424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2100336u32));
}
#[inline(always)]
pub fn block_0x00201428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102316u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102320u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966548u32, 2102324u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2102328u32)?;
    emu.apc_no_count(1usize, 2102328u32, 106496u32, 2102332u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2102340u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102344u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966580u32, 2102348u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2102352u32)?;
    emu.apc_no_count(1usize, 2102352u32, 106496u32, 2102356u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102360u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 26usize, 3u32, 2102364u32);
    emu.sli_no_count(11usize, 26usize, 6u32, 2102368u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2102372u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2102376u32);
    emu.lw_no_count(11usize, 10usize, 4294967292u32, 2102380u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2102384u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015e4));
    } else {
        emu.pc = 2102388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201474));
    }
}
#[inline(always)]
pub fn block_0x00201474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4294967288u32, 2102392u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2102396u32)?;
    emu.apc_no_count(1usize, 2102396u32, 151552u32, 2102400u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(27usize, 10usize, 1u32, 2102408u32);
    emu.sbr_no_count(10usize, 0usize, 27usize, 2102412u32);
    emu.anr_no_count(26usize, 10usize, 26usize, 2102416u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2102420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015ec));
}
#[inline(always)]
pub fn block_0x00201494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102424u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966596u32, 2102428u32);
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2102432u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2102436u32);
    emu.apc_no_count(1usize, 2102436u32, 147456u32, 2102440u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102444u32;
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
#[inline(always)]
pub fn block_0x002014ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102448u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966596u32, 2102452u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2102456u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2102460u32);
    emu.apc_no_count(1usize, 2102460u32, 147456u32, 2102464u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102472u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966628u32, 2102476u32);
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2102480u32);
    emu.apc_no_count(1usize, 2102480u32, 147456u32, 2102484u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102488u32;
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
pub fn block_0x002014d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966628u32, 2102496u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2102500u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2102504u32);
    emu.apc_no_count(1usize, 2102504u32, 147456u32, 2102508u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002014f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 13usize, 14usize, 2102516u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102520u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966644u32, 2102524u32);
    emu.apc_no_count(1usize, 2102524u32, 147456u32, 2102528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4u32, 2102536u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102540u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966644u32, 2102544u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2102548u32);
    emu.apc_no_count(1usize, 2102548u32, 147456u32, 2102552u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102556u32;
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
pub fn block_0x0020151c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 26usize, 0u32, 2102560u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2102564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015ec));
}
#[inline(always)]
pub fn block_0x00201524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 192u32, 2102568u32)?;
    emu.lw_no_count(23usize, 2usize, 196u32, 2102572u32)?;
    emu.adi_no_count(11usize, 2usize, 200u32, 2102576u32);
    emu.adi_no_count(10usize, 2usize, 388u32, 2102580u32);
    emu.adi_no_count(12usize, 0usize, 60u32, 2102584u32);
    emu.apc_no_count(1usize, 2102584u32, 36864u32, 2102588u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 380u32, 2102596u32)?;
    emu.sw_no_count(23usize, 2usize, 384u32, 2102600u32)?;
    emu.apc_no_count(1usize, 2102600u32, 4294963200u32, 2102604u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102608u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 448u32, 2102612u32)?;
    emu.sw_no_count(11usize, 2usize, 452u32, 2102616u32)?;
    emu.sw_no_count(0usize, 2usize, 456u32, 2102620u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2103712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019a0));
    } else {
        emu.pc = 2102624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201560));
    }
}
#[inline(always)]
pub fn block_0x00201560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2102628u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201564));
}
#[inline(always)]
pub fn block_0x00201564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 11usize, 22usize, 2102632u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2102636u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2102640u32);
    emu.apc_no_count(1usize, 2102640u32, 36864u32, 2102644u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 448u32, 2102652u32)?;
    emu.adr_no_count(20usize, 22usize, 20usize, 2102656u32);
    emu.sbr_no_count(10usize, 10usize, 20usize, 2102660u32);
    emu.sw_no_count(20usize, 2usize, 456u32, 2102664u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2103752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002019c8));
    } else {
        emu.pc = 2102668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020158c));
    }
}
#[inline(always)]
pub fn block_0x0020158c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 452u32, 2102672u32)?;
    emu.adr_no_count(10usize, 21usize, 20usize, 2102676u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2102680u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2102684u32);
    emu.apc_no_count(1usize, 2102684u32, 36864u32, 2102688u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002015a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 20usize, 18usize, 2102696u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2102700u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2102704u32);
    emu.apc_no_count(1usize, 2102704u32, 28672u32, 2102708u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002015b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 188u32, 2102716u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2103444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201894));
    } else {
        emu.pc = 2102720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015c0));
    }
}
#[inline(always)]
pub fn block_0x002015c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100332u32));
    } else {
        emu.pc = 2102724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015c4));
    }
}
#[inline(always)]
pub fn block_0x002015c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2102728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100332u32));
    } else {
        emu.pc = 2102732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015cc));
    }
}
#[inline(always)]
pub fn block_0x002015cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 196u32, 2102736u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2102740u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100332u32));
    } else {
        emu.pc = 2102744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002015d8));
    }
}
#[inline(always)]
pub fn block_0x002015d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2102748u32;
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
pub fn block_0x002015dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2102752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2100332u32));
}
#[inline(always)]
pub fn block_0x002015e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 56u32, 2102756u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e4));
}
#[inline(always)]
pub fn block_0x002015e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2102760u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e8));
}
#[inline(always)]
pub fn block_0x002015e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(27usize, 0usize, 0u32, 2102764u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015ec));
}
#[inline(never)]
pub fn block_0x002015ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 72u32, 2102768u32)?;
    emu.lw_no_count(11usize, 2usize, 76u32, 2102772u32)?;
    emu.sw_no_count(0usize, 2usize, 524u32, 2102776u32)?;
    emu.sw_no_count(0usize, 2usize, 528u32, 2102780u32)?;
    emu.sw_no_count(0usize, 2usize, 532u32, 2102784u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2102788u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2102792u32)?;
    emu.sw_no_count(0usize, 2usize, 544u32, 2102796u32)?;
    emu.sw_no_count(0usize, 2usize, 548u32, 2102800u32)?;
    emu.sw_no_count(0usize, 2usize, 552u32, 2102804u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2102808u32);
    emu.sri_no_count(13usize, 11usize, 1u32, 2102812u32);
    emu.sbr_no_count(13usize, 11usize, 13usize, 2102816u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2102820u32)?;
    emu.sw_no_count(11usize, 2usize, 192u32, 2102824u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2102828u32)?;
    emu.sw_no_count(13usize, 2usize, 200u32, 2102832u32)?;
    emu.sw_no_count(12usize, 2usize, 204u32, 2102836u32)?;
    emu.sb_no_count(12usize, 2usize, 208u32, 2102840u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2102844u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966796u32, 2102848u32);
    emu.adi_no_count(10usize, 2usize, 380u32, 2102852u32);
    emu.adi_no_count(11usize, 2usize, 188u32, 2102856u32);
    emu.apc_no_count(1usize, 2102856u32, 32768u32, 2102860u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 388u32, 2102868u32)?;
    emu.adi_no_count(10usize, 0usize, 32u32, 2102872u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2102880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201660));
    } else {
        emu.pc = 2102876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020165c));
    }
}
#[inline(always)]
pub fn block_0x0020165c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2102880u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2102880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201660));
}
#[inline(always)]
pub fn block_0x00201660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 384u32, 2102884u32)?;
    emu.adi_no_count(10usize, 2usize, 524u32, 2102888u32);
    emu.apc_no_count(1usize, 2102888u32, 36864u32, 2102892u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2102896u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 176u32, 2102900u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2102904u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002016a0));
    } else {
        emu.pc = 2102908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020167c));
    }
}
#[inline]
pub fn block_0x0020167c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 572u32, 2102912u32)?;
    emu.sw_no_count(0usize, 2usize, 576u32, 2102916u32)?;
    emu.sw_no_count(0usize, 2usize, 580u32, 2102920u32)?;
    emu.sw_no_count(0usize, 2usize, 584u32, 2102924u32)?;
    emu.sw_no_count(0usize, 2usize, 556u32, 2102928u32)?;
    emu.sw_no_count(0usize, 2usize, 560u32, 2102932u32)?;
    emu.sw_no_count(0usize, 2usize, 564u32, 2102936u32)?;
    emu.sw_no_count(0usize, 2usize, 568u32, 2102940u32)?;
    emu.add_memory_rw_events(9usize);
    let return_addr = 2102944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2103076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201724));
}
#[inline(never)]
pub fn block_0x002016a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 180u32, 2102948u32)?;
    emu.lw_no_count(11usize, 2usize, 184u32, 2102952u32)?;
    emu.sw_no_count(0usize, 2usize, 556u32, 2102956u32)?;
    emu.sw_no_count(0usize, 2usize, 560u32, 2102960u32)?;
    emu.sw_no_count(0usize, 2usize, 564u32, 2102964u32)?;
    emu.sw_no_count(0usize, 2usize, 568u32, 2102968u32)?;
    emu.sw_no_count(0usize, 2usize, 572u32, 2102972u32)?;
    emu.sw_no_count(0usize, 2usize, 576u32, 2102976u32)?;
    emu.sw_no_count(0usize, 2usize, 580u32, 2102980u32)?;
    emu.sw_no_count(0usize, 2usize, 584u32, 2102984u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2102988u32);
    emu.sri_no_count(13usize, 11usize, 1u32, 2102992u32);
    emu.sbr_no_count(13usize, 11usize, 13usize, 2102996u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2103000u32)?;
    emu.sw_no_count(11usize, 2usize, 192u32, 2103004u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2103008u32)?;
    emu.sw_no_count(13usize, 2usize, 200u32, 2103012u32)?;
    emu.sw_no_count(12usize, 2usize, 204u32, 2103016u32)?;
    emu.sb_no_count(12usize, 2usize, 208u32, 2103020u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103024u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966796u32, 2103028u32);
    emu.adi_no_count(10usize, 2usize, 380u32, 2103032u32);
    emu.adi_no_count(11usize, 2usize, 188u32, 2103036u32);
    emu.apc_no_count(1usize, 2103036u32, 32768u32, 2103040u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 388u32, 2103048u32)?;
    emu.adi_no_count(10usize, 0usize, 32u32, 2103052u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2103060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201714));
    } else {
        emu.pc = 2103056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201710));
    }
}
#[inline(always)]
pub fn block_0x00201710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2103060u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2103060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201714));
}
#[inline(always)]
pub fn block_0x00201714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 384u32, 2103064u32)?;
    emu.adi_no_count(10usize, 2usize, 556u32, 2103068u32);
    emu.apc_no_count(1usize, 2103068u32, 36864u32, 2103072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 316u32, 2103080u32);
    emu.adi_no_count(11usize, 2usize, 556u32, 2103084u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2103088u32);
    emu.apc_no_count(1usize, 2103088u32, 36864u32, 2103092u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103096u32;
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
pub fn block_0x00201738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(8usize, 26usize, 24u32, 2103100u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2103104u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2103108u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103112u32);
    emu.apc_no_count(1usize, 2103112u32, 36864u32, 2103116u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103120u32;
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
pub fn block_0x00201750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 64u32, 2103124u32)?;
    emu.sb_no_count(10usize, 2usize, 219u32, 2103128u32);
    emu.adi_no_count(10usize, 2usize, 220u32, 2103132u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2103136u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103140u32);
    emu.apc_no_count(1usize, 2103140u32, 36864u32, 2103144u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020176c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(18usize, 2usize, 251u32, 2103152u32);
    emu.adi_no_count(10usize, 2usize, 252u32, 2103156u32);
    emu.adi_no_count(12usize, 0usize, 31u32, 2103160u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103164u32);
    emu.apc_no_count(1usize, 2103164u32, 36864u32, 2103168u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103172u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(27usize, 2usize, 283u32, 2103176u32);
    emu.adi_no_count(10usize, 2usize, 284u32, 2103180u32);
    emu.adi_no_count(11usize, 2usize, 524u32, 2103184u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2103188u32);
    emu.apc_no_count(1usize, 2103188u32, 36864u32, 2103192u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020179c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 348u32, 2103200u32);
    emu.adi_no_count(12usize, 0usize, 28u32, 2103204u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103208u32);
    emu.apc_no_count(1usize, 2103208u32, 36864u32, 2103212u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002017b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 26usize, 8u32, 2103220u32);
    emu.sri_no_count(11usize, 26usize, 16u32, 2103224u32);
    emu.sb_no_count(8usize, 2usize, 376u32, 2103228u32);
    emu.sb_no_count(11usize, 2usize, 377u32, 2103232u32);
    emu.sb_no_count(10usize, 2usize, 378u32, 2103236u32);
    emu.sb_no_count(26usize, 2usize, 379u32, 2103240u32);
    emu.apc_no_count(1usize, 2103240u32, 32768u32, 2103244u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103248u32;
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
pub fn block_0x002017d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103252u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2103256u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2103260u32);
    emu.adi_no_count(12usize, 0usize, 192u32, 2103264u32);
    emu.apc_no_count(1usize, 2103264u32, 36864u32, 2103268u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002017e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201970));
    } else {
        emu.pc = 2103276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002017ec));
    }
}
#[inline(always)]
pub fn block_0x002017ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2103280u32);
    emu.apc_no_count(1usize, 2103280u32, 32768u32, 2103284u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103288u32;
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
#[inline(always)]
pub fn block_0x002017f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103292u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2103296u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2103300u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2103304u32);
    emu.adi_no_count(9usize, 0usize, 4u32, 2103308u32);
    emu.apc_no_count(1usize, 2103308u32, 36864u32, 2103312u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201988));
    } else {
        emu.pc = 2103320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201818));
    }
}
#[inline]
pub fn block_0x00201818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 6u32, 2103324u32);
    emu.sw_no_count(11usize, 2usize, 380u32, 2103328u32)?;
    emu.sw_no_count(8usize, 2usize, 384u32, 2103332u32)?;
    emu.sw_no_count(0usize, 2usize, 388u32, 2103336u32)?;
    emu.sw_no_count(9usize, 2usize, 392u32, 2103340u32)?;
    emu.sw_no_count(10usize, 2usize, 396u32, 2103344u32)?;
    emu.sw_no_count(0usize, 2usize, 400u32, 2103348u32)?;
    emu.adi_no_count(10usize, 2usize, 188u32, 2103352u32);
    emu.adi_no_count(11usize, 2usize, 380u32, 2103356u32);
    emu.apc_no_count(1usize, 2103356u32, 24576u32, 2103360u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 388u32, 2103368u32)?;
    emu.lw_no_count(10usize, 2usize, 384u32, 2103372u32)?;
    emu.sli_no_count(11usize, 11usize, 5u32, 2103376u32);
    emu.apc_no_count(1usize, 2103376u32, 32768u32, 2103380u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00201858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 636u32, 2103388u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2103392u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2103396u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2103400u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2103404u32)?;
    emu.lw_no_count(20usize, 2usize, 616u32, 2103408u32)?;
    emu.lw_no_count(21usize, 2usize, 612u32, 2103412u32)?;
    emu.lw_no_count(22usize, 2usize, 608u32, 2103416u32)?;
    emu.lw_no_count(23usize, 2usize, 604u32, 2103420u32)?;
    emu.lw_no_count(24usize, 2usize, 600u32, 2103424u32)?;
    emu.lw_no_count(25usize, 2usize, 596u32, 2103428u32)?;
    emu.lw_no_count(26usize, 2usize, 592u32, 2103432u32)?;
    emu.lw_no_count(27usize, 2usize, 588u32, 2103436u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2103440u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103444u32;
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
pub fn block_0x00201894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 192u32, 2103448u32);
    emu.adi_no_count(10usize, 2usize, 460u32, 2103452u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2103456u32);
    emu.apc_no_count(1usize, 2103456u32, 36864u32, 2103460u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002018a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c4));
    } else {
        emu.pc = 2103468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018ac));
    }
}
#[inline(always)]
pub fn block_0x002018ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2103472u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2103492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c4));
    } else {
        emu.pc = 2103476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018b4));
    }
}
#[inline(always)]
pub fn block_0x002018b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 196u32, 2103480u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2103484u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2103492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c4));
    } else {
        emu.pc = 2103488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018c0));
    }
}
#[inline(always)]
pub fn block_0x002018c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2103492u32;
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
pub fn block_0x002018c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 188u32, 2103496u32);
    emu.adi_no_count(11usize, 2usize, 380u32, 2103500u32);
    emu.adi_no_count(14usize, 2usize, 460u32, 2103504u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2103508u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2103512u32);
    emu.apc_no_count(1usize, 2103512u32, 12288u32, 2103516u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002018e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 188u32, 2103524u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2103552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201900));
    } else {
        emu.pc = 2103528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018e8));
    }
}
#[inline(always)]
pub fn block_0x002018e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2103532u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2103552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201900));
    } else {
        emu.pc = 2103536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018f0));
    }
}
#[inline(always)]
pub fn block_0x002018f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 196u32, 2103540u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2103544u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2103552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201900));
    } else {
        emu.pc = 2103548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002018fc));
    }
}
#[inline(always)]
pub fn block_0x002018fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2103552u32;
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
pub fn block_0x00201900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(18usize, 8usize, 1u32, 2103556u32);
    emu.anr_no_count(10usize, 24usize, 18usize, 2103560u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2103564u32)?;
    emu.lw_no_count(26usize, 2usize, 88u32, 2103568u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2100352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2100352u32));
    } else {
        emu.pc = 2103572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201914));
    }
}
#[inline(always)]
pub fn block_0x00201914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2103576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002015e8));
}
#[inline(always)]
pub fn block_0x00201918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103580u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966564u32, 2103584u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2103588u32);
    emu.apc_no_count(1usize, 2103588u32, 147456u32, 2103592u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020192c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103600u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966564u32, 2103604u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2103608u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2103612u32);
    emu.apc_no_count(1usize, 2103612u32, 147456u32, 2103616u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103620u32;
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
#[inline]
pub fn block_0x00201944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 188u32, 2103624u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2103628u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965252u32, 2103632u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2103636u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2036u32, 2103640u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2103644u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965276u32, 2103648u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2103652u32);
    emu.adi_no_count(12usize, 2usize, 188u32, 2103656u32);
    emu.apc_no_count(1usize, 2103656u32, 126976u32, 2103660u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103664u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103668u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966460u32, 2103672u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103676u32);
    emu.adi_no_count(11usize, 0usize, 192u32, 2103680u32);
    emu.apc_no_count(1usize, 2103680u32, 106496u32, 2103684u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103688u32;
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
pub fn block_0x00201988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103692u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966476u32, 2103696u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2103700u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2103704u32);
    emu.apc_no_count(1usize, 2103704u32, 106496u32, 2103708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002019a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 448u32, 2103716u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2103720u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2103724u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2103728u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2103732u32);
    emu.apc_no_count(1usize, 2103732u32, 12288u32, 2103736u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002019bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 452u32, 2103744u32)?;
    emu.lw_no_count(22usize, 2usize, 456u32, 2103748u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2103752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201564));
}
#[inline(always)]
pub fn block_0x002019c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 448u32, 2103756u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2103760u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2103764u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2103768u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2103772u32);
    emu.apc_no_count(1usize, 2103772u32, 12288u32, 2103776u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103780u32;
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
pub fn block_0x002019e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 456u32, 2103784u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2103788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020158c));
}
#[inline(always)]
pub fn block_0x002019ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103792u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966612u32, 2103796u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2103800u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2103804u32);
    emu.apc_no_count(1usize, 2103804u32, 106496u32, 2103808u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 12u32, 2103816u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103820u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103824u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966548u32, 2103828u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2103832u32)?;
    emu.apc_no_count(1usize, 2103832u32, 106496u32, 2103836u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103840u32;
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
pub fn block_0x00201a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 2usize, 8u32, 2103844u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2103848u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2103852u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966580u32, 2103856u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2103860u32)?;
    emu.apc_no_count(1usize, 2103860u32, 106496u32, 2103864u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2103868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
