pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2194376u32;
pub const PC_MAX: u32 = 2196992u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00217bc8,
        block_0x00217bd4,
        block_0x00217bf4,
        block_0x00217c44,
        block_0x00217c50,
        block_0x00217c60,
        block_0x00217c68,
        block_0x00217c70,
        block_0x00217c78,
        block_0x00217c80,
        block_0x00217c90,
        block_0x00217c98,
        block_0x00217ca0,
        block_0x00217ca8,
        block_0x00217cb0,
        block_0x00217cbc,
        block_0x00217cc4,
        block_0x00217cd0,
        block_0x00217cdc,
        block_0x00217ce0,
        block_0x00217cf8,
        block_0x00217cfc,
        block_0x00217d18,
        block_0x00217d28,
        block_0x00217d2c,
        block_0x00217d38,
        block_0x00217d60,
        block_0x00217d6c,
        block_0x00217d70,
        block_0x00217d7c,
        block_0x00217da0,
        block_0x00217dac,
        block_0x00217de0,
        block_0x00217de8,
        block_0x00217e00,
        block_0x00217e08,
        block_0x00217e10,
        block_0x00217e28,
        block_0x00217e30,
        block_0x00217e38,
        block_0x00217e50,
        block_0x00217e58,
        block_0x00217e70,
        block_0x00217e78,
        block_0x00217e88,
        block_0x00217e98,
        block_0x00217eac,
        block_0x00217ebc,
        block_0x00217ed4,
        block_0x00217eec,
        block_0x00217f48,
        block_0x00217f54,
        block_0x00217fa8,
        block_0x00217fb0,
        block_0x00217fb4,
        block_0x00217fbc,
        block_0x00217fd0,
        block_0x00217fd8,
        block_0x00217fdc,
        block_0x00218000,
        block_0x00218008,
        block_0x00218030,
        block_0x00218040,
        block_0x00218048,
        block_0x00218050,
        block_0x00218094,
        block_0x00218098,
        block_0x002180a0,
        block_0x002180a4,
        block_0x002180b8,
        block_0x002180bc,
        block_0x002180f8,
        block_0x00218130,
        block_0x0021815c,
        block_0x00218190,
        block_0x0021819c,
        block_0x002181b8,
        block_0x002181d4,
        block_0x00218208,
        block_0x00218214,
        block_0x00218230,
        block_0x0021824c,
        block_0x00218268,
        block_0x00218284,
        block_0x00218298,
        block_0x002182a4,
        block_0x002182c4,
        block_0x002182d8,
        block_0x002182fc,
        block_0x00218354,
        block_0x00218360,
        block_0x00218384,
        block_0x002183a8,
        block_0x002183cc,
        block_0x00218424,
        block_0x00218430,
        block_0x00218454,
        block_0x00218478,
        block_0x00218490,
        block_0x002184b4,
        block_0x002184f4,
        block_0x00218518,
        block_0x0021852c,
        block_0x00218554,
        block_0x00218578,
        block_0x00218584,
        block_0x002185a4,
        block_0x00218600,
    ];
    const IDX: [u16; 655usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 6u16,
        0u16, 7u16, 0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16,
        12u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16, 16u16, 0u16, 17u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 25u16,
        0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        46u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16,
        57u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16,
        68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 76u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
    ];
    if pc < 2194376u32 || pc > 2196992u32 {
        return None;
    }
    let word_offset = ((pc - 2194376u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00217bc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2194380u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2194384u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194388u32;
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
pub fn block_0x00217bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2194392u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2194396u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2194400u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2194404u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2194408u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2194412u32);
    emu.apc_no_count(6usize, 2194412u32, 45056u32, 2194416u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194420u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2194424u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2194428u32)?;
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2194432u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967088u32, 2194436u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2194440u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966716u32, 2194444u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2194448u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2194452u32)?;
    emu.adi_no_count(15usize, 2usize, 36u32, 2194456u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2194460u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2194464u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2194468u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2194472u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2194476u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2194480u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2194484u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2194488u32)?;
    emu.adi_no_count(12usize, 2usize, 12u32, 2194492u32);
    emu.apc_no_count(1usize, 2194492u32, 40960u32, 2194496u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2194504u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2194508u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194512u32;
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
pub fn block_0x00217c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2194516u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2194520u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2194524u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c70));
    } else {
        emu.pc = 2194528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c60));
    }
}
#[inline(always)]
pub fn block_0x00217c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2194532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c78));
    } else {
        emu.pc = 2194536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c68));
    }
}
#[inline(always)]
pub fn block_0x00217c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194536u32, 20480u32, 2194540u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194544u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194544u32, 20480u32, 2194548u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194552u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194552u32, 20480u32, 2194556u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194560u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00217c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2194564u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2194568u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2194572u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ca0));
    } else {
        emu.pc = 2194576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c90));
    }
}
#[inline(always)]
pub fn block_0x00217c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2194580u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ca8));
    } else {
        emu.pc = 2194584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217c98));
    }
}
#[inline(always)]
pub fn block_0x00217c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194584u32, 20480u32, 2194588u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194592u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194592u32, 20480u32, 2194596u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194600u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2194600u32, 20480u32, 2194604u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194608u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2194612u32)?;
    emu.apc_no_count(6usize, 2194612u32, 0u32, 2194616u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2194620u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2194624u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2194712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d18));
    } else {
        emu.pc = 2194628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cc4));
    }
}
#[inline(always)]
pub fn block_0x00217cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2194632u32)?;
    emu.lw_no_count(16usize, 11usize, 8u32, 2194636u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2194784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d60));
    } else {
        emu.pc = 2194640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cd0));
    }
}
#[inline(always)]
pub fn block_0x00217cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 16usize, 0u32, 2194644u32);
    emu.adi_no_count(14usize, 0usize, 39u32, 2194648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2195032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e58));
    } else {
        emu.pc = 2194652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cdc));
    }
}
#[inline(always)]
pub fn block_0x00217cdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2194848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217da0));
    } else {
        emu.pc = 2194656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ce0));
    }
}
#[inline(always)]
pub fn block_0x00217ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 16usize, 1u32, 2194660u32);
    emu.adr_no_count(14usize, 16usize, 12usize, 2194664u32);
    emu.lb_no_count(17usize, 14usize, 0u32, 2194668u32);
    emu.ani_no_count(15usize, 17usize, 127u32, 2194672u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2194676u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217de0));
    } else {
        emu.pc = 2194680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217cf8));
    }
}
#[inline(always)]
pub fn block_0x00217cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 1u32, 2194684u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2194684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cfc));
}
#[inline(always)]
pub fn block_0x00217cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 16usize, 12usize, 2194688u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2194692u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2194696u32)?;
    emu.sw_no_count(14usize, 10usize, 4u32, 2194700u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2194704u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2194708u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194712u32;
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
pub fn block_0x00217d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2194716u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2194720u32);
    emu.adi_no_count(14usize, 0usize, 40u32, 2194724u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2195032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e58));
    } else {
        emu.pc = 2194728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d28));
    }
}
#[inline(always)]
pub fn block_0x00217d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2195132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ebc));
    } else {
        emu.pc = 2194732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d2c));
    }
}
#[inline(always)]
pub fn block_0x00217d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 12usize, 1u32, 2194736u32);
    emu.adi_no_count(13usize, 0usize, 119u32, 2194740u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2194812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d7c));
    } else {
        emu.pc = 2194744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d38));
    }
}
#[inline]
pub fn block_0x00217d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2194748u32);
    emu.sw_no_count(14usize, 11usize, 0u32, 2194752u32)?;
    emu.sw_no_count(0usize, 11usize, 4u32, 2194756u32)?;
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194760u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1639u32, 2194764u32);
    emu.mulhu_no_count(11usize, 12usize, 11usize, 2194768u32);
    emu.sw_no_count(14usize, 10usize, 4u32, 2194772u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2194776u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2194780u32)?;
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194784u32;
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
pub fn block_0x00217d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 16usize, 0u32, 2194788u32);
    emu.adi_no_count(12usize, 0usize, 40u32, 2194792u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2195032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e58));
    } else {
        emu.pc = 2194796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d6c));
    }
}
#[inline(always)]
pub fn block_0x00217d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2195156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217ed4));
    } else {
        emu.pc = 2194800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d70));
    }
}
#[inline(always)]
pub fn block_0x00217d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 16usize, 1u32, 2194804u32);
    emu.adi_no_count(13usize, 0usize, 119u32, 2194808u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2194860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217dac));
    } else {
        emu.pc = 2194812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217d7c));
    }
}
#[inline]
pub fn block_0x00217d7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2194816u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1639u32, 2194820u32);
    emu.mulhu_no_count(11usize, 12usize, 11usize, 2194824u32);
    emu.sb_no_count(0usize, 10usize, 4u32, 2194828u32);
    emu.sb_no_count(12usize, 10usize, 5u32, 2194832u32);
    emu.sw_no_count(11usize, 10usize, 8u32, 2194836u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2194840u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2194844u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194848u32;
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
pub fn block_0x00217da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 4u32, 2194852u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2194856u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194860u32;
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
pub fn block_0x00217dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 1u32, 2194864u32);
    let a = 0u32.wrapping_add(107372544u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2194868u32;
    emu.update_insn_clock();
    emu.sw_no_count(14usize, 11usize, 0u32, 2194872u32)?;
    emu.sw_no_count(14usize, 11usize, 4u32, 2194876u32)?;
    emu.adi_no_count(11usize, 0usize, 40u32, 2194880u32);
    emu.adi_no_count(15usize, 15usize, 1639u32, 2194884u32);
    emu.mulhu_no_count(15usize, 12usize, 15usize, 2194888u32);
    emu.mul_no_count(11usize, 15usize, 11usize, 2194892u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2194896u32);
    emu.sw_no_count(14usize, 10usize, 4u32, 2194900u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2194904u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2194908u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2194912u32;
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
pub fn block_0x00217de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 1u32, 2194916u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2195096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e98));
    } else {
        emu.pc = 2194920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217de8));
    }
}
#[inline(always)]
pub fn block_0x00217de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2194924u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2194928u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2194932u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2194936u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2194940u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e08));
    } else {
        emu.pc = 2194944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e00));
    }
}
#[inline(always)]
pub fn block_0x00217e00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 2u32, 2194948u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cfc));
}
#[inline(always)]
pub fn block_0x00217e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 2u32, 2194956u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2195096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e98));
    } else {
        emu.pc = 2194960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e10));
    }
}
#[inline(always)]
pub fn block_0x00217e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2194964u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2194968u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2194972u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2194976u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2194980u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2194992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e30));
    } else {
        emu.pc = 2194984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e28));
    }
}
#[inline(always)]
pub fn block_0x00217e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 3u32, 2194988u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2194992u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cfc));
}
#[inline(always)]
pub fn block_0x00217e30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 3u32, 2194996u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2195096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e98));
    } else {
        emu.pc = 2195000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e38));
    }
}
#[inline(always)]
pub fn block_0x00217e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 16usize, 17usize, 2195004u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2195008u32);
    emu.sli_no_count(15usize, 15usize, 7u32, 2195012u32);
    emu.ani_no_count(5usize, 17usize, 127u32, 2195016u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2195020u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2195056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e70));
    } else {
        emu.pc = 2195024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e50));
    }
}
#[inline(always)]
pub fn block_0x00217e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 4u32, 2195028u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2195032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cfc));
}
#[inline(always)]
pub fn block_0x00217e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195036u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 840u32, 2195040u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2195044u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2195048u32);
    emu.apc_no_count(1usize, 2195048u32, 53248u32, 2195052u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 12usize, 4u32, 2195060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2195096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e98));
    } else {
        emu.pc = 2195064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e78));
    }
}
#[inline(always)]
pub fn block_0x00217e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 16usize, 17usize, 2195068u32);
    emu.lbu_no_count(13usize, 16usize, 0u32, 2195072u32);
    emu.adi_no_count(16usize, 0usize, 16u32, 2195076u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2195116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217eac));
    } else {
        emu.pc = 2195080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217e88));
    }
}
#[inline(always)]
pub fn block_0x00217e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 7u32, 2195084u32);
    emu.orr_no_count(15usize, 15usize, 13usize, 2195088u32);
    emu.adi_no_count(16usize, 0usize, 5u32, 2195092u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2195096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2194684u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217cfc));
}
#[inline(always)]
pub fn block_0x00217e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2195100u32);
    emu.sb_no_count(11usize, 10usize, 4u32, 2195104u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2195108u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2195112u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195116u32;
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
pub fn block_0x00217eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2195120u32);
    emu.sb_no_count(13usize, 10usize, 4u32, 2195124u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2195128u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195132u32;
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
pub fn block_0x00217ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195136u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 760u32, 2195140u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2195144u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2195148u32);
    emu.apc_no_count(1usize, 2195148u32, 24576u32, 2195152u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195156u32;
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
pub fn block_0x00217ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195160u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 776u32, 2195164u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2195168u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2195172u32);
    emu.apc_no_count(1usize, 2195172u32, 24576u32, 2195176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966088u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00217eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2195184u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2195188u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2195192u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2195196u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2195200u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966448u32, 2195204u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2195208u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 876u32, 2195212u32);
    emu.adi_no_count(14usize, 0usize, 2u32, 2195216u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2195220u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2195224u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2195228u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2195232u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2195236u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2195240u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2195244u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2195248u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2195252u32)?;
    emu.sw_no_count(16usize, 2usize, 24u32, 2195256u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2195260u32)?;
    emu.adi_no_count(12usize, 2usize, 12u32, 2195264u32);
    emu.apc_no_count(1usize, 2195264u32, 36864u32, 2195268u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2195276u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2195280u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195284u32;
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
pub fn block_0x00217f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2195288u32);
    emu.sw_no_count(1usize, 2usize, 108u32, 2195292u32)?;
    emu.sw_no_count(8usize, 2usize, 104u32, 2195296u32)?;
    emu.sw_no_count(9usize, 2usize, 100u32, 2195300u32)?;
    emu.sw_no_count(18usize, 2usize, 96u32, 2195304u32)?;
    emu.sw_no_count(19usize, 2usize, 92u32, 2195308u32)?;
    emu.sw_no_count(20usize, 2usize, 88u32, 2195312u32)?;
    emu.sw_no_count(21usize, 2usize, 84u32, 2195316u32)?;
    emu.sw_no_count(22usize, 2usize, 80u32, 2195320u32)?;
    emu.sw_no_count(23usize, 2usize, 76u32, 2195324u32)?;
    emu.sw_no_count(24usize, 2usize, 72u32, 2195328u32)?;
    emu.sw_no_count(25usize, 2usize, 68u32, 2195332u32)?;
    emu.sw_no_count(26usize, 2usize, 64u32, 2195336u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2195340u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2195344u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2195348u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2195352u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2195356u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2195360u32);
    emu.apc_no_count(1usize, 2195360u32, 0u32, 2195364u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195368u32;
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
pub fn block_0x00217fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2195372u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f8));
    } else {
        emu.pc = 2195376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217fb0));
    }
}
#[inline(always)]
pub fn block_0x00217fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2195380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2195380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00217fb4));
}
#[inline(always)]
pub fn block_0x00217fb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2195384u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2195420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217fdc));
    } else {
        emu.pc = 2195388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217fbc));
    }
}
#[inline(always)]
pub fn block_0x00217fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 20usize, 1u32, 2195392u32);
    emu.adi_no_count(10usize, 2usize, 32u32, 2195396u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2195400u32);
    emu.apc_no_count(1usize, 2195400u32, 0u32, 2195404u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00217fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2195412u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2195380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217fb4));
    } else {
        emu.pc = 2195416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00217fd8));
    }
}
#[inline(always)]
pub fn block_0x00217fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2195420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002180f8));
}
#[inline]
pub fn block_0x00217fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 9usize, 0u32, 2195424u32)?;
    emu.lw_no_count(9usize, 9usize, 4u32, 2195428u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2195432u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2195436u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2195440u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2195444u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2195448u32);
    emu.apc_no_count(1usize, 2195448u32, 0u32, 2195452u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2195460u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f8));
    } else {
        emu.pc = 2195464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218008));
    }
}
#[inline]
pub fn block_0x00218008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 2usize, 28u32, 2195468u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2195472u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294967288u32, 2195476u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2195480u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294966716u32, 2195484u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2195488u32);
    emu.adi_no_count(25usize, 2usize, 56u32, 2195492u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2195496u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 892u32, 2195500u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2195504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2195528u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218048));
}
#[inline(always)]
pub fn block_0x00218030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2195508u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2195512u32);
    emu.apc_no_count(1usize, 2195512u32, 0u32, 2195516u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2195524u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180f8));
    } else {
        emu.pc = 2195528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218048));
    }
}
#[inline(always)]
pub fn block_0x00218048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 36u32, 2195532u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2195644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180bc));
    } else {
        emu.pc = 2195536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218050));
    }
}
#[inline]
pub fn block_0x00218050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 40u32, 2195540u32)?;
    emu.lw_no_count(26usize, 2usize, 24u32, 2195544u32)?;
    emu.sw_no_count(21usize, 2usize, 56u32, 2195548u32)?;
    emu.sw_no_count(22usize, 2usize, 60u32, 2195552u32)?;
    emu.sw_no_count(0usize, 2usize, 48u32, 2195556u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2195560u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2195564u32)?;
    emu.sw_no_count(26usize, 2usize, 24u32, 2195568u32)?;
    emu.sw_no_count(23usize, 2usize, 32u32, 2195572u32)?;
    emu.sw_no_count(24usize, 2usize, 36u32, 2195576u32)?;
    emu.sw_no_count(25usize, 2usize, 40u32, 2195580u32)?;
    emu.sw_no_count(24usize, 2usize, 44u32, 2195584u32)?;
    emu.adi_no_count(12usize, 2usize, 32u32, 2195588u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195592u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2195596u32);
    emu.apc_no_count(1usize, 2195596u32, 36864u32, 2195600u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195604u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180bc));
    } else {
        emu.pc = 2195608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218098));
    }
}
#[inline(always)]
pub fn block_0x00218098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 26usize, 1u32, 2195612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2195504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218030));
    } else {
        emu.pc = 2195616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180a0));
    }
}
#[inline(always)]
pub fn block_0x002180a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2195504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218030));
    } else {
        emu.pc = 2195620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180a4));
    }
}
#[inline(always)]
pub fn block_0x002180a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 9usize, 12u32, 2195624u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2195628u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2195632u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2195636u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2195640u32;
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
pub fn block_0x002180b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2195504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218030));
    } else {
        emu.pc = 2195644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002180bc));
    }
}
#[inline]
pub fn block_0x002180bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2195648u32);
    emu.lw_no_count(1usize, 2usize, 108u32, 2195652u32)?;
    emu.lw_no_count(8usize, 2usize, 104u32, 2195656u32)?;
    emu.lw_no_count(9usize, 2usize, 100u32, 2195660u32)?;
    emu.lw_no_count(18usize, 2usize, 96u32, 2195664u32)?;
    emu.lw_no_count(19usize, 2usize, 92u32, 2195668u32)?;
    emu.lw_no_count(20usize, 2usize, 88u32, 2195672u32)?;
    emu.lw_no_count(21usize, 2usize, 84u32, 2195676u32)?;
    emu.lw_no_count(22usize, 2usize, 80u32, 2195680u32)?;
    emu.lw_no_count(23usize, 2usize, 76u32, 2195684u32)?;
    emu.lw_no_count(24usize, 2usize, 72u32, 2195688u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2195692u32)?;
    emu.lw_no_count(26usize, 2usize, 64u32, 2195696u32)?;
    emu.adi_no_count(2usize, 2usize, 112u32, 2195700u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195704u32;
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
pub fn block_0x002180f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2195708u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2195712u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2195716u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2195720u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2195724u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 808u32, 2195728u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2195732u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 792u32, 2195736u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2195740u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 824u32, 2195744u32);
    emu.adi_no_count(11usize, 0usize, 13u32, 2195748u32);
    emu.adi_no_count(12usize, 2usize, 56u32, 2195752u32);
    emu.apc_no_count(1usize, 2195752u32, 36864u32, 2195756u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2195764u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2195768u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2195772u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2195776u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2195780u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2195784u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 728u32, 2195788u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2195792u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2195796u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2195800u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2195804u32;
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
pub fn block_0x0021815c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2195808u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2195812u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195816u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 912u32, 2195820u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2195824u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 922u32, 2195828u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2195832u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 896u32, 2195836u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2195840u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2195844u32);
    emu.adi_no_count(15usize, 2usize, 4u32, 2195848u32);
    emu.apc_no_count(1usize, 2195848u32, 40960u32, 2195852u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2195860u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195864u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195868u32;
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
pub fn block_0x0021819c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195872u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 979u32, 2195876u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2195880u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2195884u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195888u32);
    emu.apc_no_count(6usize, 2195888u32, 40960u32, 2195892u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195896u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002181b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195900u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 934u32, 2195904u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2195908u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2195912u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195916u32);
    emu.apc_no_count(6usize, 2195916u32, 40960u32, 2195920u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2195924u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002181d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2195928u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2195932u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 960u32, 2195940u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2195944u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 973u32, 2195948u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2195952u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 944u32, 2195956u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2195960u32);
    emu.adi_no_count(14usize, 0usize, 6u32, 2195964u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2195968u32);
    emu.apc_no_count(1usize, 2195968u32, 40960u32, 2195972u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2195980u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2195984u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2195988u32;
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
pub fn block_0x00218214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2195992u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1003u32, 2195996u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2196000u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2196004u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2196008u32);
    emu.apc_no_count(6usize, 2196008u32, 40960u32, 2196012u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196016u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00218230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196020u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 925u32, 2196024u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2196028u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2196032u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2196036u32);
    emu.apc_no_count(6usize, 2196036u32, 40960u32, 2196040u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196044u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021824c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196048u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 984u32, 2196052u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2196056u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2196060u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2196064u32);
    emu.apc_no_count(6usize, 2196064u32, 40960u32, 2196068u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196072u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00218268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196076u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 990u32, 2196080u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2196084u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2196088u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2196092u32);
    emu.apc_no_count(6usize, 2196092u32, 40960u32, 2196096u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196100u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2196104u32);
    emu.sb_no_count(10usize, 2usize, 15u32, 2196108u32);
    emu.lbu_no_count(10usize, 2usize, 15u32, 2196112u32);
    emu.adi_no_count(2usize, 2usize, 16u32, 2196116u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196120u32;
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
pub fn block_0x00218298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2196124u32);
    emu.apc_no_count(6usize, 2196124u32, 0u32, 2196128u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196132u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002182a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2196136u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2196140u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2196144u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2196148u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196152u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2196156u32);
    emu.apc_no_count(6usize, 2196156u32, 40960u32, 2196160u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196164u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002182c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2196168u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2196172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1088u32, 2196176u32);
    emu.apc_no_count(6usize, 2196176u32, 36864u32, 2196180u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196184u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002182d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2196188u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2196192u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2196196u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2196200u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196204u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1016u32, 2196208u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2196212u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2196216u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2196220u32;
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
pub fn block_0x002182fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2196224u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2196228u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196232u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965392u32, 2196236u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196240u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1132u32, 2196244u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2196248u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2196252u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2196256u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2196260u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2196264u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2196268u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2196272u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2196276u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2196280u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2196284u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2196288u32)?;
    emu.sb_no_count(12usize, 2usize, 0u32, 2196292u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196296u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2196300u32);
    emu.apc_no_count(1usize, 2196300u32, 36864u32, 2196304u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196308u32;
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
pub fn block_0x00218354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2196312u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196316u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196320u32;
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
pub fn block_0x00218360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196324u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1315u32, 2196328u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2196332u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196336u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196340u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196344u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196348u32);
    emu.apc_no_count(6usize, 2196348u32, 40960u32, 2196352u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196356u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196360u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1287u32, 2196364u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2196368u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196372u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196376u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196380u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196384u32);
    emu.apc_no_count(6usize, 2196384u32, 40960u32, 2196388u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196392u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002183a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196396u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1272u32, 2196400u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2196404u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196408u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196412u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196416u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196420u32);
    emu.apc_no_count(6usize, 2196420u32, 40960u32, 2196424u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196428u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002183cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2196432u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2196436u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196440u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966648u32, 2196444u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196448u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1204u32, 2196452u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2196456u32);
    emu.adi_no_count(16usize, 2usize, 32u32, 2196460u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2196464u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2196468u32)?;
    emu.sw_no_count(13usize, 2usize, 36u32, 2196472u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2196476u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2196480u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2196484u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2196488u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2196492u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2196496u32)?;
    emu.sw_no_count(12usize, 2usize, 0u32, 2196500u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2196504u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2196508u32);
    emu.apc_no_count(1usize, 2196508u32, 36864u32, 2196512u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2196520u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196524u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196528u32;
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
pub fn block_0x00218430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196532u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 608u32, 2196536u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2196540u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196544u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196548u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196552u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196556u32);
    emu.apc_no_count(6usize, 2196556u32, 40960u32, 2196560u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196564u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967200u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00218454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196568u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1258u32, 2196572u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2196576u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196580u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196584u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196588u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196592u32);
    emu.apc_no_count(6usize, 2196592u32, 40960u32, 2196596u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196600u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2196604u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2196608u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2196612u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196616u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1296u32, 2196620u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2196624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2196780u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021852c));
}
#[inline]
pub fn block_0x00218490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196628u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1302u32, 2196632u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2196636u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196640u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196644u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196648u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196652u32);
    emu.apc_no_count(6usize, 2196652u32, 40960u32, 2196656u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196660u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002184b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2196664u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2196668u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2196672u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196676u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1728u32, 2196680u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2196684u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1176u32, 2196688u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2196692u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196696u32)?;
    emu.adi_no_count(17usize, 2usize, 0u32, 2196700u32);
    emu.sw_no_count(13usize, 2usize, 0u32, 2196704u32)?;
    emu.sw_no_count(14usize, 2usize, 4u32, 2196708u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2196712u32);
    emu.sw_no_count(12usize, 2usize, 32u32, 2196716u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2196720u32)?;
    emu.add_memory_rw_events(16usize);
    let return_addr = 2196724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2196820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218554));
}
#[inline]
pub fn block_0x002184f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2196728u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1236u32, 2196732u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2196736u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2196740u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196744u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196748u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196752u32);
    emu.apc_no_count(6usize, 2196752u32, 40960u32, 2196756u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196760u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 8u32, 2196764u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2196768u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2196772u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196776u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1176u32, 2196780u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2196780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021852c));
}
#[inline]
pub fn block_0x0021852c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2196784u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1160u32, 2196788u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2196792u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196796u32)?;
    emu.adi_no_count(17usize, 2usize, 32u32, 2196800u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2196804u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2196808u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2196812u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2196816u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2196820u32)?;
    emu.add_memory_rw_events(10usize);
    emu.pc = 2196820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218554));
}
#[inline]
pub fn block_0x00218554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2196824u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2196828u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2196832u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2196836u32)?;
    emu.sw_no_count(17usize, 2usize, 16u32, 2196840u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2196844u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2196848u32);
    emu.apc_no_count(1usize, 2196848u32, 36864u32, 2196852u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2196860u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196864u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196868u32;
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
pub fn block_0x00218584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2196872u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2196876u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2196880u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2196884u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2196888u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2196892u32);
    emu.apc_no_count(6usize, 2196892u32, 40960u32, 2196896u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2196900u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002185a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2196904u32)?;
    emu.lw_no_count(10usize, 10usize, 8u32, 2196908u32)?;
    emu.adi_no_count(13usize, 2usize, 0u32, 2196912u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2196916u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 676u32, 2196920u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2196924u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1228u32, 2196928u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2196932u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2196936u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2196940u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2196944u32)?;
    emu.adi_no_count(13usize, 2usize, 32u32, 2196948u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2196952u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2196956u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2196960u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2196964u32)?;
    emu.sw_no_count(15usize, 2usize, 8u32, 2196968u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2196972u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2196976u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2196980u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2196984u32);
    emu.apc_no_count(1usize, 2196984u32, 36864u32, 2196988u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2196992u32;
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
pub fn block_0x00218600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2196996u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2197000u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2197004u32;
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
