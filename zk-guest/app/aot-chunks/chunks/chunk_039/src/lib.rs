pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2245460u32;
pub const PC_MAX: u32 = 2247432u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 111usize] = [
        block_0x00224354,
        block_0x00224364,
        block_0x00224368,
        block_0x00224374,
        block_0x00224378,
        block_0x0022438c,
        block_0x002243bc,
        block_0x002243c0,
        block_0x002243c4,
        block_0x002243d8,
        block_0x002243e0,
        block_0x002243e4,
        block_0x002243e8,
        block_0x002243fc,
        block_0x00224400,
        block_0x00224418,
        block_0x0022442c,
        block_0x00224430,
        block_0x00224440,
        block_0x00224444,
        block_0x00224448,
        block_0x0022445c,
        block_0x00224484,
        block_0x0022448c,
        block_0x00224490,
        block_0x00224498,
        block_0x002244a0,
        block_0x002244a4,
        block_0x002244b0,
        block_0x002244c4,
        block_0x002244ec,
        block_0x002244f0,
        block_0x002244f4,
        block_0x002244fc,
        block_0x00224504,
        block_0x00224518,
        block_0x00224540,
        block_0x00224544,
        block_0x00224548,
        block_0x00224550,
        block_0x00224560,
        block_0x00224568,
        block_0x0022456c,
        block_0x00224574,
        block_0x00224584,
        block_0x00224590,
        block_0x00224594,
        block_0x0022459c,
        block_0x002245b8,
        block_0x002245bc,
        block_0x002245d4,
        block_0x002245d8,
        block_0x002245ec,
        block_0x002245f4,
        block_0x00224604,
        block_0x00224620,
        block_0x0022462c,
        block_0x00224644,
        block_0x00224648,
        block_0x0022464c,
        block_0x0022465c,
        block_0x00224660,
        block_0x00224664,
        block_0x00224674,
        block_0x0022468c,
        block_0x00224690,
        block_0x0022469c,
        block_0x002246a0,
        block_0x002246ec,
        block_0x002246f0,
        block_0x002246fc,
        block_0x00224714,
        block_0x00224728,
        block_0x00224734,
        block_0x0022474c,
        block_0x00224754,
        block_0x00224758,
        block_0x00224774,
        block_0x0022478c,
        block_0x002247a4,
        block_0x002247c0,
        block_0x002247dc,
        block_0x002247f8,
        block_0x00224814,
        block_0x00224830,
        block_0x0022484c,
        block_0x00224860,
        block_0x00224878,
        block_0x00224890,
        block_0x002248e0,
        block_0x002248f8,
        block_0x00224908,
        block_0x0022491c,
        block_0x00224920,
        block_0x00224924,
        block_0x00224928,
        block_0x00224930,
        block_0x00224934,
        block_0x00224938,
        block_0x00224968,
        block_0x002249d0,
        block_0x00224a2c,
        block_0x00224a94,
        block_0x00224aa8,
        block_0x00224ab8,
        block_0x00224ac8,
        block_0x00224acc,
        block_0x00224adc,
        block_0x00224af4,
        block_0x00224af8,
        block_0x00224b08,
    ];
    const IDX: [u16; 494usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        7u16, 8u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 13u16, 0u16,
        0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16,
        0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 27u16, 28u16, 0u16, 0u16, 29u16, 0u16,
        0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        31u16, 32u16, 33u16, 0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 39u16, 0u16,
        40u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 43u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 45u16, 0u16, 0u16, 46u16, 47u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16,
        60u16, 0u16, 0u16, 0u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16,
        94u16, 95u16, 96u16, 0u16, 97u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16,
    ];
    if pc < 2245460u32 || pc > 2247432u32 {
        return None;
    }
    let word_offset = ((pc - 2245460u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00224354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2245464u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2245468u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2245472u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2245480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224368));
    } else {
        emu.pc = 2245476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224364));
    }
}
#[inline(always)]
pub fn block_0x00224364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2245480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224368));
}
#[inline(always)]
pub fn block_0x00224368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 40u32, 2245484u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2245488u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224714));
    } else {
        emu.pc = 2245492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224374));
    }
}
#[inline(always)]
pub fn block_0x00224374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243d8));
    } else {
        emu.pc = 2245496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224378));
    }
}
#[inline(always)]
pub fn block_0x00224378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2245500u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2245504u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2245508u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2245512u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2245516u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022438c));
}
#[inline]
pub fn block_0x0022438c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2245520u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2245524u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2245528u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2245532u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2245536u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2245540u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2245544u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2245548u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2245552u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2245556u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2245560u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2245516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022438c));
    } else {
        emu.pc = 2245564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243bc));
    }
}
#[inline(always)]
pub fn block_0x002243bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2245592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243d8));
    } else {
        emu.pc = 2245568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243c0));
    }
}
#[inline(always)]
pub fn block_0x002243c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224860));
    } else {
        emu.pc = 2245572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243c4));
    }
}
#[inline(always)]
pub fn block_0x002243c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2245576u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2245580u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2245584u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2245588u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2245592u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002243d8));
}
#[inline(always)]
pub fn block_0x002243d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 1336u32, 2245596u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2245604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243e4));
    } else {
        emu.pc = 2245600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243e0));
    }
}
#[inline(always)]
pub fn block_0x002243e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2245604u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002243e4));
}
#[inline(always)]
pub fn block_0x002243e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224714));
    } else {
        emu.pc = 2245608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243e8));
    }
}
#[inline(always)]
pub fn block_0x002243e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 10usize, 2u32, 2245612u32);
    emu.sbr_no_count(10usize, 0usize, 13usize, 2245616u32);
    emu.adi_no_count(12usize, 2usize, 1172u32, 2245620u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2245624u32);
    emu.adr_no_count(13usize, 7usize, 13usize, 2245628u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002243fc));
}
#[inline(always)]
pub fn block_0x002243fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224430));
    } else {
        emu.pc = 2245632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224400));
    }
}
#[inline(always)]
pub fn block_0x00224400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2245636u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2245640u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2245644u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2245648u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2245652u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2245628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002243fc));
    } else {
        emu.pc = 2245656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224418));
    }
}
#[inline(always)]
pub fn block_0x00224418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2245660u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2245664u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2245668u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2245672u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2245696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224440));
    } else {
        emu.pc = 2245676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022442c));
    }
}
#[inline(always)]
pub fn block_0x0022442c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2245680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245992u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224568));
}
#[inline(always)]
pub fn block_0x00224430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2245684u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2245688u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2245692u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2245992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224568));
    } else {
        emu.pc = 2245696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224440));
    }
}
#[inline(always)]
pub fn block_0x00224440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2245992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224568));
    } else {
        emu.pc = 2245700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224444));
    }
}
#[inline(always)]
pub fn block_0x00224444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2245796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244a4));
    } else {
        emu.pc = 2245704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224448));
    }
}
#[inline(always)]
pub fn block_0x00224448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2245708u32);
    emu.sli_no_count(13usize, 21usize, 2u32, 2245712u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2245716u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2245720u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2245724u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022445c));
}
#[inline]
pub fn block_0x0022445c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2245728u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2245732u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2245736u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2245740u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2245744u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2245748u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2245752u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2245756u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2245760u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2245724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022445c));
    } else {
        emu.pc = 2245764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224484));
    }
}
#[inline(always)]
pub fn block_0x00224484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2245768u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2245784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224498));
    } else {
        emu.pc = 2245772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022448c));
    }
}
#[inline(always)]
pub fn block_0x0022448c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224860));
    } else {
        emu.pc = 2245776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224490));
    }
}
#[inline(always)]
pub fn block_0x00224490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2245780u32)?;
    emu.adi_no_count(21usize, 21usize, 1u32, 2245784u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2245784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224498));
}
#[inline(always)]
pub fn block_0x00224498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 188u32, 2245788u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2245808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244b0));
    } else {
        emu.pc = 2245792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244a0));
    }
}
#[inline(always)]
pub fn block_0x002244a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2245796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002244fc));
}
#[inline(always)]
pub fn block_0x002244a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2245800u32)?;
    emu.sw_no_count(21usize, 2usize, 188u32, 2245804u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2245884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244fc));
    } else {
        emu.pc = 2245808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244b0));
    }
}
#[inline(always)]
pub fn block_0x002244b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2245812u32);
    emu.sli_no_count(13usize, 25usize, 2u32, 2245816u32);
    emu.adi_no_count(10usize, 2usize, 192u32, 2245820u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2245824u32);
    emu.adi_no_count(14usize, 2usize, 192u32, 2245828u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002244c4));
}
#[inline]
pub fn block_0x002244c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2245832u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2245836u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2245840u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2245844u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2245848u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2245852u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2245856u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2245860u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2245864u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2245828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244c4));
    } else {
        emu.pc = 2245868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244ec));
    }
}
#[inline(always)]
pub fn block_0x002244ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244fc));
    } else {
        emu.pc = 2245872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244f0));
    }
}
#[inline(always)]
pub fn block_0x002244f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224860));
    } else {
        emu.pc = 2245876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002244f4));
    }
}
#[inline(always)]
pub fn block_0x002244f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2245880u32)?;
    emu.adi_no_count(25usize, 25usize, 1u32, 2245884u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2245884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002244fc));
}
#[inline(always)]
pub fn block_0x002244fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 352u32, 2245888u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2245968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224550));
    } else {
        emu.pc = 2245892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224504));
    }
}
#[inline(always)]
pub fn block_0x00224504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2245896u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2245900u32);
    emu.adi_no_count(10usize, 2usize, 356u32, 2245904u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2245908u32);
    emu.adi_no_count(14usize, 2usize, 356u32, 2245912u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224518));
}
#[inline]
pub fn block_0x00224518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2245916u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2245920u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2245924u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2245928u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2245932u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2245936u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2245940u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2245944u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2245948u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2245912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224518));
    } else {
        emu.pc = 2245952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224540));
    }
}
#[inline(always)]
pub fn block_0x00224540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224550));
    } else {
        emu.pc = 2245956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224544));
    }
}
#[inline(always)]
pub fn block_0x00224544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224860));
    } else {
        emu.pc = 2245960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224548));
    }
}
#[inline(always)]
pub fn block_0x00224548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2245964u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2245968u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2245968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224550));
}
#[inline(always)]
pub fn block_0x00224550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2245972u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2245976u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2245980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2244652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2244652u32));
    } else {
        emu.pc = 2245984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224560));
    }
}
#[inline(always)]
pub fn block_0x00224560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2245988u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2245992u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2244652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2244652u32));
}
#[inline(always)]
pub fn block_0x00224568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2246220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022464c));
    } else {
        emu.pc = 2245996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022456c));
    }
}
#[inline(always)]
pub fn block_0x0022456c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 8u32, 2246000u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2246104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245d8));
    } else {
        emu.pc = 2246004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224574));
    }
}
#[inline(always)]
pub fn block_0x00224574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2246008u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2246012u32);
    emu.apc_no_count(1usize, 2246012u32, 4294942720u32, 2246016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246020u32;
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
pub fn block_0x00224584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 188u32, 2246024u32)?;
    emu.lw_no_count(10usize, 2usize, 680u32, 2246028u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2246036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224594));
    } else {
        emu.pc = 2246032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224590));
    }
}
#[inline(always)]
pub fn block_0x00224590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2246036u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2246036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224594));
}
#[inline(always)]
pub fn block_0x00224594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2246040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224714));
    } else {
        emu.pc = 2246044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022459c));
    }
}
#[inline(always)]
pub fn block_0x0022459c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 2u32, 2246048u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2246052u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2246056u32);
    emu.sbr_no_count(10usize, 0usize, 11usize, 2246060u32);
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2246064u32);
    emu.adr_no_count(11usize, 12usize, 14usize, 2246068u32);
    emu.adr_no_count(12usize, 13usize, 14usize, 2246072u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2246072u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002245b8));
}
#[inline(always)]
pub fn block_0x002245b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2246380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246ec));
    } else {
        emu.pc = 2246076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245bc));
    }
}
#[inline(always)]
pub fn block_0x002245bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2246080u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2246084u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2246088u32);
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2246092u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2246096u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2246072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245b8));
    } else {
        emu.pc = 2246100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245d4));
    }
}
#[inline(always)]
pub fn block_0x002245d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2246384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246f0));
    } else {
        emu.pc = 2246104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245d8));
    }
}
#[inline(always)]
pub fn block_0x002245d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2246108u32);
    emu.lw_no_count(21usize, 2usize, 20u32, 2246112u32)?;
    emu.adr_no_count(8usize, 21usize, 23usize, 2246116u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2246120u32);
    emu.adi_no_count(12usize, 0usize, 57u32, 2246124u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2246124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002245ec));
}
#[inline(always)]
pub fn block_0x002245ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 20usize, 11usize, 2246128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2246240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224660));
    } else {
        emu.pc = 2246132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245f4));
    }
}
#[inline(always)]
pub fn block_0x002245f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 19usize, 11usize, 2246136u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2246140u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2246144u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2246124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245ec));
    } else {
        emu.pc = 2246148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224604));
    }
}
#[inline(always)]
pub fn block_0x00224604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 20usize, 11usize, 2246152u32);
    emu.adr_no_count(12usize, 10usize, 21usize, 2246156u32);
    emu.lbu_no_count(13usize, 12usize, 1u32, 2246160u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2246164u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2246168u32);
    emu.sb_no_count(13usize, 12usize, 1u32, 2246172u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2246732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022484c));
    } else {
        emu.pc = 2246176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224620));
    }
}
#[inline(always)]
pub fn block_0x00224620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2246180u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2246184u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2246300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022469c));
    } else {
        emu.pc = 2246188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022462c));
    }
}
#[inline(always)]
pub fn block_0x0022462c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(12usize, 11usize, 4294967295u32, 2246192u32);
    emu.adr_no_count(10usize, 19usize, 11usize, 2246196u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2246200u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2246204u32);
    emu.apc_no_count(1usize, 2246204u32, 4294860800u32, 2246208u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246212u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2246304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246a0));
    } else {
        emu.pc = 2246216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224648));
    }
}
#[inline(always)]
pub fn block_0x00224648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2246220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2246396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002246fc));
}
#[inline(always)]
pub fn block_0x0022464c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2246224u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2246228u32)?;
    emu.lw_no_count(22usize, 2usize, 8u32, 2246232u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2246304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246a0));
    } else {
        emu.pc = 2246236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022465c));
    }
}
#[inline(always)]
pub fn block_0x0022465c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2246240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2246396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002246fc));
}
#[inline(always)]
pub fn block_0x00224660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2246440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224728));
    } else {
        emu.pc = 2246244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224664));
    }
}
#[inline(always)]
pub fn block_0x00224664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2246248u32);
    emu.sb_no_count(10usize, 21usize, 0u32, 2246252u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2246256u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2246476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022474c));
    } else {
        emu.pc = 2246260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224674));
    }
}
#[inline(always)]
pub fn block_0x00224674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 1u32, 2246264u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2246268u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2246272u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2246276u32);
    emu.apc_no_count(1usize, 2246276u32, 4294860800u32, 2246280u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246284u32;
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
pub fn block_0x0022468c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2246452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224734));
    } else {
        emu.pc = 2246288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224690));
    }
}
#[inline(always)]
pub fn block_0x00224690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2246292u32);
    emu.adi_no_count(23usize, 20usize, 2u32, 2246296u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2246300u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2246300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022469c));
}
#[inline(always)]
pub fn block_0x0022469c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2246396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246fc));
    } else {
        emu.pc = 2246304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246a0));
    }
}
#[inline]
pub fn block_0x002246a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2246308u32)?;
    emu.sw_no_count(21usize, 10usize, 0u32, 2246312u32)?;
    emu.sw_no_count(23usize, 10usize, 4u32, 2246316u32)?;
    emu.sh_no_count(22usize, 10usize, 8u32, 2246320u32)?;
    emu.lw_no_count(1usize, 2usize, 1388u32, 2246324u32)?;
    emu.lw_no_count(8usize, 2usize, 1384u32, 2246328u32)?;
    emu.lw_no_count(9usize, 2usize, 1380u32, 2246332u32)?;
    emu.lw_no_count(18usize, 2usize, 1376u32, 2246336u32)?;
    emu.lw_no_count(19usize, 2usize, 1372u32, 2246340u32)?;
    emu.lw_no_count(20usize, 2usize, 1368u32, 2246344u32)?;
    emu.lw_no_count(21usize, 2usize, 1364u32, 2246348u32)?;
    emu.lw_no_count(22usize, 2usize, 1360u32, 2246352u32)?;
    emu.lw_no_count(23usize, 2usize, 1356u32, 2246356u32)?;
    emu.lw_no_count(24usize, 2usize, 1352u32, 2246360u32)?;
    emu.lw_no_count(25usize, 2usize, 1348u32, 2246364u32)?;
    emu.lw_no_count(26usize, 2usize, 1344u32, 2246368u32)?;
    emu.lw_no_count(27usize, 2usize, 1340u32, 2246372u32)?;
    emu.adi_no_count(2usize, 2usize, 1392u32, 2246376u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246380u32;
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
pub fn block_0x002246ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2246104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002245d8));
    } else {
        emu.pc = 2246384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246f0));
    }
}
#[inline(always)]
pub fn block_0x002246f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2246388u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2246392u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2246304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246a0));
    } else {
        emu.pc = 2246396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002246fc));
    }
}
#[inline(always)]
pub fn block_0x002246fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246400u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966680u32, 2246404u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2246408u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2246412u32);
    emu.apc_no_count(1usize, 2246412u32, 4096u32, 2246416u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246420u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246424u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2246428u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2246432u32);
    emu.apc_no_count(1usize, 2246432u32, 4096u32, 2246436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246440u32;
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
pub fn block_0x00224728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2246444u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2246448u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2246288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224690));
    } else {
        emu.pc = 2246452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224734));
    }
}
#[inline(always)]
pub fn block_0x00224734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246456u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966664u32, 2246460u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2246464u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2246468u32);
    emu.apc_no_count(1usize, 2246468u32, 4294938624u32, 2246472u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022474c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2246480u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2246288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224690));
    } else {
        emu.pc = 2246484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224754));
    }
}
#[inline(always)]
pub fn block_0x00224754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2246488u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2246452u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224734));
}
#[inline(always)]
pub fn block_0x00224758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246492u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1644u32, 2246496u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246500u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2246504u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2246508u32);
    emu.apc_no_count(1usize, 2246508u32, 4294938624u32, 2246512u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246516u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00224774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246520u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2246524u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2246528u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2246532u32);
    emu.apc_no_count(1usize, 2246532u32, 4096u32, 2246536u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022478c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246544u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2246548u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2246552u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2246556u32);
    emu.apc_no_count(1usize, 2246556u32, 4096u32, 2246560u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002247a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246568u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966448u32, 2246572u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246576u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966476u32, 2246580u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2246584u32);
    emu.apc_no_count(1usize, 2246584u32, 4294938624u32, 2246588u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002247c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246596u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966492u32, 2246600u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246604u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966524u32, 2246608u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2246612u32);
    emu.apc_no_count(1usize, 2246612u32, 4294938624u32, 2246616u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002247dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246624u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966540u32, 2246628u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246632u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966568u32, 2246636u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2246640u32);
    emu.apc_no_count(1usize, 2246640u32, 4294938624u32, 2246644u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002247f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246652u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966768u32, 2246656u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246660u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966824u32, 2246664u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2246668u32);
    emu.apc_no_count(1usize, 2246668u32, 4294938624u32, 2246672u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246680u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966696u32, 2246684u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246688u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966752u32, 2246692u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2246696u32);
    emu.apc_no_count(1usize, 2246696u32, 4294938624u32, 2246700u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246704u32;
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
pub fn block_0x00224830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2246708u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966584u32, 2246712u32);
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246716u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966632u32, 2246720u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2246724u32);
    emu.apc_no_count(1usize, 2246724u32, 4294938624u32, 2246728u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246732u32;
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
pub fn block_0x0022484c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246736u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1776u32, 2246740u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2246744u32);
    emu.apc_no_count(1usize, 2246744u32, 4096u32, 2246748u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246756u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2246760u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2246764u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2246768u32);
    emu.apc_no_count(1usize, 2246768u32, 4294938624u32, 2246772u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2246780u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966648u32, 2246784u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2246788u32);
    emu.adi_no_count(11usize, 24usize, 0u32, 2246792u32);
    emu.apc_no_count(1usize, 2246792u32, 4294938624u32, 2246796u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2246800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00224890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966416u32, 2246804u32);
    emu.sw_no_count(1usize, 2usize, 876u32, 2246808u32)?;
    emu.sw_no_count(8usize, 2usize, 872u32, 2246812u32)?;
    emu.sw_no_count(9usize, 2usize, 868u32, 2246816u32)?;
    emu.sw_no_count(18usize, 2usize, 864u32, 2246820u32)?;
    emu.sw_no_count(19usize, 2usize, 860u32, 2246824u32)?;
    emu.sw_no_count(20usize, 2usize, 856u32, 2246828u32)?;
    emu.sw_no_count(21usize, 2usize, 852u32, 2246832u32)?;
    emu.sw_no_count(22usize, 2usize, 848u32, 2246836u32)?;
    emu.sw_no_count(23usize, 2usize, 844u32, 2246840u32)?;
    emu.sw_no_count(24usize, 2usize, 840u32, 2246844u32)?;
    emu.sw_no_count(25usize, 2usize, 836u32, 2246848u32)?;
    emu.sw_no_count(26usize, 2usize, 832u32, 2246852u32)?;
    emu.sw_no_count(27usize, 2usize, 828u32, 2246856u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2246860u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2246864u32);
    emu.lw_no_count(5usize, 11usize, 0u32, 2246868u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2246872u32)?;
    emu.orr_no_count(14usize, 5usize, 13usize, 2246876u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2249772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249772u32));
    } else {
        emu.pc = 2246880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002248e0));
    }
}
#[inline(always)]
pub fn block_0x002248e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2246884u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2246888u32)?;
    emu.lw_no_count(10usize, 11usize, 8u32, 2246892u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2246896u32)?;
    emu.orr_no_count(15usize, 10usize, 14usize, 2246900u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2249800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249800u32));
    } else {
        emu.pc = 2246904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002248f8));
    }
}
#[inline(always)]
pub fn block_0x002248f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 16u32, 2246908u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2246912u32)?;
    emu.orr_no_count(17usize, 15usize, 16usize, 2246916u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2249828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249828u32));
    } else {
        emu.pc = 2246920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224908));
    }
}
#[inline(always)]
pub fn block_0x00224908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 5usize, 15usize, 2246924u32);
    emu.sltru_no_count(15usize, 15usize, 5usize, 2246928u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2246932u32);
    emu.adr_no_count(16usize, 16usize, 15usize, 2246936u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2246944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224920));
    } else {
        emu.pc = 2246940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022491c));
    }
}
#[inline(always)]
pub fn block_0x0022491c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2246944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2246944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224920));
}
#[inline(always)]
pub fn block_0x00224920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2249856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249856u32));
    } else {
        emu.pc = 2246948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224924));
    }
}
#[inline(always)]
pub fn block_0x00224924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2246960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224930));
    } else {
        emu.pc = 2246952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224928));
    }
}
#[inline(always)]
pub fn block_0x00224928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 14usize, 2246956u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2246960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2246964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224934));
}
#[inline(always)]
pub fn block_0x00224930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 10usize, 2246964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2246964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224934));
}
#[inline(always)]
pub fn block_0x00224934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2249884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2249884u32));
    } else {
        emu.pc = 2246968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224938));
    }
}
#[inline]
pub fn block_0x00224938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(20usize, 11usize, 24u32, 2246972u32)?;
    emu.sltiu_no_count(10usize, 5usize, 1u32, 2246976u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2246980u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2246984u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2246988u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 13usize, 10usize, 2246992u32);
    emu.adi_no_count(15usize, 11usize, 1365u32, 2246996u32);
    emu.adi_no_count(14usize, 14usize, 819u32, 2247000u32);
    emu.adi_no_count(10usize, 17usize, 4294967055u32, 2247004u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2247008u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 257u32, 2247012u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2247120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002249d0));
    } else {
        emu.pc = 2247016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224968));
    }
}
#[inline(never)]
pub fn block_0x00224968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 5usize, 4294967295u32, 2247020u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2247024u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247028u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2247032u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247036u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2247040u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247044u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2247048u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247052u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2247056u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247060u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2247064u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2247068u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2247072u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2247076u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2247080u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2247084u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2247088u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2247092u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2247096u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2247100u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2247104u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2247108u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2247112u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2247116u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2247120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2247212u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224a2c));
}
#[inline]
pub fn block_0x002249d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 1u32, 2247124u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247128u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2247132u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247136u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2247140u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247144u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2247148u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247152u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2247156u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2247160u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2247164u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2247168u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2247172u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2247176u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2247180u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2247184u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2247188u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2247192u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2247196u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2247200u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2247204u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2247208u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2247212u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2247212u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224a2c));
}
#[inline(never)]
pub fn block_0x00224a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2247216u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2247220u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2247224u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2247228u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2247232u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2247236u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2247240u32);
    emu.sbr_no_count(11usize, 11usize, 14usize, 2247244u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2247248u32);
    emu.anr_no_count(13usize, 14usize, 13usize, 2247252u32);
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2247256u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 128u32, 2247260u32);
    emu.sw_no_count(11usize, 2usize, 168u32, 2247264u32)?;
    emu.adr_no_count(14usize, 10usize, 14usize, 2247268u32);
    emu.sw_no_count(5usize, 2usize, 8u32, 2247272u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2247276u32)?;
    emu.sltru_no_count(10usize, 14usize, 10usize, 2247280u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2247284u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2247288u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2247292u32);
    emu.sai_no_count(22usize, 10usize, 1040u32, 2247296u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2247300u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2247304u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2247308u32);
    emu.apc_no_count(1usize, 2247308u32, 4294860800u32, 2247312u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 176u32, 2247320u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2247324u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2247328u32);
    emu.apc_no_count(1usize, 2247328u32, 4294860800u32, 2247332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2247340u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2247344u32)?;
    emu.sw_no_count(10usize, 2usize, 172u32, 2247348u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2247388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224adc));
    } else {
        emu.pc = 2247352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224ab8));
    }
}
#[inline(always)]
pub fn block_0x00224ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2247356u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2247360u32);
    emu.apc_no_count(1usize, 2247360u32, 4294938624u32, 2247364u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2247416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224af8));
    } else {
        emu.pc = 2247372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224acc));
    }
}
#[inline(always)]
pub fn block_0x00224acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2247376u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2247380u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2247384u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2247388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2247432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224b08));
}
#[inline(always)]
pub fn block_0x00224adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 20usize, 2247392u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2247396u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2247400u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2247404u32);
    emu.apc_no_count(1usize, 2247404u32, 4294938624u32, 2247408u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00224af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2247372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224acc));
    } else {
        emu.pc = 2247416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224af8));
    }
}
#[inline(always)]
pub fn block_0x00224af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 22usize, 2247420u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2247424u32);
    emu.sri_no_count(11usize, 10usize, 16u32, 2247428u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2247432u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2247432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224b08));
}
#[inline(always)]
pub fn block_0x00224b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2247432u32, 4294963200u32, 2247436u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2247440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
