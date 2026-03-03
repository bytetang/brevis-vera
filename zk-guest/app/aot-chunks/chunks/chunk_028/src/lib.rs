pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2221376u32;
pub const PC_MAX: u32 = 2223696u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 117usize] = [
        block_0x0021e540,
        block_0x0021e56c,
        block_0x0021e58c,
        block_0x0021e594,
        block_0x0021e5b4,
        block_0x0021e5e4,
        block_0x0021e620,
        block_0x0021e658,
        block_0x0021e674,
        block_0x0021e684,
        block_0x0021e690,
        block_0x0021e694,
        block_0x0021e6bc,
        block_0x0021e6cc,
        block_0x0021e71c,
        block_0x0021e720,
        block_0x0021e738,
        block_0x0021e73c,
        block_0x0021e74c,
        block_0x0021e750,
        block_0x0021e76c,
        block_0x0021e770,
        block_0x0021e778,
        block_0x0021e78c,
        block_0x0021e794,
        block_0x0021e7ac,
        block_0x0021e7b4,
        block_0x0021e7d0,
        block_0x0021e7d8,
        block_0x0021e7e8,
        block_0x0021e7f0,
        block_0x0021e800,
        block_0x0021e808,
        block_0x0021e810,
        block_0x0021e830,
        block_0x0021e84c,
        block_0x0021e864,
        block_0x0021e868,
        block_0x0021e878,
        block_0x0021e87c,
        block_0x0021e884,
        block_0x0021e8b8,
        block_0x0021e8cc,
        block_0x0021e8dc,
        block_0x0021e8e8,
        block_0x0021e8ec,
        block_0x0021e90c,
        block_0x0021e91c,
        block_0x0021e970,
        block_0x0021e974,
        block_0x0021e990,
        block_0x0021e994,
        block_0x0021e99c,
        block_0x0021e9b0,
        block_0x0021e9bc,
        block_0x0021e9cc,
        block_0x0021e9d0,
        block_0x0021e9d4,
        block_0x0021ea04,
        block_0x0021ea20,
        block_0x0021ea28,
        block_0x0021ea30,
        block_0x0021ea38,
        block_0x0021ea40,
        block_0x0021ea50,
        block_0x0021ea78,
        block_0x0021ea84,
        block_0x0021eaac,
        block_0x0021eab4,
        block_0x0021eab8,
        block_0x0021ead0,
        block_0x0021eb00,
        block_0x0021eb10,
        block_0x0021eb20,
        block_0x0021eb50,
        block_0x0021eb60,
        block_0x0021eb70,
        block_0x0021eb84,
        block_0x0021eba4,
        block_0x0021ebb8,
        block_0x0021ebcc,
        block_0x0021ebf8,
        block_0x0021ec00,
        block_0x0021ec1c,
        block_0x0021ec40,
        block_0x0021ec44,
        block_0x0021ec48,
        block_0x0021ec5c,
        block_0x0021ec74,
        block_0x0021ec7c,
        block_0x0021ec94,
        block_0x0021ec98,
        block_0x0021ecac,
        block_0x0021ecc4,
        block_0x0021eccc,
        block_0x0021ece4,
        block_0x0021ed10,
        block_0x0021ed14,
        block_0x0021ed30,
        block_0x0021ed34,
        block_0x0021ed3c,
        block_0x0021ed48,
        block_0x0021ed50,
        block_0x0021ed6c,
        block_0x0021ed88,
        block_0x0021ed9c,
        block_0x0021edb4,
        block_0x0021edbc,
        block_0x0021edd4,
        block_0x0021edd8,
        block_0x0021edec,
        block_0x0021ee04,
        block_0x0021ee0c,
        block_0x0021ee24,
        block_0x0021ee44,
        block_0x0021ee4c,
        block_0x0021ee50,
    ];
    const IDX: [u16; 581usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 15u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16, 0u16,
        0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 29u16, 0u16,
        0u16, 0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16,
        39u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16,
        0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 57u16, 58u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 0u16, 64u16,
        0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16,
        0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 87u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 101u16, 0u16, 0u16, 102u16,
        0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16,
        0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 113u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 115u16,
        0u16, 116u16, 117u16,
    ];
    if pc < 2221376u32 || pc > 2223696u32 {
        return None;
    }
    let word_offset = ((pc - 2221376u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021e540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2221380u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2221384u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2221388u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2221392u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2221396u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2221400u32)?;
    emu.lw_no_count(9usize, 10usize, 8u32, 2221404u32)?;
    emu.lbu_no_count(12usize, 9usize, 0u32, 2221408u32);
    emu.lw_no_count(8usize, 10usize, 0u32, 2221412u32)?;
    emu.lw_no_count(18usize, 10usize, 4u32, 2221416u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2221492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5b4));
    } else {
        emu.pc = 2221420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e56c));
    }
}
#[inline(always)]
pub fn block_0x0021e56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 18usize, 12u32, 2221424u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221428u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 616u32, 2221432u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2221436u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2221440u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2221444u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2221448u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2221452u32;
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
pub fn block_0x0021e58c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2221456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2221492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e5b4));
    } else {
        emu.pc = 2221460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e594));
    }
}
#[inline(always)]
pub fn block_0x0021e594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2221464u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2221468u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2221472u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2221476u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2221480u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2221484u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2221488u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221492u32;
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
pub fn block_0x0021e5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 4294967286u32, 2221496u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2221500u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2221504u32);
    emu.lw_no_count(6usize, 18usize, 16u32, 2221508u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2221512u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2221516u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2221520u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2221524u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2221528u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2221532u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2221536u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2221540u32;
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
pub fn block_0x0021e5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2221544u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2221548u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2221552u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2221556u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2221560u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2221564u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2221568u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2221572u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2221576u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2221580u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2221584u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2221588u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2221592u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2221596u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2221656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e658));
    } else {
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    }
}
#[inline]
pub fn block_0x0021e620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 8usize, 4u32, 2221604u32);
    emu.sb_no_count(21usize, 8usize, 5u32, 2221608u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2221612u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2221616u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2221620u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2221624u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2221628u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2221632u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2221636u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2221640u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2221644u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2221648u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2221652u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221656u32;
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
pub fn block_0x0021e658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 14usize, 0u32, 2221660u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2221664u32);
    emu.lw_no_count(19usize, 8usize, 0u32, 2221668u32)?;
    emu.lbu_no_count(10usize, 8usize, 5u32, 2221672u32);
    emu.lbu_no_count(13usize, 19usize, 10u32, 2221676u32);
    emu.ani_no_count(13usize, 13usize, 128u32, 2221680u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2221712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e690));
    } else {
        emu.pc = 2221684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e674));
    }
}
#[inline(always)]
pub fn block_0x0021e674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 10usize, 3u32, 2221688u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2221692u32);
    emu.adi_no_count(23usize, 12usize, 0u32, 2221696u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e770));
    } else {
        emu.pc = 2221700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e684));
    }
}
#[inline(always)]
pub fn block_0x0021e684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221704u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1752u32, 2221708u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2221712u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221944u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e778));
}
#[inline(always)]
pub fn block_0x0021e690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2221772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e6cc));
    } else {
        emu.pc = 2221716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e694));
    }
}
#[inline]
pub fn block_0x0021e694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 19usize, 4u32, 2221720u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2221724u32)?;
    emu.lw_no_count(14usize, 13usize, 12u32, 2221728u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2221732u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1759u32, 2221736u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2221740u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2221744u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2221748u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2221752u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2221756u32;
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
pub fn block_0x0021e6bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2221760u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2221764u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2221768u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2221772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e6cc));
    }
}
#[inline]
pub fn block_0x0021e6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2221776u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2221780u32);
    emu.lw_no_count(13usize, 19usize, 0u32, 2221784u32)?;
    emu.lw_no_count(14usize, 19usize, 4u32, 2221788u32)?;
    emu.lw_no_count(15usize, 19usize, 8u32, 2221792u32)?;
    emu.lw_no_count(16usize, 19usize, 12u32, 2221796u32)?;
    emu.adi_no_count(17usize, 2usize, 12u32, 2221800u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2221804u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2221808u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2221812u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2221816u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1728u32, 2221820u32);
    emu.sb_no_count(20usize, 2usize, 27u32, 2221824u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2221828u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2221832u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2221836u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2221840u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2221844u32);
    emu.apc_no_count(1usize, 2221844u32, 0u32, 2221848u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e71c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2221856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e720));
    }
}
#[inline(always)]
pub fn block_0x0021e720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221860u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1757u32, 2221864u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2221868u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2221872u32);
    emu.apc_no_count(1usize, 2221872u32, 0u32, 2221876u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2221884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e73c));
    }
}
#[inline(always)]
pub fn block_0x0021e73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2221888u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2221892u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2221896u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2221900u32;
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
pub fn block_0x0021e74c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2221904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e750));
    }
}
#[inline(always)]
pub fn block_0x0021e750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2221908u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2221912u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2221916u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221920u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1762u32, 2221924u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2221928u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2221932u32;
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
pub fn block_0x0021e76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221936u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e7e8));
}
#[inline(always)]
pub fn block_0x0021e770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221940u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1755u32, 2221944u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2221944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e778));
}
#[inline(always)]
pub fn block_0x0021e778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 4u32, 2221948u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2221952u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2221956u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2221960u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2221964u32;
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
pub fn block_0x0021e78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2221968u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2221972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e794));
    }
}
#[inline(always)]
pub fn block_0x0021e794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 0u32, 2221976u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2221980u32);
    emu.lw_no_count(13usize, 19usize, 4u32, 2221984u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2221988u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2221992u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2221996u32;
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
pub fn block_0x0021e7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2222000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2222004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e7b4));
    }
}
#[inline(always)]
pub fn block_0x0021e7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 19usize, 4u32, 2222008u32)?;
    emu.lw_no_count(10usize, 19usize, 0u32, 2222012u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2222016u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222020u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1757u32, 2222024u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2222028u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2222032u32;
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
pub fn block_0x0021e7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2222036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e620));
    } else {
        emu.pc = 2222040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e7d8));
    }
}
#[inline(always)]
pub fn block_0x0021e7d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 12u32, 2222044u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2222048u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2222052u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2222056u32;
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
pub fn block_0x0021e7e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2222060u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2222064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e620));
}
#[inline(always)]
pub fn block_0x0021e7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2222068u32);
    emu.lbu_no_count(12usize, 10usize, 5u32, 2222072u32);
    emu.lbu_no_count(10usize, 10usize, 4u32, 2222076u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2222204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e87c));
    } else {
        emu.pc = 2222080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e800));
    }
}
#[inline(always)]
pub fn block_0x0021e800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2222084u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2222096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e810));
    } else {
        emu.pc = 2222088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e808));
    }
}
#[inline(always)]
pub fn block_0x0021e808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2222092u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2222096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e878));
}
#[inline(always)]
pub fn block_0x0021e810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2222100u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2222104u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2222108u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2222112u32);
    emu.lw_no_count(10usize, 11usize, 0u32, 2222116u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2222120u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2222124u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e84c));
    } else {
        emu.pc = 2222128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e830));
    }
}
#[inline(always)]
pub fn block_0x0021e830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2222132u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2222136u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2222140u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222144u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1765u32, 2222148u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2222152u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2222156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e864));
}
#[inline(always)]
pub fn block_0x0021e84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2222160u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2222164u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2222168u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1764u32, 2222176u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2222180u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2222180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e864));
}
#[inline(always)]
pub fn block_0x0021e864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2222184u32;
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
pub fn block_0x0021e868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2222188u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2222192u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2222196u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2222200u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2222200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e878));
}
#[inline(always)]
pub fn block_0x0021e878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 11usize, 4u32, 2222204u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e87c));
}
#[inline(always)]
pub fn block_0x0021e87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2222208u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222212u32;
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
pub fn block_0x0021e884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2222216u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2222220u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2222224u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2222228u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2222232u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2222236u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2222240u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2222244u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2222248u32);
    emu.lbu_no_count(11usize, 10usize, 8u32, 2222252u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2222256u32)?;
    emu.adi_no_count(8usize, 0usize, 1u32, 2222260u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9d4));
    } else {
        emu.pc = 2222264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8b8));
    }
}
#[inline(always)]
pub fn block_0x0021e8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 4u32, 2222268u32)?;
    emu.lbu_no_count(11usize, 18usize, 10u32, 2222272u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2222276u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2222280u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8e8));
    } else {
        emu.pc = 2222284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8cc));
    }
}
#[inline(always)]
pub fn block_0x0021e8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 0usize, 19usize, 2222288u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2222292u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2222296u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2222484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e994));
    } else {
        emu.pc = 2222300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8dc));
    }
}
#[inline(always)]
pub fn block_0x0021e8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222304u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1755u32, 2222308u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2222312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e99c));
}
#[inline(always)]
pub fn block_0x0021e8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2222364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e91c));
    } else {
        emu.pc = 2222316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e8ec));
    }
}
#[inline(always)]
pub fn block_0x0021e8ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2222320u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2222324u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2222328u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1768u32, 2222336u32);
    emu.adi_no_count(21usize, 12usize, 0u32, 2222340u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2222344u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2222348u32;
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
pub fn block_0x0021e90c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 21usize, 0u32, 2222352u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2222356u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2222360u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9d4));
    } else {
        emu.pc = 2222364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e91c));
    }
}
#[inline]
pub fn block_0x0021e91c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2222368u32);
    emu.adi_no_count(10usize, 2usize, 19u32, 2222372u32);
    emu.lw_no_count(11usize, 18usize, 0u32, 2222376u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2222380u32)?;
    emu.lw_no_count(14usize, 18usize, 8u32, 2222384u32)?;
    emu.lw_no_count(15usize, 18usize, 12u32, 2222388u32)?;
    emu.adi_no_count(16usize, 2usize, 4u32, 2222392u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2222396u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2222400u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2222404u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2222408u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1728u32, 2222412u32);
    emu.lw_no_count(12usize, 12usize, 12u32, 2222416u32)?;
    emu.sb_no_count(8usize, 2usize, 19u32, 2222420u32);
    emu.sw_no_count(16usize, 2usize, 20u32, 2222424u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2222428u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2222432u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2222436u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2222440u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2222444u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2222448u32;
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
pub fn block_0x0021e970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2222544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9d0));
    } else {
        emu.pc = 2222452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e974));
    }
}
#[inline(always)]
pub fn block_0x0021e974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2222456u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2222460u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2222464u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222468u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1762u32, 2222472u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2222476u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2222480u32;
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
pub fn block_0x0021e990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2222484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9cc));
}
#[inline(always)]
pub fn block_0x0021e994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222488u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1767u32, 2222492u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2222492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e99c));
}
#[inline(always)]
pub fn block_0x0021e99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 4u32, 2222496u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2222500u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2222504u32)?;
    emu.adi_no_count(12usize, 13usize, 0u32, 2222508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2222512u32;
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
pub fn block_0x0021e9b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2222516u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2222520u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9d4));
    } else {
        emu.pc = 2222524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e9bc));
    }
}
#[inline(always)]
pub fn block_0x0021e9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 12u32, 2222528u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2222532u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2222536u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2222540u32;
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
pub fn block_0x0021e9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2222544u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9d0));
}
#[inline(always)]
pub fn block_0x0021e9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2222548u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e9d4));
}
#[inline]
pub fn block_0x0021e9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2222552u32);
    emu.sw_no_count(19usize, 10usize, 0u32, 2222556u32)?;
    emu.sb_no_count(8usize, 10usize, 8u32, 2222560u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2222564u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2222568u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2222572u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2222576u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2222580u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2222584u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2222588u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2222592u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222596u32;
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
pub fn block_0x0021ea04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2222600u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2222604u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2222608u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2222612u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2222616u32)?;
    emu.lbu_no_count(8usize, 10usize, 8u32, 2222620u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2222776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eab8));
    } else {
        emu.pc = 2222624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea20));
    }
}
#[inline(always)]
pub fn block_0x0021ea20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(8usize, 8usize, 1u32, 2222628u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2222640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea30));
    } else {
        emu.pc = 2222632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea28));
    }
}
#[inline(always)]
pub fn block_0x0021ea28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2222636u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2222640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2222772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eab4));
}
#[inline(always)]
pub fn block_0x0021ea30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2222644u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea84));
    } else {
        emu.pc = 2222648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea38));
    }
}
#[inline(always)]
pub fn block_0x0021ea38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 9u32, 2222652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2222724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea84));
    } else {
        emu.pc = 2222656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea40));
    }
}
#[inline(always)]
pub fn block_0x0021ea40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2222660u32)?;
    emu.lbu_no_count(12usize, 11usize, 10u32, 2222664u32);
    emu.ani_no_count(12usize, 12usize, 128u32, 2222668u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2222724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea84));
    } else {
        emu.pc = 2222672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea50));
    }
}
#[inline]
pub fn block_0x0021ea50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2222676u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2222680u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2222684u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1771u32, 2222692u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2222696u32);
    emu.adi_no_count(8usize, 0usize, 1u32, 2222700u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2222704u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2222708u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2222712u32;
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
pub fn block_0x0021ea78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2222716u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2222720u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2222772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eab4));
    } else {
        emu.pc = 2222724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ea84));
    }
}
#[inline]
pub fn block_0x0021ea84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2222728u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2222732u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2222736u32)?;
    emu.lw_no_count(14usize, 12usize, 12u32, 2222740u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222744u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1770u32, 2222748u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2222752u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2222756u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2222760u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2222764u32;
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
pub fn block_0x0021eaac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2222768u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2222772u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2222772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eab4));
}
#[inline(always)]
pub fn block_0x0021eab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 10usize, 8u32, 2222776u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2222776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021eab8));
}
#[inline(always)]
pub fn block_0x0021eab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 8usize, 1u32, 2222780u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2222784u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2222788u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2222792u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2222796u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222800u32;
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
pub fn block_0x0021ead0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966192u32, 2222804u32);
    emu.sw_no_count(1usize, 2usize, 1100u32, 2222808u32)?;
    emu.sw_no_count(8usize, 2usize, 1096u32, 2222812u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2222816u32);
    emu.adi_no_count(5usize, 0usize, 4u32, 2222820u32);
    emu.adi_no_count(10usize, 2usize, 1080u32, 2222824u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2222828u32);
    emu.adi_no_count(16usize, 0usize, 1024u32, 2222832u32);
    emu.adi_no_count(17usize, 2usize, 1032u32, 2222836u32);
    emu.sw_no_count(5usize, 2usize, 0u32, 2222840u32)?;
    emu.apc_no_count(1usize, 2222840u32, 4096u32, 2222844u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222848u32;
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
#[inline(always)]
pub fn block_0x0021eb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1080u32, 2222852u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2222856u32);
    emu.apc_no_count(1usize, 2222856u32, 12288u32, 2222860u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222864u32;
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
#[inline(always)]
pub fn block_0x0021eb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 1100u32, 2222868u32)?;
    emu.lw_no_count(8usize, 2usize, 1096u32, 2222872u32)?;
    emu.adi_no_count(2usize, 2usize, 1104u32, 2222876u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222880u32;
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
pub fn block_0x0021eb20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2222884u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2222888u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2222892u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2222896u32);
    emu.adi_no_count(5usize, 0usize, 4u32, 2222900u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2222904u32);
    emu.adi_no_count(15usize, 2usize, 7u32, 2222908u32);
    emu.adi_no_count(16usize, 0usize, 17u32, 2222912u32);
    emu.adi_no_count(17usize, 2usize, 24u32, 2222916u32);
    emu.sw_no_count(5usize, 2usize, 0u32, 2222920u32)?;
    emu.apc_no_count(1usize, 2222920u32, 0u32, 2222924u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222928u32;
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
pub fn block_0x0021eb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 72u32, 2222932u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2222936u32);
    emu.apc_no_count(1usize, 2222936u32, 12288u32, 2222940u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021eb60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2222948u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2222952u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2222956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2222960u32;
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
pub fn block_0x0021eb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2222964u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2222968u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1728u32, 2222972u32);
    emu.apc_no_count(6usize, 2222972u32, 12288u32, 2222976u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2222980u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021eb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 0u32, 2222984u32);
    emu.lw_no_count(13usize, 11usize, 8u32, 2222988u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2222992u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2222996u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2223000u32;
    emu.update_insn_clock();
    emu.sli_no_count(14usize, 13usize, 3u32, 2223004u32);
    emu.anr_no_count(10usize, 13usize, 10usize, 2223008u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebb8));
    } else {
        emu.pc = 2223012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eba4));
    }
}
#[inline(always)]
pub fn block_0x0021eba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 0usize, 10usize, 2223016u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2223020u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2223024u32);
    emu.apc_no_count(6usize, 2223024u32, 0u32, 2223028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2223032u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ebb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 15usize, 14u32, 2223036u32)?;
    emu.sltru_no_count(13usize, 0usize, 10usize, 2223040u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2223044u32);
    emu.apc_no_count(6usize, 2223044u32, 0u32, 2223048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2223052u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021ebcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967136u32, 2223056u32);
    emu.sw_no_count(1usize, 2usize, 156u32, 2223060u32)?;
    emu.sw_no_count(8usize, 2usize, 152u32, 2223064u32)?;
    emu.sw_no_count(9usize, 2usize, 148u32, 2223068u32)?;
    emu.sw_no_count(18usize, 2usize, 144u32, 2223072u32)?;
    emu.sw_no_count(19usize, 2usize, 140u32, 2223076u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2223080u32);
    emu.lw_no_count(11usize, 11usize, 8u32, 2223084u32)?;
    emu.sli_no_count(12usize, 11usize, 6u32, 2223088u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2223092u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec48));
    } else {
        emu.pc = 2223096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ebf8));
    }
}
#[inline(always)]
pub fn block_0x0021ebf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 5u32, 2223100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec98));
    } else {
        emu.pc = 2223104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec00));
    }
}
#[inline(always)]
pub fn block_0x0021ec00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 0u32, 2223108u32)?;
    emu.adi_no_count(18usize, 2usize, 12u32, 2223112u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2223116u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223120u32);
    emu.adi_no_count(19usize, 0usize, 10u32, 2223124u32);
    emu.apc_no_count(1usize, 2223124u32, 4294959104u32, 2223128u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223132u32;
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
#[inline]
pub fn block_0x0021ec1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2223136u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2223140u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2223144u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2223148u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2223152u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2223156u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2223160u32);
    emu.apc_no_count(1usize, 2223160u32, 12288u32, 2223164u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ec40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed14));
    } else {
        emu.pc = 2223172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec44));
    }
}
#[inline(always)]
pub fn block_0x0021ec44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2223176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee50));
}
#[inline(always)]
pub fn block_0x0021ec48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2223180u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2223184u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2223188u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223192u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2223196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ec74));
}
#[inline(always)]
pub fn block_0x0021ec5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2223200u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223204u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223208u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223212u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223216u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2223332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ece4));
    } else {
        emu.pc = 2223220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec74));
    }
}
#[inline(always)]
pub fn block_0x0021ec74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2223224u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2223196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec5c));
    } else {
        emu.pc = 2223228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec7c));
    }
}
#[inline(always)]
pub fn block_0x0021ec7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2223232u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223236u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223240u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223244u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223248u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2223220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec74));
    } else {
        emu.pc = 2223252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ec94));
    }
}
#[inline(always)]
pub fn block_0x0021ec94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2223256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ece4));
}
#[inline(always)]
pub fn block_0x0021ec98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2223260u32);
    emu.lw_no_count(10usize, 9usize, 0u32, 2223264u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2223268u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223272u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2223276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ecc4));
}
#[inline(always)]
pub fn block_0x0021ecac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2223280u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223284u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223288u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223292u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223296u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2223332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ece4));
    } else {
        emu.pc = 2223300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecc4));
    }
}
#[inline(always)]
pub fn block_0x0021ecc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2223304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2223276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecac));
    } else {
        emu.pc = 2223308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021eccc));
    }
}
#[inline(always)]
pub fn block_0x0021eccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2223312u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223316u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223320u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223324u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223328u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2223300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ecc4));
    } else {
        emu.pc = 2223332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ece4));
    }
}
#[inline]
pub fn block_0x0021ece4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2223336u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2223340u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2223344u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2223348u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2223352u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2223356u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2223360u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2223364u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2223368u32);
    emu.apc_no_count(1usize, 2223368u32, 12288u32, 2223372u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ed10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee50));
    } else {
        emu.pc = 2223380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed14));
    }
}
#[inline(always)]
pub fn block_0x0021ed14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2223384u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2223388u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2223392u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2223396u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1772u32, 2223400u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2223404u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2223408u32;
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
pub fn block_0x0021ed30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2223420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed3c));
    } else {
        emu.pc = 2223412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed34));
    }
}
#[inline(always)]
pub fn block_0x0021ed34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2223416u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2223420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee50));
}
#[inline(always)]
pub fn block_0x0021ed3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 8u32, 2223424u32)?;
    emu.sli_no_count(11usize, 10usize, 6u32, 2223428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed88));
    } else {
        emu.pc = 2223432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed48));
    }
}
#[inline(always)]
pub fn block_0x0021ed48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 5u32, 2223436u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2223576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edd8));
    } else {
        emu.pc = 2223440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed50));
    }
}
#[inline(always)]
pub fn block_0x0021ed50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2223444u32)?;
    emu.adi_no_count(9usize, 2usize, 12u32, 2223448u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2223452u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223456u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2223460u32);
    emu.apc_no_count(1usize, 2223460u32, 4294959104u32, 2223464u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ed6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2223472u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2223476u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2223480u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2223484u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2223488u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2223492u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2223496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee44));
}
#[inline(always)]
pub fn block_0x0021ed88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2223500u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2223504u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2223508u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223512u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2223516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021edb4));
}
#[inline(always)]
pub fn block_0x0021ed9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2223520u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223524u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223528u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223532u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223536u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2223652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee24));
    } else {
        emu.pc = 2223540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edb4));
    }
}
#[inline(always)]
pub fn block_0x0021edb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2223544u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2223516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ed9c));
    } else {
        emu.pc = 2223548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edbc));
    }
}
#[inline(always)]
pub fn block_0x0021edbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2223552u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223556u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223560u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223564u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223568u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2223540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edb4));
    } else {
        emu.pc = 2223572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edd4));
    }
}
#[inline(always)]
pub fn block_0x0021edd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2223576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee24));
}
#[inline(always)]
pub fn block_0x0021edd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 0u32, 2223580u32);
    emu.lw_no_count(10usize, 9usize, 4u32, 2223584u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2223588u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2223592u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2223596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2223620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee04));
}
#[inline(always)]
pub fn block_0x0021edec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2223600u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223604u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223608u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223612u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223616u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2223652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee24));
    } else {
        emu.pc = 2223620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee04));
    }
}
#[inline(always)]
pub fn block_0x0021ee04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2223624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2223596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021edec));
    } else {
        emu.pc = 2223628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee0c));
    }
}
#[inline(always)]
pub fn block_0x0021ee0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2223632u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2223636u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2223640u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2223644u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2223648u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2223620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee04));
    } else {
        emu.pc = 2223652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ee24));
    }
}
#[inline(always)]
pub fn block_0x0021ee24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2223656u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2223660u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2223664u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2223668u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2223672u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2223676u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2223680u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2223684u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2223684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee44));
}
#[inline(always)]
pub fn block_0x0021ee44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2223684u32, 12288u32, 2223688u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ee4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2223696u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2223696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ee50));
}
#[inline(always)]
pub fn block_0x0021ee50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2223700u32);
    emu.lw_no_count(1usize, 2usize, 156u32, 2223704u32)?;
    emu.lw_no_count(8usize, 2usize, 152u32, 2223708u32)?;
    emu.lw_no_count(9usize, 2usize, 148u32, 2223712u32)?;
    emu.lw_no_count(18usize, 2usize, 144u32, 2223716u32)?;
    emu.lw_no_count(19usize, 2usize, 140u32, 2223720u32)?;
    emu.adi_no_count(2usize, 2usize, 160u32, 2223724u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2223728u32;
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
