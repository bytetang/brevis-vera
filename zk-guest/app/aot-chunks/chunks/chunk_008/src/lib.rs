pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2119324u32;
pub const PC_MAX: u32 = 2121912u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 111usize] = [
        block_0x0020569c,
        block_0x002056b0,
        block_0x002056b8,
        block_0x002056ec,
        block_0x002056f4,
        block_0x002056fc,
        block_0x00205704,
        block_0x0020570c,
        block_0x00205714,
        block_0x0020573c,
        block_0x0020579c,
        block_0x002057a4,
        block_0x002057b4,
        block_0x002057bc,
        block_0x002057c0,
        block_0x002057f0,
        block_0x002057f8,
        block_0x00205810,
        block_0x00205818,
        block_0x0020582c,
        block_0x00205844,
        block_0x002058a4,
        block_0x002058ac,
        block_0x002058cc,
        block_0x0020592c,
        block_0x00205930,
        block_0x00205934,
        block_0x0020593c,
        block_0x00205944,
        block_0x0020594c,
        block_0x00205978,
        block_0x00205980,
        block_0x00205988,
        block_0x0020598c,
        block_0x002059b0,
        block_0x002059bc,
        block_0x00205a50,
        block_0x00205aa4,
        block_0x00205aa8,
        block_0x00205ac4,
        block_0x00205b60,
        block_0x00205b8c,
        block_0x00205b98,
        block_0x00205ba4,
        block_0x00205bac,
        block_0x00205bd0,
        block_0x00205bdc,
        block_0x00205be8,
        block_0x00205c24,
        block_0x00205c40,
        block_0x00205c70,
        block_0x00205c90,
        block_0x00205cb4,
        block_0x00205cdc,
        block_0x00205ce0,
        block_0x00205ce8,
        block_0x00205cf8,
        block_0x00205d08,
        block_0x00205d30,
        block_0x00205d58,
        block_0x00205d6c,
        block_0x00205d84,
        block_0x00205de8,
        block_0x00205df0,
        block_0x00205e00,
        block_0x00205e04,
        block_0x00205e34,
        block_0x00205e44,
        block_0x00205e64,
        block_0x00205e74,
        block_0x00205e78,
        block_0x00205e9c,
        block_0x00205eac,
        block_0x00205ec8,
        block_0x00205ed8,
        block_0x00205ee0,
        block_0x00205ef4,
        block_0x00205ef8,
        block_0x00205f08,
        block_0x00205f0c,
        block_0x00205f14,
        block_0x00205f24,
        block_0x00205f28,
        block_0x00205f34,
        block_0x00205f38,
        block_0x00205f3c,
        block_0x00205f58,
        block_0x00205f60,
        block_0x00205f64,
        block_0x00205f90,
        block_0x00205f94,
        block_0x00205fa0,
        block_0x00205fb0,
        block_0x00205fb4,
        block_0x00205fbc,
        block_0x00205fc8,
        block_0x00205fdc,
        block_0x00205ff4,
        block_0x00205ffc,
        block_0x00206020,
        block_0x00206028,
        block_0x00206034,
        block_0x0020605c,
        block_0x00206060,
        block_0x00206078,
        block_0x00206084,
        block_0x00206090,
        block_0x002060a4,
        block_0x002060a8,
        block_0x002060b4,
        block_0x002060b8,
    ];
    const IDX: [u16; 648usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 0u16,
        7u16, 0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 12u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 27u16, 0u16, 28u16,
        0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 31u16, 0u16, 32u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 45u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 55u16, 0u16,
        56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16,
        0u16, 77u16, 78u16, 0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 82u16, 83u16, 0u16, 0u16, 84u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 87u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 93u16,
        94u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 108u16, 109u16,
        0u16, 0u16, 110u16, 111u16,
    ];
    if pc < 2119324u32 || pc > 2121912u32 {
        return None;
    }
    let word_offset = ((pc - 2119324u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020569c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2119328u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2119332u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2119336u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2119340u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2119588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057a4));
    } else {
        emu.pc = 2119344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056b0));
    }
}
#[inline(always)]
pub fn block_0x002056b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2119348u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2119352u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2119352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002056b8));
}
#[inline]
pub fn block_0x002056b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2119356u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2119360u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2119364u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2119368u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2119372u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2119376u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2119380u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2119384u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2119388u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2119392u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2119396u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2119400u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2119352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056b8));
    } else {
        emu.pc = 2119404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056ec));
    }
}
#[inline(always)]
pub fn block_0x002056ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2119408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2119604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057b4));
    } else {
        emu.pc = 2119412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056f4));
    }
}
#[inline(always)]
pub fn block_0x002056f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2119416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205934));
    } else {
        emu.pc = 2119420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056fc));
    }
}
#[inline(always)]
pub fn block_0x002056fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2119424u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2119724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020582c));
    } else {
        emu.pc = 2119428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205704));
    }
}
#[inline(always)]
pub fn block_0x00205704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2119432u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2119852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002058ac));
    } else {
        emu.pc = 2119436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020570c));
    }
}
#[inline(always)]
pub fn block_0x0020570c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2119440u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2119988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205934));
    } else {
        emu.pc = 2119444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205714));
    }
}
#[inline]
pub fn block_0x00205714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2119448u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2119452u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2119456u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2119460u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2119464u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2119468u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2119472u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2119476u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2119480u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2119484u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2119484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020573c));
}
#[inline]
pub fn block_0x0020573c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2119488u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2119492u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2119496u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2119500u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2119504u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2119508u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2119512u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2119516u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2119520u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2119524u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2119528u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2119532u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2119536u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2119540u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2119544u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2119548u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2119552u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2119556u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2119560u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2119564u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2119568u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2119572u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2119576u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2119484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020573c));
    } else {
        emu.pc = 2119580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020579c));
    }
}
#[inline(always)]
pub fn block_0x0020579c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2119584u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2119588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205930));
}
#[inline(always)]
pub fn block_0x002057a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2119592u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2119596u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2119600u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2119412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002056f4));
    } else {
        emu.pc = 2119604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057b4));
    }
}
#[inline(always)]
pub fn block_0x002057b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2119608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2119664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057f0));
    } else {
        emu.pc = 2119612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057bc));
    }
}
#[inline(always)]
pub fn block_0x002057bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2119616u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002057c0));
}
#[inline]
pub fn block_0x002057c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2119620u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2119624u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2119628u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2119632u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2119636u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2119640u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2119644u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2119648u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2119652u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2119656u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2119660u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2119616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057c0));
    } else {
        emu.pc = 2119664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057f0));
    }
}
#[inline(always)]
pub fn block_0x002057f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2119668u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2119696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205810));
    } else {
        emu.pc = 2119672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002057f8));
    }
}
#[inline(always)]
pub fn block_0x002057f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2119676u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2119680u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2119684u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2119688u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2119692u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2119696u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2119696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205810));
}
#[inline(always)]
pub fn block_0x00205810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2119700u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2120056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205978));
    } else {
        emu.pc = 2119704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205818));
    }
}
#[inline(always)]
pub fn block_0x00205818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2119708u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2119712u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2119716u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2119720u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2119724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205978));
}
#[inline(always)]
pub fn block_0x0020582c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2119728u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2119732u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2119736u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2119740u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2119744u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2119748u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2119748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205844));
}
#[inline]
pub fn block_0x00205844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2119752u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2119756u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2119760u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2119764u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2119768u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2119772u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2119776u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2119780u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2119784u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2119788u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2119792u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2119796u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2119800u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2119804u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2119808u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2119812u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2119816u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2119820u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2119824u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2119828u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2119832u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2119836u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2119840u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2119748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205844));
    } else {
        emu.pc = 2119844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002058a4));
    }
}
#[inline(always)]
pub fn block_0x002058a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2119848u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2119852u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2119984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205930));
}
#[inline(always)]
pub fn block_0x002058ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2119856u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2119860u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2119864u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2119868u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2119872u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2119876u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2119880u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2119884u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2119884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002058cc));
}
#[inline]
pub fn block_0x002058cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2119888u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2119892u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2119896u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2119900u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2119904u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2119908u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2119912u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2119916u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2119920u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2119924u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2119928u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2119932u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2119936u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2119940u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2119944u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2119948u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2119952u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2119956u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2119960u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2119964u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2119968u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2119972u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2119976u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2119884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002058cc));
    } else {
        emu.pc = 2119980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020592c));
    }
}
#[inline(always)]
pub fn block_0x0020592c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2119984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205930));
}
#[inline(always)]
pub fn block_0x00205930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2119988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2119988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205934));
}
#[inline(always)]
pub fn block_0x00205934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2119992u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002059bc));
    } else {
        emu.pc = 2119996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020593c));
    }
}
#[inline(always)]
pub fn block_0x0020593c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2120000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a50));
    } else {
        emu.pc = 2120004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205944));
    }
}
#[inline(always)]
pub fn block_0x00205944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2120008u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2120056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205978));
    } else {
        emu.pc = 2120012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020594c));
    }
}
#[inline]
pub fn block_0x0020594c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2120016u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2120020u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2120024u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2120028u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2120032u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2120036u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2120040u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2120044u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2120048u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2120052u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2120056u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2120056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205978));
}
#[inline(always)]
pub fn block_0x00205978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2120060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020598c));
    } else {
        emu.pc = 2120064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205980));
    }
}
#[inline(always)]
pub fn block_0x00205980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2120068u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002059b0));
    } else {
        emu.pc = 2120072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205988));
    }
}
#[inline(always)]
pub fn block_0x00205988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120076u32;
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
pub fn block_0x0020598c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2120080u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2120084u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2120088u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2120092u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2120096u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2120100u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2120104u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2120108u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2120072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205988));
    } else {
        emu.pc = 2120112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002059b0));
    }
}
#[inline(always)]
pub fn block_0x002059b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2120116u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2120120u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120124u32;
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
#[inline(never)]
pub fn block_0x002059bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2120128u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2120132u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2120136u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2120140u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2120144u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2120148u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2120152u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2120156u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2120160u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2120164u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2120168u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2120172u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2120176u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2120180u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2120184u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2120188u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2120192u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2120196u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2120200u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2120204u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2120208u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2120212u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2120216u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2120220u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2120224u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2120228u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2120232u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2120236u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2120240u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2120244u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2120248u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2120252u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2120256u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2120260u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2120264u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2120268u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2120004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205944));
    } else {
        emu.pc = 2120272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205a50));
    }
}
#[inline]
pub fn block_0x00205a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2120276u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2120280u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2120284u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2120288u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2120292u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2120296u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2120300u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2120304u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2120308u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2120312u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2120316u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2120320u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2120324u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2120328u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2120332u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2120336u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2120340u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2120344u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2120348u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2120352u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2120012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020594c));
    } else {
        emu.pc = 2120356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205aa4));
    }
}
#[inline(always)]
pub fn block_0x00205aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2120360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2120056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205978));
}
#[inline(always)]
pub fn block_0x00205aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(3usize, 2120360u32, 4292915200u32, 2120364u32);
    emu.adi_no_count(3usize, 3usize, 4294966616u32, 2120368u32);
    emu.apc_no_count(2usize, 2120368u32, 81920u32, 2120372u32);
    emu.adi_no_count(2usize, 2usize, 1844u32, 2120376u32);
    emu.lw_no_count(2usize, 2usize, 0u32, 2120380u32)?;
    emu.apc_no_count(1usize, 2120380u32, 0u32, 2120384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120392u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120396u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2120400u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2120404u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2120408u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2120412u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 0usize, 1u32, 2120416u32);
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120420u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120424u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2120428u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2120432u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2120436u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2120440u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 8usize, 4294967120u32, 2120444u32);
    emu.adi_no_count(10usize, 10usize, 1639u32, 2120448u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2120452u32);
    emu.sw_no_count(9usize, 18usize, 40u32, 2120456u32)?;
    emu.sw_no_count(0usize, 18usize, 44u32, 2120460u32)?;
    emu.sw_no_count(10usize, 18usize, 48u32, 2120464u32)?;
    emu.sw_no_count(11usize, 18usize, 52u32, 2120468u32)?;
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120472u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 12usize, 882u32, 2120476u32);
    emu.adi_no_count(12usize, 13usize, 1338u32, 2120480u32);
    emu.adi_no_count(13usize, 14usize, 639u32, 2120484u32);
    emu.adi_no_count(14usize, 15usize, 4294965388u32, 2120488u32);
    emu.sw_no_count(11usize, 18usize, 56u32, 2120492u32)?;
    emu.sw_no_count(12usize, 18usize, 60u32, 2120496u32)?;
    emu.sw_no_count(13usize, 18usize, 64u32, 2120500u32)?;
    emu.sw_no_count(14usize, 18usize, 68u32, 2120504u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120508u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965675u32, 2120512u32);
    emu.adi_no_count(11usize, 11usize, 4294966553u32, 2120516u32);
    emu.sw_no_count(10usize, 18usize, 72u32, 2120520u32)?;
    emu.sw_no_count(11usize, 18usize, 76u32, 2120524u32)?;
    emu.adi_no_count(10usize, 18usize, 80u32, 2120528u32);
    emu.adi_no_count(12usize, 0usize, 73u32, 2120532u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2120536u32);
    emu.apc_no_count(1usize, 2120536u32, 0u32, 2120540u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 4294967120u32, 2120548u32)?;
    emu.sw_no_count(0usize, 8usize, 4294967124u32, 2120552u32)?;
    emu.sw_no_count(0usize, 18usize, 8u32, 2120556u32)?;
    emu.sw_no_count(0usize, 18usize, 12u32, 2120560u32)?;
    emu.sw_no_count(0usize, 18usize, 16u32, 2120564u32)?;
    emu.sw_no_count(0usize, 18usize, 20u32, 2120568u32)?;
    emu.sw_no_count(0usize, 18usize, 24u32, 2120572u32)?;
    emu.sw_no_count(0usize, 18usize, 28u32, 2120576u32)?;
    emu.sw_no_count(0usize, 18usize, 32u32, 2120580u32)?;
    emu.apc_no_count(1usize, 2120580u32, 4294959104u32, 2120584u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2120592u32);
    emu.apc_no_count(1usize, 2120592u32, 4294963200u32, 2120596u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120604u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294967112u32, 2120608u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2120620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bac));
    } else {
        emu.pc = 2120612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ba4));
    }
}
#[inline(always)]
pub fn block_0x00205ba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 52u32, 2120620u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2120620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205bac));
}
#[inline]
pub fn block_0x00205bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967295u32, 2120624u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2120628u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2120632u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2120636u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2120640u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2120644u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2120648u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2120652u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2120680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205be8));
    } else {
        emu.pc = 2120656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bd0));
    }
}
#[inline(always)]
pub fn block_0x00205bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120660u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2120664u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2120680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205be8));
    } else {
        emu.pc = 2120668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205bdc));
    }
}
#[inline(always)]
pub fn block_0x00205bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120672u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 11usize, 4294967112u32, 2120676u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120680u32;
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
pub fn block_0x00205be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2120684u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2120688u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 460u32, 2120692u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2120696u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2120700u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2120704u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2120708u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2120712u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2120716u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2120720u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2120724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 468u32, 2120728u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2120732u32);
    emu.apc_no_count(1usize, 2120732u32, 40960u32, 2120736u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120740u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2120744u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 488u32, 2120748u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2120752u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2120756u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2120760u32);
    emu.apc_no_count(6usize, 2120760u32, 61440u32, 2120764u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2120768u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00205c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2120772u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2120776u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2120780u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2120784u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2120788u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2120792u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2120796u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2120800u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2120804u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2120808u32);
    emu.apc_no_count(1usize, 2120808u32, 4294963200u32, 2120812u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2120820u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2120824u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2120828u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2120832u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2120836u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2120840u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2120844u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120848u32;
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
pub fn block_0x00205c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2120852u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2120856u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2120860u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2120864u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2120868u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2120872u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2120876u32);
    emu.apc_no_count(1usize, 2120876u32, 0u32, 2120880u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2120888u32);
    emu.ani_no_count(10usize, 10usize, 3u32, 2120892u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2120896u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2120900u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2120904u32);
    emu.ani_no_count(18usize, 10usize, 4294967292u32, 2120908u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2120912u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2120916u32);
    emu.apc_no_count(1usize, 2120916u32, 36864u32, 2120920u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d30));
    } else {
        emu.pc = 2120928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ce0));
    }
}
#[inline(always)]
pub fn block_0x00205ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2120928u32, 4294963200u32, 2120932u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120936u32;
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
pub fn block_0x00205ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2120940u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2120944u32);
    emu.apc_no_count(1usize, 2120944u32, 4294950912u32, 2120948u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2120956u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2120960u32);
    emu.apc_no_count(1usize, 2120960u32, 0u32, 2120964u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2120968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00205d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 0u32, 2120972u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2120976u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2120980u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2120984u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2120988u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2120992u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2120996u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2121000u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2121004u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121008u32;
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
pub fn block_0x00205d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2121012u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 516u32, 2121016u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121020u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 500u32, 2121024u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121028u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 660u32, 2121032u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2121036u32);
    emu.adi_no_count(12usize, 2usize, 11u32, 2121040u32);
    emu.apc_no_count(1usize, 2121040u32, 53248u32, 2121044u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2121052u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2121056u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2121060u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2121064u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d84));
    } else {
        emu.pc = 2121068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205d6c));
    }
}
#[inline(always)]
pub fn block_0x00205d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2121072u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2121076u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2121080u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2121084u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2121088u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121092u32;
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
#[inline(never)]
pub fn block_0x00205d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2121096u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121100u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1532u32, 2121104u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121108u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966804u32, 2121112u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121116u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 348u32, 2121120u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121124u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 724u32, 2121128u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2121132u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2121136u32)?;
    emu.adi_no_count(16usize, 2usize, 44u32, 2121140u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2121144u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2121148u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2121152u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2121156u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2121160u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2121164u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2121168u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2121172u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2121176u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2121180u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2121184u32);
    emu.apc_no_count(1usize, 2121184u32, 32768u32, 2121188u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2121192u32, 4294963200u32, 2121196u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121200u32;
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
pub fn block_0x00205df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2121204u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2121208u32);
    emu.apc_no_count(1usize, 2121208u32, 4294950912u32, 2121212u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967168u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e34));
    } else {
        emu.pc = 2121220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e04));
    }
}
#[inline]
pub fn block_0x00205e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2121224u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2121228u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2121232u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2121236u32)?;
    emu.sw_no_count(10usize, 12usize, 0u32, 2121240u32)?;
    emu.sw_no_count(11usize, 12usize, 4u32, 2121244u32)?;
    emu.sw_no_count(13usize, 12usize, 8u32, 2121248u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2121252u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2121256u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2121260u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2121264u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121268u32;
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
pub fn block_0x00205e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2121272u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2121276u32);
    emu.apc_no_count(1usize, 2121276u32, 32768u32, 2121280u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121288u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121292u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2121296u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2121300u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2121304u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2121308u32)?;
    emu.apc_no_count(1usize, 2121308u32, 4294963200u32, 2121312u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2121320u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2121324u32);
    emu.apc_no_count(1usize, 2121324u32, 4294950912u32, 2121328u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e9c));
    } else {
        emu.pc = 2121336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205e78));
    }
}
#[inline]
pub fn block_0x00205e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121340u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 10usize, 0u32, 2121344u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2121348u32)?;
    emu.sw_no_count(9usize, 10usize, 8u32, 2121352u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2121356u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2121360u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2121364u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121368u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121372u32;
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
pub fn block_0x00205e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2121376u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2121380u32);
    emu.apc_no_count(1usize, 2121380u32, 32768u32, 2121384u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121392u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121396u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2121400u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2121404u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2121408u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2121412u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2121524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f34));
    } else {
        emu.pc = 2121416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ec8));
    }
}
#[inline(always)]
pub fn block_0x00205ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2121420u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2121424u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2121428u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2121480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f08));
    } else {
        emu.pc = 2121432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ed8));
    }
}
#[inline(always)]
pub fn block_0x00205ed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2121436u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2121480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f08));
    } else {
        emu.pc = 2121440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ee0));
    }
}
#[inline(always)]
pub fn block_0x00205ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2121444u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2121448u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2121452u32);
    emu.apc_no_count(1usize, 2121452u32, 4294950912u32, 2121456u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f28));
    } else {
        emu.pc = 2121464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ef8));
    }
}
#[inline(always)]
pub fn block_0x00205ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2121468u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2121472u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2121476u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2121480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f3c));
}
#[inline(always)]
pub fn block_0x00205f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f58));
    } else {
        emu.pc = 2121484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f0c));
    }
}
#[inline(always)]
pub fn block_0x00205f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2121484u32, 4294963200u32, 2121488u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121492u32;
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
pub fn block_0x00205f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2121496u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2121500u32);
    emu.apc_no_count(1usize, 2121500u32, 4294950912u32, 2121504u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ef8));
    } else {
        emu.pc = 2121512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f28));
    }
}
#[inline(always)]
pub fn block_0x00205f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2121516u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2121520u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2121524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121528u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f38));
}
#[inline(always)]
pub fn block_0x00205f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2121528u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121528u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f38));
}
#[inline(always)]
pub fn block_0x00205f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2121532u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f3c));
}
#[inline(always)]
pub fn block_0x00205f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2121536u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2121540u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2121544u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2121548u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2121552u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121556u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121560u32;
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
pub fn block_0x00205f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2121564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2121464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ef8));
    } else {
        emu.pc = 2121568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f60));
    }
}
#[inline(always)]
pub fn block_0x00205f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2121572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f28));
}
#[inline]
pub fn block_0x00205f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2121576u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2121580u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2121584u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2121588u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2121592u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2121596u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2121600u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2121604u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2121608u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2121612u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2121620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f94));
    } else {
        emu.pc = 2121616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f90));
    }
}
#[inline(always)]
pub fn block_0x00205f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2121620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205f94));
}
#[inline(always)]
pub fn block_0x00205f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 13usize, 29u32, 2121624u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2121628u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2121660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fbc));
    } else {
        emu.pc = 2121632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fa0));
    }
}
#[inline(always)]
pub fn block_0x00205fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 18usize, 2u32, 2121636u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121640u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2121644u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2121768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206028));
    } else {
        emu.pc = 2121648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fb0));
    }
}
#[inline(always)]
pub fn block_0x00205fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fc8));
    } else {
        emu.pc = 2121652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205fb4));
    }
}
#[inline(always)]
pub fn block_0x00205fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2121656u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2121660u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205fdc));
}
#[inline(always)]
pub fn block_0x00205fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2121664u32);
    emu.apc_no_count(1usize, 2121664u32, 32768u32, 2121668u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121672u32;
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
pub fn block_0x00205fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2121676u32)?;
    emu.sli_no_count(13usize, 13usize, 2u32, 2121680u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2121684u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2121688u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2121692u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2121692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00205fdc));
}
#[inline(always)]
pub fn block_0x00205fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2121696u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2121700u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2121704u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2121708u32);
    emu.apc_no_count(1usize, 2121708u32, 0u32, 2121712u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2121720u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2121760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206020));
    } else {
        emu.pc = 2121724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205ffc));
    }
}
#[inline]
pub fn block_0x00205ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2121728u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2121732u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2121736u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2121740u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2121744u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2121748u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2121752u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2121756u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121760u32;
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
pub fn block_0x00206020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2121764u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2121768u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2121768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206028));
}
#[inline(always)]
pub fn block_0x00206028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2121772u32);
    emu.apc_no_count(1usize, 2121772u32, 32768u32, 2121776u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2121784u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2121788u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2121792u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2121796u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2121800u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2121804u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2121808u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2121812u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2121816u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2121848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206078));
    } else {
        emu.pc = 2121820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020605c));
    }
}
#[inline(always)]
pub fn block_0x0020605c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2121824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206060));
}
#[inline(always)]
pub fn block_0x00206060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121828u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 808u32, 2121832u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2121836u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2121840u32);
    emu.apc_no_count(1usize, 2121840u32, 32768u32, 2121844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121848u32;
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
pub fn block_0x00206078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2121852u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2121856u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2121908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002060b4));
    } else {
        emu.pc = 2121860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206084));
    }
}
#[inline(always)]
pub fn block_0x00206084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 12usize, 0u32, 2121864u32);
    emu.apc_no_count(1usize, 2121864u32, 4294963200u32, 2121868u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121872u32;
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
pub fn block_0x00206090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2121876u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2121880u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2121884u32);
    emu.apc_no_count(1usize, 2121884u32, 4294950912u32, 2121888u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121892u32;
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
pub fn block_0x002060a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2121824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206060));
    } else {
        emu.pc = 2121896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002060a8));
    }
}
#[inline(always)]
pub fn block_0x002060a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2121900u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2121904u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2121908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2121912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002060b8));
}
#[inline(always)]
pub fn block_0x002060b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2121912u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2121912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002060b8));
}
#[inline(always)]
pub fn block_0x002060b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2121916u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2121920u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2121924u32);
    emu.apc_no_count(1usize, 2121924u32, 4294963200u32, 2121928u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
