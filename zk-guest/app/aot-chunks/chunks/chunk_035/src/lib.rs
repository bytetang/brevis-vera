pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2237420u32;
pub const PC_MAX: u32 = 2239624u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 120usize] = [
        block_0x002223ec,
        block_0x002223fc,
        block_0x00222430,
        block_0x00222450,
        block_0x00222490,
        block_0x002224b8,
        block_0x002224c8,
        block_0x002224d0,
        block_0x002224e0,
        block_0x002224fc,
        block_0x00222514,
        block_0x00222518,
        block_0x0022253c,
        block_0x0022259c,
        block_0x002225c4,
        block_0x002225e0,
        block_0x002225f0,
        block_0x002225f8,
        block_0x00222608,
        block_0x00222624,
        block_0x0022263c,
        block_0x00222640,
        block_0x00222674,
        block_0x002226b0,
        block_0x002226dc,
        block_0x00222728,
        block_0x00222730,
        block_0x00222760,
        block_0x0022276c,
        block_0x00222784,
        block_0x00222788,
        block_0x00222798,
        block_0x0022279c,
        block_0x002227a0,
        block_0x002227b4,
        block_0x002227b8,
        block_0x00222804,
        block_0x00222808,
        block_0x00222824,
        block_0x00222828,
        block_0x0022282c,
        block_0x00222838,
        block_0x00222858,
        block_0x0022285c,
        block_0x00222878,
        block_0x00222880,
        block_0x00222890,
        block_0x0022289c,
        block_0x002228b0,
        block_0x002228c4,
        block_0x0022291c,
        block_0x00222924,
        block_0x0022292c,
        block_0x0022296c,
        block_0x00222970,
        block_0x00222980,
        block_0x0022298c,
        block_0x0022299c,
        block_0x002229a0,
        block_0x002229a4,
        block_0x002229b0,
        block_0x002229b4,
        block_0x002229c4,
        block_0x002229dc,
        block_0x002229f8,
        block_0x00222a1c,
        block_0x00222a28,
        block_0x00222a34,
        block_0x00222a48,
        block_0x00222a5c,
        block_0x00222a64,
        block_0x00222a6c,
        block_0x00222a70,
        block_0x00222a7c,
        block_0x00222a80,
        block_0x00222a84,
        block_0x00222a88,
        block_0x00222a8c,
        block_0x00222aa0,
        block_0x00222aa4,
        block_0x00222ab0,
        block_0x00222ad0,
        block_0x00222ad4,
        block_0x00222ae0,
        block_0x00222af0,
        block_0x00222af4,
        block_0x00222af8,
        block_0x00222b10,
        block_0x00222b14,
        block_0x00222b1c,
        block_0x00222b28,
        block_0x00222b34,
        block_0x00222b3c,
        block_0x00222b48,
        block_0x00222b54,
        block_0x00222b5c,
        block_0x00222b64,
        block_0x00222b6c,
        block_0x00222b7c,
        block_0x00222b90,
        block_0x00222b94,
        block_0x00222b98,
        block_0x00222ba8,
        block_0x00222bac,
        block_0x00222bb4,
        block_0x00222bb8,
        block_0x00222bbc,
        block_0x00222bc0,
        block_0x00222bd0,
        block_0x00222bd8,
        block_0x00222be0,
        block_0x00222be4,
        block_0x00222bfc,
        block_0x00222c00,
        block_0x00222c0c,
        block_0x00222c10,
        block_0x00222c50,
        block_0x00222c58,
        block_0x00222c6c,
        block_0x00222c88,
    ];
    const IDX: [u16; 552usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16,
        0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16,
        0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16,
        0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16,
        0u16, 32u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16,
        41u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16, 0u16, 53u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 58u16, 59u16, 60u16, 0u16, 0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16,
        71u16, 0u16, 72u16, 73u16, 0u16, 0u16, 74u16, 75u16, 76u16, 77u16, 78u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 86u16,
        87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 89u16, 0u16, 90u16, 0u16, 0u16,
        91u16, 0u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16,
        0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 101u16, 102u16, 0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 105u16,
        106u16, 107u16, 108u16, 0u16, 0u16, 0u16, 109u16, 0u16, 110u16, 0u16, 111u16,
        112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 114u16, 0u16, 0u16, 115u16, 116u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 117u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 120u16,
    ];
    if pc < 2237420u32 || pc > 2239624u32 {
        return None;
    }
    let word_offset = ((pc - 2237420u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002223ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2237424u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237428u32)?;
    emu.lw_no_count(6usize, 13usize, 12u32, 2237432u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2237436u32;
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
pub fn block_0x002223fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2237440u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2237444u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2237448u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2237452u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2237456u32);
    emu.lw_no_count(14usize, 11usize, 4u32, 2237460u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2237464u32)?;
    emu.lw_no_count(14usize, 14usize, 12u32, 2237468u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2237472u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2237476u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2237480u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2237484u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2237488u32;
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
pub fn block_0x00222430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 9usize, 0u32, 2237492u32)?;
    emu.sb_no_count(10usize, 9usize, 4u32, 2237496u32);
    emu.sb_no_count(0usize, 9usize, 5u32, 2237500u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2237504u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2237508u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2237512u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2237516u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237520u32;
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
pub fn block_0x00222450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2237524u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2237528u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2237532u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2237536u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2237540u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2237544u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2237548u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2237552u32);
    emu.adi_no_count(9usize, 15usize, 0u32, 2237556u32);
    emu.adi_no_count(18usize, 14usize, 0u32, 2237560u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2237564u32);
    emu.adi_no_count(20usize, 10usize, 0u32, 2237568u32);
    emu.lw_no_count(13usize, 10usize, 4u32, 2237572u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237576u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2237580u32)?;
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2237584u32;
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
#[inline]
pub fn block_0x00222490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 0u32, 2237588u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2237592u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2237596u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2237600u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2237604u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2237608u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2237612u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2237616u32);
    emu.apc_no_count(1usize, 2237616u32, 4294950912u32, 2237620u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002224b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2237628u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2237632u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2237636u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2237720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222518));
    } else {
        emu.pc = 2237640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002224c8));
    }
}
#[inline(always)]
pub fn block_0x002224c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2237644u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2237720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222518));
    } else {
        emu.pc = 2237648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002224d0));
    }
}
#[inline(always)]
pub fn block_0x002224d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2237652u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2237656u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2237660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2237692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002224fc));
    } else {
        emu.pc = 2237664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002224e0));
    }
}
#[inline(always)]
pub fn block_0x002224e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2237668u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237672u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2237676u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2237680u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1765u32, 2237684u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2237688u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2237692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237716u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222514));
}
#[inline(always)]
pub fn block_0x002224fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2237696u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237700u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2237704u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2237708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1764u32, 2237712u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2237716u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2237716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222514));
}
#[inline(always)]
pub fn block_0x00222514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2237720u32;
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
#[inline]
pub fn block_0x00222518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2237724u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2237728u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2237732u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2237736u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2237740u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2237744u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2237748u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2237752u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237756u32;
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
pub fn block_0x0022253c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2237760u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2237764u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2237768u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2237772u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2237776u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2237780u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2237784u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2237788u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2237792u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2237796u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2237800u32)?;
    emu.adi_no_count(8usize, 17usize, 0u32, 2237804u32);
    emu.adi_no_count(9usize, 16usize, 0u32, 2237808u32);
    emu.adi_no_count(18usize, 15usize, 0u32, 2237812u32);
    emu.adi_no_count(19usize, 14usize, 0u32, 2237816u32);
    emu.adi_no_count(20usize, 13usize, 0u32, 2237820u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2237824u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2237828u32)?;
    emu.lw_no_count(13usize, 21usize, 4u32, 2237832u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2237836u32)?;
    emu.lw_no_count(23usize, 2usize, 52u32, 2237840u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2237844u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2237848u32)?;
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2237852u32;
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
#[inline]
pub fn block_0x0022259c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 0u32, 2237856u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2237860u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2237864u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2237868u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2237872u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2237876u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2237880u32);
    emu.adi_no_count(14usize, 9usize, 0u32, 2237884u32);
    emu.apc_no_count(1usize, 2237884u32, 4294950912u32, 2237888u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002225c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2237896u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2237900u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2237904u32);
    emu.adi_no_count(13usize, 23usize, 0u32, 2237908u32);
    emu.adi_no_count(14usize, 24usize, 0u32, 2237912u32);
    emu.apc_no_count(1usize, 2237912u32, 4294950912u32, 2237916u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002225e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2237924u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2237928u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2237932u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2238016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222640));
    } else {
        emu.pc = 2237936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002225f0));
    }
}
#[inline(always)]
pub fn block_0x002225f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2237940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2238016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222640));
    } else {
        emu.pc = 2237944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002225f8));
    }
}
#[inline(always)]
pub fn block_0x002225f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2237948u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2237952u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2237956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2237988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222624));
    } else {
        emu.pc = 2237960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222608));
    }
}
#[inline(always)]
pub fn block_0x00222608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2237964u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237968u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2237972u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2237976u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1765u32, 2237980u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2237984u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2237988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238012u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022263c));
}
#[inline(always)]
pub fn block_0x00222624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2237992u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2237996u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2238000u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238004u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1764u32, 2238008u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2238012u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2238012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022263c));
}
#[inline(always)]
pub fn block_0x0022263c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2238016u32;
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
#[inline]
pub fn block_0x00222640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2238020u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2238024u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2238028u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2238032u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2238036u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2238040u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2238044u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2238048u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2238052u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2238056u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2238060u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2238064u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2238068u32;
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
pub fn block_0x00222674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2238072u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2238076u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2238080u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2238084u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2238088u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2238092u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2238096u32);
    emu.lw_no_count(13usize, 11usize, 4u32, 2238100u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2238104u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2238108u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2238112u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2238116u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2238120u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2238124u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2238128u32;
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
#[inline]
pub fn block_0x002226b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2238132u32);
    emu.sw_no_count(0usize, 18usize, 0u32, 2238136u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2238140u32)?;
    emu.sb_no_count(10usize, 18usize, 8u32, 2238144u32);
    emu.sb_no_count(11usize, 18usize, 9u32, 2238148u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2238152u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2238156u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2238160u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2238164u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2238168u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2238172u32;
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
pub fn block_0x002226dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2238176u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2238180u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2238184u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2238188u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2238192u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2238196u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2238200u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2238204u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2238208u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2238212u32)?;
    emu.adi_no_count(20usize, 14usize, 0u32, 2238216u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2238220u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2238224u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2238228u32);
    emu.lw_no_count(22usize, 10usize, 4u32, 2238232u32)?;
    emu.lw_no_count(21usize, 10usize, 0u32, 2238236u32)?;
    emu.lw_no_count(23usize, 22usize, 12u32, 2238240u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2238244u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2238248u32;
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
pub fn block_0x00222728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2238252u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2238304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222760));
    } else {
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    }
}
#[inline]
pub fn block_0x00222730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2238260u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2238264u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2238268u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2238272u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2238276u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2238280u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2238284u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2238288u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2238292u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2238296u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2238300u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2238304u32;
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
pub fn block_0x00222760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2238308u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2238312u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2238368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002227a0));
    } else {
        emu.pc = 2238316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022276c));
    }
}
#[inline(always)]
pub fn block_0x0022276c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1767u32, 2238324u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2238328u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2238332u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2238336u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2238340u32;
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
pub fn block_0x00222784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222788));
    }
}
#[inline(always)]
pub fn block_0x00222788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 12u32, 2238348u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2238352u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2238356u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2238360u32;
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
pub fn block_0x00222798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022279c));
    }
}
#[inline(always)]
pub fn block_0x0022279c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2238368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222828));
}
#[inline(always)]
pub fn block_0x002227a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1768u32, 2238376u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2238380u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2238384u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2238388u32;
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
pub fn block_0x002227b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002227b8));
    }
}
#[inline]
pub fn block_0x002227b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2238396u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2238400u32);
    emu.lw_no_count(11usize, 8usize, 8u32, 2238404u32)?;
    emu.lw_no_count(12usize, 8usize, 12u32, 2238408u32)?;
    emu.adi_no_count(13usize, 2usize, 12u32, 2238412u32);
    emu.sw_no_count(21usize, 2usize, 12u32, 2238416u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2238420u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2238424u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2238428u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1728u32, 2238432u32);
    emu.lw_no_count(14usize, 20usize, 12u32, 2238436u32)?;
    emu.sb_no_count(18usize, 2usize, 27u32, 2238440u32);
    emu.sw_no_count(13usize, 2usize, 28u32, 2238444u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2238448u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2238452u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2238456u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2238460u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2238464u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2238468u32;
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
pub fn block_0x00222804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222808));
    }
}
#[inline(always)]
pub fn block_0x00222808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2238476u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2238480u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2238484u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238488u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1762u32, 2238492u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2238496u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2238500u32;
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
pub fn block_0x00222824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222828));
    }
}
#[inline(always)]
pub fn block_0x00222828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2238556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022285c));
    } else {
        emu.pc = 2238508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022282c));
    }
}
#[inline(always)]
pub fn block_0x0022282c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2238512u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2238516u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2238556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022285c));
    } else {
        emu.pc = 2238520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222838));
    }
}
#[inline(always)]
pub fn block_0x00222838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2238524u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2238528u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2238532u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1771u32, 2238540u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2238544u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2238548u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2238552u32;
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
pub fn block_0x00222858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2238256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222730));
    } else {
        emu.pc = 2238556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022285c));
    }
}
#[inline(always)]
pub fn block_0x0022285c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2238560u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2238564u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2238568u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1770u32, 2238576u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2238580u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2238584u32;
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
pub fn block_0x00222878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2238588u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2238592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222730));
}
#[inline(always)]
pub fn block_0x00222880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2238596u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2238600u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2238604u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2238608u32;
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
pub fn block_0x00222890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 0u32, 2238612u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2238616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2238640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002228b0));
    } else {
        emu.pc = 2238620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022289c));
    }
}
#[inline(always)]
pub fn block_0x0022289c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238624u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 600u32, 2238628u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2238632u32);
    emu.apc_no_count(6usize, 2238632u32, 4294963200u32, 2238636u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2238640u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002228b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2238644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1272u32, 2238648u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2238652u32);
    emu.apc_no_count(6usize, 2238652u32, 4294963200u32, 2238656u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2238660u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002228c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2238664u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2238668u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2238672u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2238676u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2238680u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2238684u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2238688u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2238692u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2238696u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2238700u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2238704u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2238708u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2238712u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2238716u32)?;
    emu.adi_no_count(19usize, 11usize, 0u32, 2238720u32);
    emu.lw_no_count(21usize, 12usize, 4u32, 2238724u32)?;
    emu.lw_no_count(8usize, 12usize, 0u32, 2238728u32)?;
    emu.lw_no_count(9usize, 21usize, 16u32, 2238732u32)?;
    emu.adi_no_count(23usize, 10usize, 0u32, 2238736u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2238740u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2238744u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2238748u32;
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
pub fn block_0x0022291c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2238752u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2239504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c10));
    } else {
        emu.pc = 2238756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222924));
    }
}
#[inline(always)]
pub fn block_0x00222924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 24u32, 2238760u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2239404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bac));
    } else {
        emu.pc = 2238764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022292c));
    }
}
#[inline]
pub fn block_0x0022292c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2238768u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2238772u32)?;
    emu.adi_no_count(20usize, 0usize, 0u32, 2238776u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2238780u32);
    emu.sbr_no_count(10usize, 0usize, 19usize, 2238784u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2238788u32)?;
    emu.adi_no_count(27usize, 0usize, 4294967201u32, 2238792u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2238796u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 0usize, 1u32, 2238800u32);
    emu.adi_no_count(26usize, 0usize, 34u32, 2238804u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2238808u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2238812u32)?;
    emu.adi_no_count(21usize, 0usize, 92u32, 2238816u32);
    emu.adi_no_count(18usize, 23usize, 0u32, 2238820u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2238824u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2238828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222980));
}
#[inline(always)]
pub fn block_0x0022296c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2238832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2238832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222970));
}
#[inline(always)]
pub fn block_0x00222970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 9usize, 2238836u32);
    emu.sbr_no_count(13usize, 8usize, 18usize, 2238840u32);
    emu.adr_no_count(9usize, 10usize, 25usize, 2238844u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2239576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c58));
    } else {
        emu.pc = 2238848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222980));
    }
}
#[inline(always)]
pub fn block_0x00222980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2238852u32);
    emu.adr_no_count(8usize, 18usize, 13usize, 2238856u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2238860u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2238860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022298c));
}
#[inline(always)]
pub fn block_0x0022298c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 25usize, 2238864u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2238868u32);
    emu.adi_no_count(14usize, 12usize, 4294967169u32, 2238872u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2238900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229b4));
    } else {
        emu.pc = 2238876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022299c));
    }
}
#[inline(always)]
pub fn block_0x0022299c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2238900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229b4));
    } else {
        emu.pc = 2238880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229a0));
    }
}
#[inline(always)]
pub fn block_0x002229a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2238900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229b4));
    } else {
        emu.pc = 2238884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229a4));
    }
}
#[inline(always)]
pub fn block_0x002229a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2238888u32);
    emu.adr_no_count(10usize, 11usize, 25usize, 2238892u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2238860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022298c));
    } else {
        emu.pc = 2238896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229b0));
    }
}
#[inline(always)]
pub fn block_0x002229b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2238900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222b7c));
}
#[inline(always)]
pub fn block_0x002229b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 10usize, 0u32, 2238904u32);
    emu.adi_no_count(18usize, 10usize, 1u32, 2238908u32);
    emu.ani_no_count(22usize, 11usize, 255u32, 2238912u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2239028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a34));
    } else {
        emu.pc = 2238916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229c4));
    }
}
#[inline(always)]
pub fn block_0x002229c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 18usize, 0u32, 2238920u32);
    emu.ani_no_count(11usize, 22usize, 31u32, 2238924u32);
    emu.adi_no_count(18usize, 10usize, 2u32, 2238928u32);
    emu.ani_no_count(13usize, 12usize, 63u32, 2238932u32);
    emu.adi_no_count(12usize, 0usize, 224u32, 2238936u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2239004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a1c));
    } else {
        emu.pc = 2238940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229dc));
    }
}
#[inline(always)]
pub fn block_0x002229dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 18usize, 0u32, 2238944u32);
    emu.adi_no_count(12usize, 10usize, 3u32, 2238948u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2238952u32);
    emu.ani_no_count(14usize, 14usize, 63u32, 2238956u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2238960u32);
    emu.adi_no_count(14usize, 0usize, 240u32, 2238964u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2239016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a28));
    } else {
        emu.pc = 2238968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002229f8));
    }
}
#[inline]
pub fn block_0x002229f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 4u32, 2238972u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2238976u32);
    emu.sli_no_count(11usize, 11usize, 29u32, 2238980u32);
    emu.sri_no_count(11usize, 11usize, 11u32, 2238984u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2238988u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2238992u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2238996u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2239000u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2239004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222a34));
}
#[inline(always)]
pub fn block_0x00222a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 6u32, 2239008u32);
    emu.orr_no_count(22usize, 11usize, 13usize, 2239012u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2239016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222a34));
}
#[inline(always)]
pub fn block_0x00222a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 12u32, 2239020u32);
    emu.orr_no_count(22usize, 13usize, 11usize, 2239024u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2239028u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2239028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222a34));
}
#[inline(always)]
pub fn block_0x00222a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2239032u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2239036u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2239040u32)?;
    emu.apc_no_count(1usize, 2239040u32, 4294959104u32, 2239044u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 40u32, 2239052u32);
    emu.lbu_no_count(11usize, 2usize, 41u32, 2239056u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2239060u32);
    emu.ani_no_count(10usize, 11usize, 255u32, 2239064u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2239316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b54));
    } else {
        emu.pc = 2239068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a5c));
    }
}
#[inline(always)]
pub fn block_0x00222a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 9usize, 25usize, 2239072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2239624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c88));
    } else {
        emu.pc = 2239076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a64));
    }
}
#[inline(always)]
pub fn block_0x00222a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 23usize, 20usize, 2239080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2239108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a84));
    } else {
        emu.pc = 2239084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a6c));
    }
}
#[inline(always)]
pub fn block_0x00222a6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2239104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a80));
    } else {
        emu.pc = 2239088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a70));
    }
}
#[inline(always)]
pub fn block_0x00222a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 11usize, 0u32, 2239092u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2239096u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2239108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a84));
    } else {
        emu.pc = 2239100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a7c));
    }
}
#[inline(always)]
pub fn block_0x00222a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2239104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c88));
}
#[inline(always)]
pub fn block_0x00222a80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2239624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c88));
    } else {
        emu.pc = 2239108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a84));
    }
}
#[inline(always)]
pub fn block_0x00222a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ab0));
    } else {
        emu.pc = 2239112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a88));
    }
}
#[inline(always)]
pub fn block_0x00222a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2239140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222aa4));
    } else {
        emu.pc = 2239116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222a8c));
    }
}
#[inline(always)]
pub fn block_0x00222a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 9usize, 2239120u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2239124u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2239128u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2239132u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2239152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ab0));
    } else {
        emu.pc = 2239136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222aa0));
    }
}
#[inline(always)]
pub fn block_0x00222aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2239140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c88));
}
#[inline(always)]
pub fn block_0x00222aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2239144u32)?;
    emu.adr_no_count(10usize, 13usize, 10usize, 2239148u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2239624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c88));
    } else {
        emu.pc = 2239152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ab0));
    }
}
#[inline(always)]
pub fn block_0x00222ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 23usize, 0u32, 2239156u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2239160u32)?;
    emu.lw_no_count(23usize, 10usize, 12u32, 2239164u32)?;
    emu.sbr_no_count(12usize, 9usize, 20usize, 2239168u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2239172u32);
    emu.lw_no_count(20usize, 2usize, 24u32, 2239176u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2239180u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2239184u32;
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
pub fn block_0x00222ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c50));
    } else {
        emu.pc = 2239188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ad4));
    }
}
#[inline(always)]
pub fn block_0x00222ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 41u32, 2239192u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2239196u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2239224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222af8));
    } else {
        emu.pc = 2239200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ae0));
    }
}
#[inline(always)]
pub fn block_0x00222ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 28u32, 2239204u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2239208u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2239212u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2239216u32;
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
pub fn block_0x00222af0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b14));
    } else {
        emu.pc = 2239220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222af4));
    }
}
#[inline(always)]
pub fn block_0x00222af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2239224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c50));
}
#[inline(always)]
pub fn block_0x00222af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 40u32, 2239228u32);
    emu.sbr_no_count(12usize, 10usize, 11usize, 2239232u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2239236u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2239240u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2239244u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2239248u32;
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
pub fn block_0x00222b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c50));
    } else {
        emu.pc = 2239252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b14));
    }
}
#[inline(always)]
pub fn block_0x00222b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2239256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2239272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b28));
    } else {
        emu.pc = 2239260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b1c));
    }
}
#[inline(always)]
pub fn block_0x00222b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2239264u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2239268u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2239272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239304u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222b48));
}
#[inline(always)]
pub fn block_0x00222b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2239276u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2239280u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2239292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b3c));
    } else {
        emu.pc = 2239284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b34));
    }
}
#[inline(always)]
pub fn block_0x00222b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2239288u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239304u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222b48));
}
#[inline(always)]
pub fn block_0x00222b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2239296u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2239300u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2239304u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2239304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222b48));
}
#[inline(always)]
pub fn block_0x00222b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2239308u32);
    emu.adr_no_count(10usize, 9usize, 25usize, 2239312u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2239316u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2239316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222b54));
}
#[inline(always)]
pub fn block_0x00222b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2239320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2238828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022296c));
    } else {
        emu.pc = 2239324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b5c));
    }
}
#[inline(always)]
pub fn block_0x00222b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2239328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2239340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b6c));
    } else {
        emu.pc = 2239332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b64));
    }
}
#[inline(always)]
pub fn block_0x00222b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2239336u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222970));
}
#[inline(always)]
pub fn block_0x00222b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2239344u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2239348u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2239352u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2239356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2238832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222970));
}
#[inline(always)]
pub fn block_0x00222b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 9usize, 2239360u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2239364u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2239368u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2239372u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2239596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c6c));
    } else {
        emu.pc = 2239376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b90));
    }
}
#[inline(always)]
pub fn block_0x00222b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2239416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bb8));
    } else {
        emu.pc = 2239380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b94));
    }
}
#[inline(always)]
pub fn block_0x00222b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2239412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bb4));
    } else {
        emu.pc = 2239384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b98));
    }
}
#[inline(always)]
pub fn block_0x00222b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 20usize, 2239388u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2239392u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2239396u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2239416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bb8));
    } else {
        emu.pc = 2239400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222ba8));
    }
}
#[inline(always)]
pub fn block_0x00222ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2239404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c6c));
}
#[inline(always)]
pub fn block_0x00222bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2239408u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239412u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222be4));
}
#[inline(always)]
pub fn block_0x00222bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2239596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c6c));
    } else {
        emu.pc = 2239416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bb8));
    }
}
#[inline(always)]
pub fn block_0x00222bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bd8));
    } else {
        emu.pc = 2239420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bbc));
    }
}
#[inline(always)]
pub fn block_0x00222bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2239456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222be0));
    } else {
        emu.pc = 2239424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bc0));
    }
}
#[inline(always)]
pub fn block_0x00222bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 13usize, 2239428u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2239432u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2239436u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2239596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c6c));
    } else {
        emu.pc = 2239440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222bd0));
    }
}
#[inline(always)]
pub fn block_0x00222bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 13usize, 0u32, 2239444u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222be4));
}
#[inline(always)]
pub fn block_0x00222bd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2239452u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222be4));
}
#[inline(always)]
pub fn block_0x00222be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2239596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c6c));
    } else {
        emu.pc = 2239460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222be4));
    }
}
#[inline(always)]
pub fn block_0x00222be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 12u32, 2239464u32)?;
    emu.sbr_no_count(12usize, 19usize, 20usize, 2239468u32);
    emu.adr_no_count(11usize, 23usize, 20usize, 2239472u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2239476u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2239480u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2239484u32;
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
pub fn block_0x00222bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2239504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c10));
    } else {
        emu.pc = 2239488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c00));
    }
}
#[inline(always)]
pub fn block_0x00222c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 34u32, 2239492u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2239496u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2239500u32;
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
pub fn block_0x00222c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2239504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2239504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c10));
}
#[inline]
pub fn block_0x00222c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2239508u32);
    emu.lw_no_count(1usize, 2usize, 92u32, 2239512u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2239516u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2239520u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2239524u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2239528u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2239532u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2239536u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2239540u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2239544u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2239548u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2239552u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2239556u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2239560u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2239564u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239568u32;
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
pub fn block_0x00222c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2239572u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2239576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2239504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222c10));
}
#[inline(always)]
pub fn block_0x00222c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 10usize, 25usize, 2239580u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2239584u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2239588u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2239592u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2239376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222b90));
    } else {
        emu.pc = 2239596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222c6c));
    }
}
#[inline(always)]
pub fn block_0x00222c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2239600u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1296u32, 2239604u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2239608u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2239612u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2239616u32);
    emu.apc_no_count(1usize, 2239616u32, 4294946816u32, 2239620u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239624u32;
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
pub fn block_0x00222c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2239628u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1280u32, 2239632u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2239636u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2239640u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2239644u32);
    emu.apc_no_count(1usize, 2239644u32, 4294946816u32, 2239648u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2239652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
