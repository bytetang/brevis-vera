pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2225868u32;
pub const PC_MAX: u32 = 2228476u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0021f6cc,
        block_0x0021f6d8,
        block_0x0021f6e4,
        block_0x0021f6f0,
        block_0x0021f6f4,
        block_0x0021f6f8,
        block_0x0021f704,
        block_0x0021f70c,
        block_0x0021f718,
        block_0x0021f720,
        block_0x0021f73c,
        block_0x0021f770,
        block_0x0021f778,
        block_0x0021f77c,
        block_0x0021f788,
        block_0x0021f790,
        block_0x0021f79c,
        block_0x0021f7a4,
        block_0x0021f7b0,
        block_0x0021f7bc,
        block_0x0021f7d4,
        block_0x0021f8c8,
        block_0x0021f8d4,
        block_0x0021f8e0,
        block_0x0021f8e4,
        block_0x0021f8e8,
        block_0x0021f900,
        block_0x0021f914,
        block_0x0021f924,
        block_0x0021f92c,
        block_0x0021f934,
        block_0x0021f980,
        block_0x0021f994,
        block_0x0021f9a4,
        block_0x0021f9b8,
        block_0x0021f9bc,
        block_0x0021f9c0,
        block_0x0021f9c4,
        block_0x0021f9cc,
        block_0x0021f9d0,
        block_0x0021f9d4,
        block_0x0021f9dc,
        block_0x0021f9ec,
        block_0x0021fa14,
        block_0x0021fa84,
        block_0x0021fa94,
        block_0x0021fb04,
        block_0x0021fb08,
        block_0x0021fb2c,
        block_0x0021fb3c,
        block_0x0021fb50,
        block_0x0021fb6c,
        block_0x0021fb70,
        block_0x0021fb90,
        block_0x0021fba4,
        block_0x0021fbb4,
        block_0x0021fbc8,
        block_0x0021fbe4,
        block_0x0021fbe8,
        block_0x0021fc04,
        block_0x0021fc44,
        block_0x0021fd80,
        block_0x0021fd88,
        block_0x0021fd98,
        block_0x0021fdec,
        block_0x0021fdf4,
        block_0x0021fe00,
        block_0x0021fe08,
        block_0x0021fe14,
        block_0x0021fe24,
        block_0x0021fe2c,
        block_0x0021fe34,
        block_0x0021fe40,
        block_0x0021fe44,
        block_0x0021fe4c,
        block_0x0021fe58,
        block_0x0021fe68,
        block_0x0021fe70,
        block_0x0021fe78,
        block_0x0021fe7c,
        block_0x0021fe84,
        block_0x0021fe94,
        block_0x0021fe98,
        block_0x0021fe9c,
        block_0x0021ff0c,
        block_0x0021ff14,
        block_0x0021ff34,
        block_0x0021ff4c,
        block_0x0021ff54,
        block_0x0021ff58,
        block_0x0021ff78,
        block_0x0021ff80,
        block_0x0021ff88,
        block_0x0021ff9c,
        block_0x0021ffac,
        block_0x0021ffb4,
        block_0x0021ffbc,
        block_0x0021ffd4,
        block_0x0021fff0,
        block_0x0021fff4,
        block_0x00220010,
        block_0x00220018,
        block_0x00220028,
        block_0x00220050,
        block_0x0022006c,
        block_0x002200b8,
        block_0x002200c0,
        block_0x002200c8,
        block_0x002200fc,
    ];
    const IDX: [u16; 653usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 5u16, 6u16, 0u16,
        0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 13u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16,
        0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 24u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 37u16, 38u16, 0u16,
        39u16, 40u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 0u16, 0u16, 67u16, 0u16, 68u16,
        0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16,
        73u16, 74u16, 0u16, 75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        78u16, 0u16, 79u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 83u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 107u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
    ];
    if pc < 2225868u32 || pc > 2228476u32 {
        return None;
    }
    let word_offset = ((pc - 2225868u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021f6cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 3u32, 2225872u32);
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2225876u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2225892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6e4));
    } else {
        emu.pc = 2225880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6d8));
    }
}
#[inline(always)]
pub fn block_0x0021f6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2225884u32);
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2225888u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2225892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2225952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f720));
}
#[inline(always)]
pub fn block_0x0021f6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 13usize, 11usize, 2225896u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2225900u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2225908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6f4));
    } else {
        emu.pc = 2225904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6f0));
    }
}
#[inline(always)]
pub fn block_0x0021f6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 0u32, 2225908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2225908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f6f4));
}
#[inline(always)]
pub fn block_0x0021f6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2225944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f718));
    } else {
        emu.pc = 2225912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f6f8));
    }
}
#[inline(always)]
pub fn block_0x0021f6f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2225916u32);
    emu.sbr_no_count(15usize, 0usize, 13usize, 2225920u32);
    emu.adi_no_count(16usize, 11usize, 0u32, 2225924u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2225924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f704));
}
#[inline(always)]
pub fn block_0x0021f704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 16usize, 0u32, 2225928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2226096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f7b0));
    } else {
        emu.pc = 2225932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f70c));
    }
}
#[inline(always)]
pub fn block_0x0021f70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2225936u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2225940u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2225924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f704));
    } else {
        emu.pc = 2225944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f718));
    }
}
#[inline(always)]
pub fn block_0x0021f718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2225948u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2226040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f778));
    } else {
        emu.pc = 2225952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f720));
    }
}
#[inline(always)]
pub fn block_0x0021f720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2225956u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 11usize, 4u32, 2225960u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2225964u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 256u32, 2225968u32);
    emu.adi_no_count(17usize, 16usize, 1u32, 2225972u32);
    emu.mul_no_count(17usize, 10usize, 17usize, 2225976u32);
    emu.adi_no_count(5usize, 5usize, 128u32, 2225980u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2225980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f73c));
}
#[inline]
pub fn block_0x0021f73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 11usize, 13usize, 2225984u32);
    emu.adr_no_count(7usize, 15usize, 13usize, 2225988u32);
    emu.lw_no_count(6usize, 6usize, 0u32, 2225992u32)?;
    emu.lw_no_count(7usize, 7usize, 0u32, 2225996u32)?;
    emu.xrr_no_count(6usize, 6usize, 17usize, 2226000u32);
    emu.xrr_no_count(7usize, 7usize, 17usize, 2226004u32);
    emu.sbr_no_count(28usize, 16usize, 6usize, 2226008u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2226012u32);
    emu.sbr_no_count(28usize, 16usize, 7usize, 2226016u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2226020u32);
    emu.anr_no_count(6usize, 6usize, 7usize, 2226024u32);
    emu.anr_no_count(6usize, 6usize, 5usize, 2226028u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a != b {
        emu.pc = 2226040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f778));
    } else {
        emu.pc = 2226032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f770));
    }
}
#[inline(always)]
pub fn block_0x0021f770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2226036u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2225980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f73c));
    } else {
        emu.pc = 2226040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f778));
    }
}
#[inline(always)]
pub fn block_0x0021f778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2226076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f79c));
    } else {
        emu.pc = 2226044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f77c));
    }
}
#[inline(always)]
pub fn block_0x0021f77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 11usize, 13usize, 2226048u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2226052u32);
    emu.sbr_no_count(12usize, 0usize, 12usize, 2226056u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2226056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f788));
}
#[inline(always)]
pub fn block_0x0021f788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 14usize, 0u32, 2226060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2226084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f7a4));
    } else {
        emu.pc = 2226064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f790));
    }
}
#[inline(always)]
pub fn block_0x0021f790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2226068u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2226072u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2226056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f788));
    } else {
        emu.pc = 2226076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f79c));
    }
}
#[inline(always)]
pub fn block_0x0021f79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2226080u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2226084u32;
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
pub fn block_0x0021f7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 11usize, 2226088u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2226092u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2226096u32;
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
pub fn block_0x0021f7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 14usize, 2226100u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2226104u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2226108u32;
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
pub fn block_0x0021f7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2226112u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2226116u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966063u32, 2226120u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2226124u32);
    emu.apc_no_count(1usize, 2226124u32, 4294959104u32, 2226128u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2226132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021f7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2226136u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2226140u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966108u32, 2226144u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2226148u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2226152u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2226156u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2226160u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226164u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2226168u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2226172u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2226176u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2226180u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2226184u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2226188u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2226192u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2226196u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226200u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2226204u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2226208u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2226212u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2226216u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2226220u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2226224u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2226228u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226232u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2226236u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2226240u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2226244u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2226248u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2226252u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2226256u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2226260u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226264u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2226268u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2226272u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2226276u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2226280u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2226284u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2226288u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226292u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2226296u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2226300u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2226304u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2226308u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2226312u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2226316u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2226320u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2226324u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2226328u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2226332u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2226336u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2226340u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2226344u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2226348u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2226352u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2226356u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2226360u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2226364u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2226368u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2226372u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2226404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f8e4));
    } else {
        emu.pc = 2226376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f8c8));
    }
}
#[inline(always)]
pub fn block_0x0021f8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2226380u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2226384u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2226408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f8e8));
    } else {
        emu.pc = 2226388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f8d4));
    }
}
#[inline(always)]
pub fn block_0x0021f8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2226392u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2226396u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2226432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f900));
    } else {
        emu.pc = 2226400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f8e0));
    }
}
#[inline(always)]
pub fn block_0x0021f8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2226404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2226476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f92c));
}
#[inline(always)]
pub fn block_0x0021f8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2226408u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2226408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f8e8));
}
#[inline(always)]
pub fn block_0x0021f8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2226412u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2226416u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2226420u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2226424u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2226428u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2226476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f92c));
    } else {
        emu.pc = 2226432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f900));
    }
}
#[inline(always)]
pub fn block_0x0021f900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2226436u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2226440u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2226444u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2226448u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965312u32, 2226452u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2226452u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f914));
}
#[inline(always)]
pub fn block_0x0021f914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2226456u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2226460u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2226464u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2226476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f92c));
    } else {
        emu.pc = 2226468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f924));
    }
}
#[inline(always)]
pub fn block_0x0021f924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2226472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2226452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f914));
    } else {
        emu.pc = 2226476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f92c));
    }
}
#[inline(always)]
pub fn block_0x0021f92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2226480u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2226484u32;
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
pub fn block_0x0021f934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2226488u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2226492u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2226496u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2226500u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2226504u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2226508u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2226512u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2226516u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2226520u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2226524u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2226528u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2226532u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2226536u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2226540u32)?;
    emu.adi_no_count(26usize, 12usize, 0u32, 2226544u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2226548u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2226552u32)?;
    emu.orr_no_count(12usize, 14usize, 16usize, 2226556u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2229668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229668u32));
    } else {
        emu.pc = 2226560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f980));
    }
}
#[inline(always)]
pub fn block_0x0021f980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2226564u32);
    emu.lw_no_count(17usize, 11usize, 8u32, 2226568u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2226572u32)?;
    emu.orr_no_count(10usize, 17usize, 6usize, 2226576u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229696u32));
    } else {
        emu.pc = 2226580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f994));
    }
}
#[inline(always)]
pub fn block_0x0021f994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 16u32, 2226584u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2226588u32)?;
    emu.orr_no_count(12usize, 10usize, 5usize, 2226592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2229724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229724u32));
    } else {
        emu.pc = 2226596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9a4));
    }
}
#[inline(always)]
pub fn block_0x0021f9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 10usize, 2226600u32);
    emu.sltru_no_count(7usize, 15usize, 14usize, 2226604u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2226608u32);
    emu.adr_no_count(12usize, 5usize, 7usize, 2226612u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2226620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9bc));
    } else {
        emu.pc = 2226616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9b8));
    }
}
#[inline(always)]
pub fn block_0x0021f9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 16usize, 2226620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2226620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f9bc));
}
#[inline(always)]
pub fn block_0x0021f9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2229752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229752u32));
    } else {
        emu.pc = 2226624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9c0));
    }
}
#[inline(always)]
pub fn block_0x0021f9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2226636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9cc));
    } else {
        emu.pc = 2226628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9c4));
    }
}
#[inline(always)]
pub fn block_0x0021f9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 16usize, 6usize, 2226632u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2226636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2226640u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f9d0));
}
#[inline(always)]
pub fn block_0x0021f9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 17usize, 2226640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2226640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021f9d0));
}
#[inline(always)]
pub fn block_0x0021f9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2229780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229780u32));
    } else {
        emu.pc = 2226644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9d4));
    }
}
#[inline(always)]
pub fn block_0x0021f9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 16u32, 2226648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2229808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229808u32));
    } else {
        emu.pc = 2226652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9dc));
    }
}
#[inline(always)]
pub fn block_0x0021f9dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 10usize, 2226656u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2226660u32);
    emu.sri_no_count(12usize, 10usize, 29u32, 2226664u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2229836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229836u32));
    } else {
        emu.pc = 2226668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021f9ec));
    }
}
#[inline]
pub fn block_0x0021f9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 15usize, 1u32, 2226672u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2226676u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2226680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2226684u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2226688u32;
    emu.update_insn_clock();
    emu.adi_no_count(30usize, 12usize, 1365u32, 2226692u32);
    emu.adi_no_count(29usize, 7usize, 819u32, 2226696u32);
    emu.adi_no_count(28usize, 28usize, 4294967055u32, 2226700u32);
    emu.adi_no_count(7usize, 31usize, 257u32, 2226704u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2226836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fa94));
    } else {
        emu.pc = 2226708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fa14));
    }
}
#[inline(never)]
pub fn block_0x0021fa14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(12usize, 15usize, 5usize, 2226712u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2226716u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226720u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2226724u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226728u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2226732u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226736u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2226740u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226744u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2226748u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2226752u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2226756u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2226760u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2226764u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2226768u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2226772u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2226776u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2226780u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2226784u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2226788u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2226792u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2226796u32);
    emu.adi_no_count(7usize, 12usize, 32u32, 2226800u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2226804u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2226808u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2226812u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2226816u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2226948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb04));
    } else {
        emu.pc = 2226820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fa84));
    }
}
#[inline(always)]
pub fn block_0x0021fa84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 10usize, 7usize, 2226824u32);
    emu.srr_no_count(11usize, 5usize, 8usize, 2226828u32);
    emu.orr_no_count(11usize, 10usize, 11usize, 2226832u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2226836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2226952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fb08));
}
#[inline(never)]
pub fn block_0x0021fa94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 10usize, 1u32, 2226840u32);
    emu.orr_no_count(12usize, 10usize, 12usize, 2226844u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2226848u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226852u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2226856u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226860u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2226864u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226868u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2226872u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2226876u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2226880u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2226884u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2226888u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2226892u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2226896u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2226900u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2226904u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2226908u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2226912u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2226916u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2226920u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2226924u32);
    emu.sri_no_count(7usize, 12usize, 24u32, 2226928u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2226932u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2226936u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2226940u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2226944u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2226820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fa84));
    } else {
        emu.pc = 2226948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb04));
    }
}
#[inline(always)]
pub fn block_0x0021fb04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(11usize, 15usize, 31usize, 2226952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2226952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fb08));
}
#[inline]
pub fn block_0x0021fb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 9usize, 1055u32, 2226956u32);
    emu.sltru_no_count(10usize, 14usize, 17usize, 2226960u32);
    emu.sbr_no_count(12usize, 16usize, 6usize, 2226964u32);
    emu.sbr_no_count(28usize, 14usize, 17usize, 2226968u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2226972u32);
    emu.sw_no_count(28usize, 2usize, 24u32, 2226976u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2226980u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2226984u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2227024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb50));
    } else {
        emu.pc = 2226988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb2c));
    }
}
#[inline(always)]
pub fn block_0x0021fb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(17usize, 28usize, 31usize, 2226992u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2226996u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2227000u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2227052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb6c));
    } else {
        emu.pc = 2227004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb3c));
    }
}
#[inline(always)]
pub fn block_0x0021fb3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 6usize, 7usize, 2227008u32);
    emu.sli_no_count(29usize, 17usize, 1u32, 2227012u32);
    emu.slr_no_count(29usize, 29usize, 8usize, 2227016u32);
    emu.orr_no_count(29usize, 12usize, 29usize, 2227020u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2227024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227056u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fb70));
}
#[inline(always)]
pub fn block_0x0021fb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 10usize, 7usize, 2227028u32);
    emu.sri_no_count(17usize, 28usize, 1u32, 2227032u32);
    emu.srr_no_count(17usize, 17usize, 8usize, 2227036u32);
    emu.orr_no_count(17usize, 12usize, 17usize, 2227040u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2227044u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2227048u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2227004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb3c));
    } else {
        emu.pc = 2227052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb6c));
    }
}
#[inline(always)]
pub fn block_0x0021fb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(29usize, 17usize, 31usize, 2227056u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fb70));
}
#[inline(always)]
pub fn block_0x0021fb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 17usize, 7usize, 2227060u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2227064u32);
    emu.anr_no_count(12usize, 5usize, 12usize, 2227068u32);
    emu.xrr_no_count(10usize, 12usize, 10usize, 2227072u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2227076u32);
    emu.sw_no_count(29usize, 2usize, 40u32, 2227080u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2227084u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229632u32));
    } else {
        emu.pc = 2227088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fb90));
    }
}
#[inline(always)]
pub fn block_0x0021fb90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 24u32, 2227092u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2227096u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2227100u32)?;
    emu.slr_no_count(10usize, 14usize, 31usize, 2227104u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2227144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fbc8));
    } else {
        emu.pc = 2227108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fba4));
    }
}
#[inline(always)]
pub fn block_0x0021fba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 10usize, 0u32, 2227112u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2227116u32);
    emu.srr_no_count(10usize, 10usize, 31usize, 2227120u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2227172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fbe4));
    } else {
        emu.pc = 2227124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fbb4));
    }
}
#[inline(always)]
pub fn block_0x0021fbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 28usize, 1u32, 2227128u32);
    emu.slr_no_count(12usize, 12usize, 8usize, 2227132u32);
    emu.srr_no_count(31usize, 29usize, 31usize, 2227136u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2227140u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2227144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fbe8));
}
#[inline(always)]
pub fn block_0x0021fbc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 14usize, 1u32, 2227148u32);
    emu.srr_no_count(12usize, 12usize, 8usize, 2227152u32);
    emu.slr_no_count(28usize, 16usize, 31usize, 2227156u32);
    emu.orr_no_count(28usize, 28usize, 12usize, 2227160u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2227164u32);
    emu.srr_no_count(10usize, 28usize, 31usize, 2227168u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2227124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fbb4));
    } else {
        emu.pc = 2227172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fbe4));
    }
}
#[inline(always)]
pub fn block_0x0021fbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 10usize, 0u32, 2227176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fbe8));
}
#[inline(always)]
pub fn block_0x0021fbe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 5usize, 10usize, 2227180u32);
    emu.xrr_no_count(12usize, 31usize, 14usize, 2227184u32);
    emu.xrr_no_count(14usize, 10usize, 16usize, 2227188u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2227192u32);
    emu.sw_no_count(31usize, 2usize, 40u32, 2227196u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2227200u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2229632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229632u32));
    } else {
        emu.pc = 2227204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fc04));
    }
}
#[inline]
pub fn block_0x0021fc04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 16u32, 2227208u32)?;
    emu.sbr_no_count(10usize, 30usize, 7usize, 2227212u32);
    emu.adi_no_count(12usize, 0usize, 4294967200u32, 2227216u32);
    emu.adi_no_count(16usize, 0usize, 80u32, 2227220u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2227224u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 10usize, 2227228u32);
    emu.adi_no_count(14usize, 14usize, 4294965651u32, 2227232u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2227236u32);
    emu.sai_no_count(12usize, 12usize, 1040u32, 2227240u32);
    emu.adi_no_count(12usize, 12usize, 1087u32, 2227244u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2227248u32);
    emu.mulh_no_count(12usize, 12usize, 14usize, 2227252u32);
    emu.sri_no_count(14usize, 12usize, 31u32, 2227256u32);
    emu.sai_no_count(12usize, 12usize, 1034u32, 2227260u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2227264u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2229912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229912u32));
    } else {
        emu.pc = 2227268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fc44));
    }
}
#[inline(never)]
pub fn block_0x0021fc44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 15usize, 7usize, 2227272u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2227276u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2227280u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966248u32, 2227284u32);
    emu.sbr_no_count(15usize, 0usize, 10usize, 2227288u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2227292u32);
    emu.lw_no_count(14usize, 16usize, 0u32, 2227296u32)?;
    emu.lw_no_count(7usize, 16usize, 4u32, 2227300u32)?;
    emu.anr_no_count(10usize, 5usize, 12usize, 2227304u32);
    emu.lh_no_count(12usize, 16usize, 8u32, 2227308u32)?;
    emu.mulhu_no_count(30usize, 14usize, 10usize, 2227312u32);
    emu.mul_no_count(31usize, 7usize, 10usize, 2227316u32);
    emu.mulhu_no_count(8usize, 7usize, 10usize, 2227320u32);
    emu.mul_no_count(9usize, 14usize, 11usize, 2227324u32);
    emu.mulhu_no_count(18usize, 14usize, 11usize, 2227328u32);
    emu.mul_no_count(10usize, 7usize, 11usize, 2227332u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2227336u32);
    emu.sbr_no_count(15usize, 15usize, 12usize, 2227340u32);
    emu.mulhu_no_count(12usize, 14usize, 6usize, 2227344u32);
    emu.mul_no_count(19usize, 7usize, 6usize, 2227348u32);
    emu.mulhu_no_count(6usize, 7usize, 6usize, 2227352u32);
    emu.mul_no_count(20usize, 14usize, 17usize, 2227356u32);
    emu.mulhu_no_count(21usize, 14usize, 17usize, 2227360u32);
    emu.mul_no_count(5usize, 7usize, 17usize, 2227364u32);
    emu.mulhu_no_count(17usize, 7usize, 17usize, 2227368u32);
    emu.mulhu_no_count(22usize, 14usize, 29usize, 2227372u32);
    emu.mul_no_count(23usize, 7usize, 29usize, 2227376u32);
    emu.mulhu_no_count(29usize, 7usize, 29usize, 2227380u32);
    emu.mul_no_count(24usize, 14usize, 28usize, 2227384u32);
    emu.mulhu_no_count(14usize, 14usize, 28usize, 2227388u32);
    emu.mul_no_count(25usize, 7usize, 28usize, 2227392u32);
    emu.mulhu_no_count(28usize, 7usize, 28usize, 2227396u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2227400u32);
    emu.sltru_no_count(7usize, 30usize, 31usize, 2227404u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2227408u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2227412u32);
    emu.sltru_no_count(31usize, 12usize, 19usize, 2227416u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2227420u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2227424u32);
    emu.sltru_no_count(6usize, 22usize, 23usize, 2227428u32);
    emu.adr_no_count(19usize, 29usize, 6usize, 2227432u32);
    emu.adr_no_count(30usize, 9usize, 30usize, 2227436u32);
    emu.sltru_no_count(6usize, 30usize, 9usize, 2227440u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2227444u32);
    emu.adr_no_count(6usize, 20usize, 12usize, 2227448u32);
    emu.sltru_no_count(12usize, 6usize, 20usize, 2227452u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2227456u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2227460u32);
    emu.sltru_no_count(29usize, 22usize, 24usize, 2227464u32);
    emu.adr_no_count(14usize, 14usize, 29usize, 2227468u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2227472u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2227476u32);
    emu.adr_no_count(9usize, 11usize, 7usize, 2227480u32);
    emu.ani_no_count(7usize, 15usize, 63u32, 2227484u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2227488u32);
    emu.sltru_no_count(8usize, 12usize, 31usize, 2227492u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2227496u32);
    emu.adi_no_count(29usize, 7usize, 4294967264u32, 2227500u32);
    emu.sri_no_count(17usize, 30usize, 31u32, 2227504u32);
    emu.sri_no_count(30usize, 22usize, 31u32, 2227508u32);
    emu.adr_no_count(11usize, 19usize, 14usize, 2227512u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2227516u32);
    emu.adr_no_count(14usize, 5usize, 12usize, 2227520u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2227524u32);
    emu.adr_no_count(31usize, 25usize, 11usize, 2227528u32);
    emu.sltru_no_count(19usize, 18usize, 10usize, 2227532u32);
    emu.adr_no_count(11usize, 17usize, 18usize, 2227536u32);
    emu.sltru_no_count(10usize, 14usize, 5usize, 2227540u32);
    emu.sltru_no_count(24usize, 31usize, 25usize, 2227544u32);
    emu.adr_no_count(5usize, 28usize, 12usize, 2227548u32);
    emu.adr_no_count(18usize, 30usize, 31usize, 2227552u32);
    emu.adr_no_count(9usize, 9usize, 19usize, 2227556u32);
    emu.sltru_no_count(17usize, 11usize, 17usize, 2227560u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2227564u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2227568u32);
    emu.sltiu_no_count(28usize, 12usize, 1u32, 2227572u32);
    emu.adr_no_count(21usize, 17usize, 28usize, 2227576u32);
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2227580u32);
    emu.add_memory_rw_events(78usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2227592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fd88));
    } else {
        emu.pc = 2227584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fd80));
    }
}
#[inline(always)]
pub fn block_0x0021fd80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(22usize, 21usize, 7usize, 2227588u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fd98));
}
#[inline(always)]
pub fn block_0x0021fd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 21usize, 1u32, 2227596u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2227600u32);
    emu.srr_no_count(9usize, 12usize, 15usize, 2227604u32);
    emu.orr_no_count(22usize, 9usize, 28usize, 2227608u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2227608u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fd98));
}
#[inline]
pub fn block_0x0021fd98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 5usize, 24usize, 2227612u32);
    emu.sw_no_count(18usize, 2usize, 12u32, 2227616u32)?;
    emu.sltru_no_count(18usize, 18usize, 30usize, 2227620u32);
    emu.lhu_no_count(9usize, 16usize, 10u32, 2227624u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2227628u32);
    emu.sai_no_count(5usize, 6usize, 1055u32, 2227632u32);
    emu.slti_no_count(10usize, 29usize, 0u32, 2227636u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2227640u32);
    emu.sri_no_count(6usize, 22usize, 4u32, 2227644u32);
    emu.adi_no_count(28usize, 0usize, 625u32, 2227648u32);
    emu.slr_no_count(30usize, 16usize, 7usize, 2227652u32);
    emu.slr_no_count(16usize, 16usize, 15usize, 2227656u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2227660u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2227664u32);
    emu.anr_no_count(15usize, 15usize, 30usize, 2227668u32);
    emu.anr_no_count(16usize, 10usize, 16usize, 2227672u32);
    emu.sltiu_no_count(10usize, 16usize, 1u32, 2227676u32);
    emu.sbr_no_count(19usize, 15usize, 10usize, 2227680u32);
    emu.adi_no_count(20usize, 16usize, 4294967295u32, 2227684u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2227688u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2227720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe08));
    } else {
        emu.pc = 2227692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fdec));
    }
}
#[inline(always)]
pub fn block_0x0021fdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 100u32, 2227696u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2227764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe34));
    } else {
        emu.pc = 2227700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fdf4));
    }
}
#[inline(always)]
pub fn block_0x0021fdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 9u32, 2227704u32);
    emu.sltiu_no_count(10usize, 22usize, 10u32, 2227708u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2227832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe78));
    } else {
        emu.pc = 2227712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe00));
    }
}
#[inline(always)]
pub fn block_0x0021fe00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1u32, 2227716u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe7c));
}
#[inline(always)]
pub fn block_0x0021fe08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2227724u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2227728u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2227788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe4c));
    } else {
        emu.pc = 2227732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe14));
    }
}
#[inline(always)]
pub fn block_0x0021fe14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2227736u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1696u32, 2227740u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2227744u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2227756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe2c));
    } else {
        emu.pc = 2227748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe24));
    }
}
#[inline(always)]
pub fn block_0x0021fe24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2227752u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 1808u32, 2227756u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2227756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe2c));
}
#[inline(always)]
pub fn block_0x0021fe2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 5u32, 2227760u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227764u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe9c));
}
#[inline(always)]
pub fn block_0x0021fe34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 1000u32, 2227768u32);
    emu.sltiu_no_count(10usize, 22usize, 1000u32, 2227772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2227780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe44));
    } else {
        emu.pc = 2227776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe40));
    }
}
#[inline(always)]
pub fn block_0x0021fe40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1000u32, 2227780u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe44));
}
#[inline(always)]
pub fn block_0x0021fe44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 3u32, 2227784u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe9c));
}
#[inline(always)]
pub fn block_0x0021fe4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2227792u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 256u32, 2227796u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2227844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe84));
    } else {
        emu.pc = 2227800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe58));
    }
}
#[inline(always)]
pub fn block_0x0021fe58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2227804u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1664u32, 2227808u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2227812u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2227824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe70));
    } else {
        emu.pc = 2227816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe68));
    }
}
#[inline(always)]
pub fn block_0x0021fe68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2227820u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 576u32, 2227824u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2227824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe70));
}
#[inline(always)]
pub fn block_0x0021fe70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 7u32, 2227828u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe9c));
}
#[inline(always)]
pub fn block_0x0021fe78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 10u32, 2227836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe7c));
}
#[inline(always)]
pub fn block_0x0021fe7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 1u32, 2227840u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2227844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2227868u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe9c));
}
#[inline(always)]
pub fn block_0x0021fe84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2227848u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 10usize, 4294965760u32, 2227852u32);
    emu.sltru_no_count(10usize, 22usize, 6usize, 2227856u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2227864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe98));
    } else {
        emu.pc = 2227860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fe94));
    }
}
#[inline(always)]
pub fn block_0x0021fe94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 6usize, 0u32, 2227864u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe98));
}
#[inline(always)]
pub fn block_0x0021fe98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 9u32, 2227868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2227868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fe9c));
}
#[inline(never)]
pub fn block_0x0021fe9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 24usize, 18usize, 2227872u32);
    emu.sw_no_count(18usize, 2usize, 4u32, 2227876u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2227880u32)?;
    emu.anr_no_count(18usize, 19usize, 21usize, 2227884u32);
    emu.anr_no_count(21usize, 20usize, 12usize, 2227888u32);
    emu.sbr_no_count(6usize, 1usize, 9usize, 2227892u32);
    emu.sltru_no_count(28usize, 5usize, 14usize, 2227896u32);
    emu.sbr_no_count(30usize, 5usize, 8usize, 2227900u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2227904u32);
    emu.adi_no_count(24usize, 0usize, 4294967295u32, 2227908u32);
    emu.sai_no_count(14usize, 29usize, 1055u32, 2227912u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2227916u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 0usize, 10u32, 2227920u32);
    emu.adi_no_count(6usize, 6usize, 1u32, 2227924u32);
    emu.sw_no_count(6usize, 2usize, 0u32, 2227928u32)?;
    emu.sbr_no_count(28usize, 30usize, 28usize, 2227932u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2227936u32);
    emu.adi_no_count(6usize, 8usize, 4294966477u32, 2227940u32);
    emu.sltru_no_count(5usize, 11usize, 5usize, 2227944u32);
    emu.adi_no_count(8usize, 11usize, 2u32, 2227948u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2227952u32);
    emu.sltru_no_count(9usize, 8usize, 11usize, 2227956u32);
    emu.anr_no_count(17usize, 8usize, 20usize, 2227960u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2227964u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2227968u32);
    emu.anr_no_count(5usize, 9usize, 19usize, 2227972u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2227976u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2227980u32)?;
    emu.add_memory_rw_events(28usize);
    emu.pc = 2227980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ff0c));
}
#[inline(always)]
pub fn block_0x0021ff0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 13usize, 11usize, 2227984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2229864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229864u32));
    } else {
        emu.pc = 2227988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff14));
    }
}
#[inline(always)]
pub fn block_0x0021ff14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 26usize, 0u32, 2227992u32);
    emu.divu_no_count(28usize, 22usize, 26usize, 2227996u32);
    emu.mul_no_count(26usize, 28usize, 26usize, 2228000u32);
    emu.adi_no_count(30usize, 28usize, 48u32, 2228004u32);
    emu.sbr_no_count(22usize, 22usize, 26usize, 2228008u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2228012u32);
    emu.slr_no_count(26usize, 22usize, 7usize, 2228016u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2228056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff58));
    } else {
        emu.pc = 2228020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff34));
    }
}
#[inline(always)]
pub fn block_0x0021ff34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 14usize, 26usize, 2228024u32);
    emu.adr_no_count(27usize, 26usize, 18usize, 2228028u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2228032u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2228036u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2228040u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2228088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff78));
    } else {
        emu.pc = 2228044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff4c));
    }
}
#[inline(always)]
pub fn block_0x0021ff4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 26usize, 8usize, 2228048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2228096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff80));
    } else {
        emu.pc = 2228052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff54));
    }
}
#[inline(always)]
pub fn block_0x0021ff54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ffac));
}
#[inline(always)]
pub fn block_0x0021ff58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(28usize, 22usize, 1u32, 2228060u32);
    emu.srr_no_count(27usize, 28usize, 31usize, 2228064u32);
    emu.anr_no_count(28usize, 14usize, 26usize, 2228068u32);
    emu.adr_no_count(27usize, 27usize, 18usize, 2228072u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2228076u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2228080u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2228084u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2228044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff4c));
    } else {
        emu.pc = 2228088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff78));
    }
}
#[inline(always)]
pub fn block_0x0021ff78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 27usize, 9usize, 2228092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2228140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ffac));
    } else {
        emu.pc = 2228096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff80));
    }
}
#[inline(always)]
pub fn block_0x0021ff80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 1usize, 11usize, 2228100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2228156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ffbc));
    } else {
        emu.pc = 2228104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff88));
    }
}
#[inline(always)]
pub fn block_0x0021ff88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(28usize, 25usize, 6usize, 2228108u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2228112u32);
    emu.sri_no_count(26usize, 28usize, 3u32, 2228116u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2228120u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2227980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff0c));
    } else {
        emu.pc = 2228124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ff9c));
    }
}
#[inline(always)]
pub fn block_0x0021ff9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2228128u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 528u32, 2228132u32);
    emu.apc_no_count(1usize, 2228132u32, 4096u32, 2228136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2228140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ffac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 25usize, 7usize, 2228144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2228416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002200c0));
    } else {
        emu.pc = 2228148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ffb4));
    }
}
#[inline(always)]
pub fn block_0x0021ffb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2228152u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2228156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002200c8));
}
#[inline(always)]
pub fn block_0x0021ffbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2228160u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2228164u32);
    emu.adi_no_count(6usize, 0usize, 1u32, 2228168u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2228172u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2228176u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2228180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021fff0));
}
#[inline(always)]
pub fn block_0x0021ffd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 18usize, 5usize, 2228184u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2228188u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2228192u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2228196u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2228200u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2228204u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a != b {
        emu.pc = 2228332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022006c));
    } else {
        emu.pc = 2228208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fff0));
    }
}
#[inline(always)]
pub fn block_0x0021fff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2229888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2229888u32));
    } else {
        emu.pc = 2228212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fff4));
    }
}
#[inline(always)]
pub fn block_0x0021fff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 14usize, 0u32, 2228216u32);
    emu.adi_no_count(8usize, 6usize, 0u32, 2228220u32);
    emu.mulhu_no_count(14usize, 21usize, 10usize, 2228224u32);
    emu.mul_no_count(6usize, 18usize, 10usize, 2228228u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2228232u32);
    emu.mul_no_count(6usize, 21usize, 10usize, 2228236u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2228248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220018));
    } else {
        emu.pc = 2228240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220010));
    }
}
#[inline(always)]
pub fn block_0x00220010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(30usize, 14usize, 7usize, 2228244u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2228248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220028));
}
#[inline(always)]
pub fn block_0x00220018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 14usize, 1u32, 2228252u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2228256u32);
    emu.srr_no_count(30usize, 6usize, 7usize, 2228260u32);
    emu.orr_no_count(30usize, 30usize, 28usize, 2228264u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2228264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220028));
}
#[inline]
pub fn block_0x00220028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(18usize, 14usize, 19usize, 2228268u32);
    emu.anr_no_count(21usize, 6usize, 20usize, 2228272u32);
    emu.mulhu_no_count(14usize, 17usize, 10usize, 2228276u32);
    emu.mul_no_count(5usize, 5usize, 10usize, 2228280u32);
    emu.mul_no_count(17usize, 17usize, 10usize, 2228284u32);
    emu.adi_no_count(22usize, 30usize, 48u32, 2228288u32);
    emu.adr_no_count(5usize, 14usize, 5usize, 2228292u32);
    emu.adr_no_count(14usize, 23usize, 11usize, 2228296u32);
    emu.sb_no_count(22usize, 14usize, 0u32, 2228300u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2228180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ffd4));
    } else {
        emu.pc = 2228304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220050));
    }
}
#[inline(always)]
pub fn block_0x00220050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 21usize, 17usize, 2228308u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2228312u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2228316u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2228320u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2228324u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2228328u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2228208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021fff0));
    } else {
        emu.pc = 2228332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022006c));
    }
}
#[inline]
pub fn block_0x0022006c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 12u32, 2228336u32)?;
    emu.sltru_no_count(10usize, 12usize, 7usize, 2228340u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2228344u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2228348u32)?;
    emu.sbr_no_count(13usize, 13usize, 28usize, 2228352u32);
    emu.sbr_no_count(12usize, 12usize, 7usize, 2228356u32);
    emu.sbr_no_count(13usize, 13usize, 10usize, 2228360u32);
    emu.mulhu_no_count(10usize, 6usize, 12usize, 2228364u32);
    emu.mul_no_count(31usize, 14usize, 12usize, 2228368u32);
    emu.mul_no_count(7usize, 6usize, 12usize, 2228372u32);
    emu.mul_no_count(12usize, 6usize, 13usize, 2228376u32);
    emu.adr_no_count(13usize, 7usize, 6usize, 2228380u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2228384u32);
    emu.sltru_no_count(12usize, 7usize, 6usize, 2228388u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2228392u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2228396u32);
    emu.sbr_no_count(29usize, 31usize, 12usize, 2228400u32);
    emu.sbr_no_count(30usize, 7usize, 6usize, 2228404u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2228584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2228584u32));
    } else {
        emu.pc = 2228408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002200b8));
    }
}
#[inline(always)]
pub fn block_0x002200b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 29usize, 2228412u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2228416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2228588u32));
}
#[inline(always)]
pub fn block_0x002200c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 1u32, 2228420u32);
    emu.srr_no_count(15usize, 13usize, 31usize, 2228424u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2228424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002200c8));
}
#[inline]
pub fn block_0x002200c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2228428u32)?;
    emu.lw_no_count(16usize, 2usize, 8u32, 2228432u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2228436u32)?;
    emu.sltru_no_count(13usize, 12usize, 17usize, 2228440u32);
    emu.lw_no_count(5usize, 2usize, 4u32, 2228444u32)?;
    emu.sbr_no_count(16usize, 16usize, 5usize, 2228448u32);
    emu.sbr_no_count(28usize, 12usize, 17usize, 2228452u32);
    emu.sbr_no_count(17usize, 16usize, 13usize, 2228456u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2228460u32);
    emu.sltiu_no_count(12usize, 28usize, 1u32, 2228464u32);
    emu.sbr_no_count(7usize, 17usize, 12usize, 2228468u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2228472u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2228484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2228484u32));
    } else {
        emu.pc = 2228476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002200fc));
    }
}
#[inline(always)]
pub fn block_0x002200fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 27usize, 7usize, 2228480u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2228484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2228488u32));
}
