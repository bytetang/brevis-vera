pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2199336u32;
pub const PC_MAX: u32 = 2201172u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00218f28,
        block_0x00218f60,
        block_0x00218f64,
        block_0x00218f6c,
        block_0x00218f7c,
        block_0x00218f94,
        block_0x00218fa4,
        block_0x00218fac,
        block_0x00218fbc,
        block_0x00218fc4,
        block_0x00218fcc,
        block_0x00218fdc,
        block_0x00218ff0,
        block_0x00219008,
        block_0x00219040,
        block_0x00219078,
        block_0x0021907c,
        block_0x00219084,
        block_0x00219094,
        block_0x002190ac,
        block_0x002190bc,
        block_0x002190c4,
        block_0x002190d4,
        block_0x002190dc,
        block_0x002190e4,
        block_0x002190f4,
        block_0x00219108,
        block_0x00219120,
        block_0x00219158,
        block_0x00219190,
        block_0x00219194,
        block_0x0021919c,
        block_0x002191ac,
        block_0x002191c4,
        block_0x002191d4,
        block_0x002191dc,
        block_0x002191ec,
        block_0x002191f4,
        block_0x002191fc,
        block_0x0021920c,
        block_0x00219220,
        block_0x00219238,
        block_0x00219270,
        block_0x002192c4,
        block_0x002192c8,
        block_0x002192ec,
        block_0x002192f0,
        block_0x002192fc,
        block_0x00219314,
        block_0x00219328,
        block_0x00219330,
        block_0x00219340,
        block_0x00219348,
        block_0x00219354,
        block_0x0021937c,
        block_0x0021938c,
        block_0x00219394,
        block_0x0021939c,
        block_0x002193ac,
        block_0x002193c0,
        block_0x002193c4,
        block_0x002193c8,
        block_0x002193dc,
        block_0x002193e4,
        block_0x002193ec,
        block_0x0021941c,
        block_0x00219430,
        block_0x00219458,
        block_0x00219478,
        block_0x00219484,
        block_0x002194a0,
        block_0x002194b0,
        block_0x002194b8,
        block_0x002194cc,
        block_0x002194d0,
        block_0x002194e0,
        block_0x002194e4,
        block_0x002194ec,
        block_0x002194fc,
        block_0x00219500,
        block_0x0021950c,
        block_0x00219510,
        block_0x00219514,
        block_0x00219530,
        block_0x00219538,
        block_0x0021953c,
        block_0x00219550,
        block_0x00219558,
        block_0x0021956c,
        block_0x00219570,
        block_0x0021957c,
        block_0x00219584,
        block_0x00219588,
        block_0x00219590,
        block_0x00219594,
        block_0x00219598,
        block_0x0021959c,
        block_0x002195b8,
        block_0x002195c8,
        block_0x002195cc,
        block_0x002195d4,
        block_0x002195e4,
        block_0x002195f8,
        block_0x00219610,
        block_0x00219618,
        block_0x00219638,
        block_0x0021964c,
        block_0x00219654,
    ];
    const IDX: [u16; 460usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 2u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16,
        10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16,
        19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 45u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 51u16,
        0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 62u16,
        0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        75u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 79u16,
        80u16, 0u16, 0u16, 81u16, 82u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        84u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 91u16, 0u16, 92u16, 93u16, 0u16,
        94u16, 95u16, 96u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16,
        0u16, 0u16, 99u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16,
        0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16,
        108u16,
    ];
    if pc < 2199336u32 || pc > 2201172u32 {
        return None;
    }
    let word_offset = ((pc - 2199336u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00218f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2199340u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2199344u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2199348u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2199352u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2199356u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2199360u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2199364u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2199368u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2199372u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199376u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1380u32, 2199380u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2199384u32);
    emu.apc_no_count(1usize, 2199384u32, 32768u32, 2199388u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f94));
    } else {
        emu.pc = 2199396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f64));
    }
}
#[inline(always)]
pub fn block_0x00218f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2199400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2199560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219008));
    } else {
        emu.pc = 2199404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f6c));
    }
}
#[inline(always)]
pub fn block_0x00218f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2199408u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2199412u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2199416u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2199420u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2199420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00218f7c));
}
#[inline(always)]
pub fn block_0x00218f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2199424u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199428u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2199432u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2199436u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2199440u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199444u32;
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
pub fn block_0x00218f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2199448u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2199452u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2199456u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2199468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fac));
    } else {
        emu.pc = 2199460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fa4));
    }
}
#[inline(always)]
pub fn block_0x00218fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2199464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2199420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218f7c));
    } else {
        emu.pc = 2199468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fac));
    }
}
#[inline(always)]
pub fn block_0x00218fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2199472u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2199476u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2199480u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fc4));
    } else {
        emu.pc = 2199484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fbc));
    }
}
#[inline(always)]
pub fn block_0x00218fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2199488u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2199492u32;
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
pub fn block_0x00218fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2199496u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fdc));
    } else {
        emu.pc = 2199500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00218fcc));
    }
}
#[inline(always)]
pub fn block_0x00218fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2199504u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2199508u32);
    emu.apc_no_count(1usize, 2199508u32, 4294873088u32, 2199512u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2199520u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2199524u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2199528u32);
    emu.apc_no_count(1usize, 2199528u32, 4294873088u32, 2199532u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00218ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2199540u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199544u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2199548u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2199552u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2199556u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199560u32;
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
pub fn block_0x00219008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2199564u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1544u32, 2199568u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2199572u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2199576u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2199580u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2199584u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2199588u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2199592u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2199596u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199600u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1552u32, 2199604u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2199608u32);
    emu.apc_no_count(1usize, 2199608u32, 20480u32, 2199612u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199616u32;
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
#[inline]
pub fn block_0x00219040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2199620u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2199624u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2199628u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2199632u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2199636u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2199640u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2199644u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2199648u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2199652u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199656u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1356u32, 2199660u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2199664u32);
    emu.apc_no_count(1usize, 2199664u32, 32768u32, 2199668u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199672u32;
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
pub fn block_0x00219078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2199724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190ac));
    } else {
        emu.pc = 2199676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021907c));
    }
}
#[inline(always)]
pub fn block_0x0021907c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2199680u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2199840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219120));
    } else {
        emu.pc = 2199684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219084));
    }
}
#[inline(always)]
pub fn block_0x00219084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2199688u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2199692u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2199696u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2199700u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2199700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219094));
}
#[inline(always)]
pub fn block_0x00219094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2199704u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199708u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2199712u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2199716u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2199720u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199724u32;
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
pub fn block_0x002190ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2199728u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2199732u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2199736u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2199748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190c4));
    } else {
        emu.pc = 2199740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190bc));
    }
}
#[inline(always)]
pub fn block_0x002190bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2199744u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2199700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219094));
    } else {
        emu.pc = 2199748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190c4));
    }
}
#[inline(always)]
pub fn block_0x002190c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2199752u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2199756u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2199760u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190dc));
    } else {
        emu.pc = 2199764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190d4));
    }
}
#[inline(always)]
pub fn block_0x002190d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2199768u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2199772u32;
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
pub fn block_0x002190dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2199776u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2199796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190f4));
    } else {
        emu.pc = 2199780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002190e4));
    }
}
#[inline(always)]
pub fn block_0x002190e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2199784u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2199788u32);
    emu.apc_no_count(1usize, 2199788u32, 4294873088u32, 2199792u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002190f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2199800u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2199804u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2199808u32);
    emu.apc_no_count(1usize, 2199808u32, 4294873088u32, 2199812u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966932u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2199820u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199824u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2199828u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2199832u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2199836u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199840u32;
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
pub fn block_0x00219120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2199844u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1544u32, 2199848u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2199852u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2199856u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2199860u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2199864u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2199868u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2199872u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2199876u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1552u32, 2199884u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2199888u32);
    emu.apc_no_count(1usize, 2199888u32, 20480u32, 2199892u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2199900u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2199904u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2199908u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2199912u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2199916u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2199920u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2199924u32);
    emu.sb_no_count(18usize, 2usize, 8u32, 2199928u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2199932u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2199936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1332u32, 2199940u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2199944u32);
    emu.apc_no_count(1usize, 2199944u32, 32768u32, 2199948u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2199952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191c4));
    } else {
        emu.pc = 2199956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219194));
    }
}
#[inline(always)]
pub fn block_0x00219194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2199960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2200120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219238));
    } else {
        emu.pc = 2199964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021919c));
    }
}
#[inline(always)]
pub fn block_0x0021919c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2199968u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2199972u32)?;
    emu.sw_no_count(10usize, 9usize, 0u32, 2199976u32)?;
    emu.sw_no_count(11usize, 9usize, 4u32, 2199980u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2199980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002191ac));
}
#[inline(always)]
pub fn block_0x002191ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2199984u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2199988u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2199992u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2199996u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2200000u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200004u32;
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
pub fn block_0x002191c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2200008u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2200012u32)?;
    emu.sb_no_count(18usize, 9usize, 0u32, 2200016u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2200028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191dc));
    } else {
        emu.pc = 2200020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191d4));
    }
}
#[inline(always)]
pub fn block_0x002191d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2200024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2199980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191ac));
    } else {
        emu.pc = 2200028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191dc));
    }
}
#[inline(always)]
pub fn block_0x002191dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2200032u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2200036u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2200040u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2200052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191f4));
    } else {
        emu.pc = 2200044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191ec));
    }
}
#[inline(always)]
pub fn block_0x002191ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2200048u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2200052u32;
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
pub fn block_0x002191f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2200056u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2200076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021920c));
    } else {
        emu.pc = 2200060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002191fc));
    }
}
#[inline(always)]
pub fn block_0x002191fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2200064u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2200068u32);
    emu.apc_no_count(1usize, 2200068u32, 4294873088u32, 2200072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021920c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2200080u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2200084u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2200088u32);
    emu.apc_no_count(1usize, 2200088u32, 4294873088u32, 2200092u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200096u32;
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
pub fn block_0x00219220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2200100u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2200104u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2200108u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2200112u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2200116u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200120u32;
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
pub fn block_0x00219238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200124u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1544u32, 2200128u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2200132u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2200136u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2200140u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2200144u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2200148u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2200152u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2200156u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2200160u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1552u32, 2200164u32);
    emu.adi_no_count(10usize, 2usize, 24u32, 2200168u32);
    emu.apc_no_count(1usize, 2200168u32, 16384u32, 2200172u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2200180u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2200184u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2200188u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2200192u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2200196u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2200200u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2200204u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2200208u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2200212u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2200216u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2200220u32);
    emu.lw_no_count(19usize, 10usize, 8u32, 2200224u32)?;
    emu.lw_no_count(10usize, 19usize, 4u32, 2200228u32)?;
    emu.lw_no_count(21usize, 19usize, 8u32, 2200232u32)?;
    emu.lw_no_count(20usize, 19usize, 12u32, 2200236u32)?;
    emu.lw_no_count(12usize, 19usize, 0u32, 2200240u32)?;
    emu.sltru_no_count(13usize, 21usize, 10usize, 2200244u32);
    emu.sltiu_no_count(14usize, 20usize, 1u32, 2200248u32);
    emu.anr_no_count(15usize, 14usize, 13usize, 2200252u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2200256u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2200264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192c8));
    } else {
        emu.pc = 2200260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192c4));
    }
}
#[inline(always)]
pub fn block_0x002192c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2200264u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002192c8));
}
#[inline]
pub fn block_0x002192c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2200268u32);
    emu.orr_no_count(14usize, 14usize, 21usize, 2200272u32);
    emu.sbr_no_count(14usize, 10usize, 14usize, 2200276u32);
    emu.sltru_no_count(10usize, 10usize, 14usize, 2200280u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2200284u32);
    emu.anr_no_count(22usize, 10usize, 14usize, 2200288u32);
    emu.adr_no_count(10usize, 12usize, 13usize, 2200292u32);
    emu.adi_no_count(18usize, 22usize, 0u32, 2200296u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2200304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192f0));
    } else {
        emu.pc = 2200300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002192ec));
    }
}
#[inline(always)]
pub fn block_0x002192ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 9usize, 0u32, 2200304u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002192f0));
}
#[inline(always)]
pub fn block_0x002192f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 18usize, 0u32, 2200308u32);
    emu.apc_no_count(1usize, 2200308u32, 4294905856u32, 2200312u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002192fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 21usize, 18usize, 2200320u32);
    emu.sltru_no_count(10usize, 18usize, 21usize, 2200324u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2200328u32);
    emu.sw_no_count(18usize, 19usize, 8u32, 2200332u32)?;
    emu.sw_no_count(10usize, 19usize, 12u32, 2200336u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2200360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219328));
    } else {
        emu.pc = 2200340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219314));
    }
}
#[inline(always)]
pub fn block_0x00219314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2200344u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 10usize, 1448u32, 2200348u32)?;
    emu.ani_no_count(12usize, 19usize, 255u32, 2200352u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2200356u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2200368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219330));
    } else {
        emu.pc = 2200360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219328));
    }
}
#[inline(always)]
pub fn block_0x00219328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2200364u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2200368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219354));
}
#[inline(always)]
pub fn block_0x00219330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 10usize, 1452u32, 2200372u32)?;
    emu.lbu_no_count(10usize, 8usize, 0u32, 2200376u32);
    emu.lw_no_count(9usize, 8usize, 4u32, 2200380u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2200444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021937c));
    } else {
        emu.pc = 2200384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219340));
    }
}
#[inline(always)]
pub fn block_0x00219340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2200388u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2200444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021937c));
    } else {
        emu.pc = 2200392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219348));
    }
}
#[inline(always)]
pub fn block_0x00219348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 8usize, 0u32, 2200396u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2200400u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2200404u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2200404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219354));
}
#[inline]
pub fn block_0x00219354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2200408u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200412u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200416u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2200420u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2200424u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2200428u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2200432u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2200436u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200440u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200444u32;
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
pub fn block_0x0021937c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 9usize, 4u32, 2200448u32)?;
    emu.lw_no_count(11usize, 21usize, 0u32, 2200452u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2200456u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2200468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219394));
    } else {
        emu.pc = 2200460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021938c));
    }
}
#[inline(always)]
pub fn block_0x0021938c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2200464u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2200468u32;
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
pub fn block_0x00219394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 21usize, 4u32, 2200472u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2200492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193ac));
    } else {
        emu.pc = 2200476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021939c));
    }
}
#[inline(always)]
pub fn block_0x0021939c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2200480u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2200484u32);
    emu.apc_no_count(1usize, 2200484u32, 4294873088u32, 2200488u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002193ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2200496u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2200500u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2200504u32);
    emu.apc_no_count(1usize, 2200504u32, 4294873088u32, 2200508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200512u32;
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
pub fn block_0x002193c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2200516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200392u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219348));
}
#[inline(always)]
pub fn block_0x002193c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193e4));
    } else {
        emu.pc = 2200520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002193c8));
    }
}
#[inline(always)]
pub fn block_0x002193c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2200524u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2200528u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2200532u32);
    emu.apc_no_count(1usize, 2200532u32, 4294905856u32, 2200536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200540u32;
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
#[inline(always)]
pub fn block_0x002193dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2200544u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2200548u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2200548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002193e4));
}
#[inline(always)]
pub fn block_0x002193e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2200552u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200556u32;
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
pub fn block_0x002193ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2200560u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2200564u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2200568u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2200572u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2200576u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2200580u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2200584u32);
    emu.lw_no_count(9usize, 10usize, 8u32, 2200588u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2200592u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2200596u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2200600u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2200664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219458));
    } else {
        emu.pc = 2200604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021941c));
    }
}
#[inline(always)]
pub fn block_0x0021941c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2200608u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2200612u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2200616u32);
    emu.apc_no_count(1usize, 2200616u32, 4294905856u32, 2200620u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(180u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00219430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2200628u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2200632u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2200636u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2200640u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2200644u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2200648u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2200652u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2200656u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2200660u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200664u32;
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
pub fn block_0x00219458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2200668u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2200672u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2200676u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2200680u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2200684u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2200688u32);
    emu.apc_no_count(1usize, 2200688u32, 0u32, 2200692u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2200700u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2200704u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200604u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021941c));
}
#[inline(always)]
pub fn block_0x00219484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2200712u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2200716u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2200720u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2200724u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2200728u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2200732u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2200844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021950c));
    } else {
        emu.pc = 2200736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194a0));
    }
}
#[inline(always)]
pub fn block_0x002194a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2200740u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2200744u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2200748u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2200800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194e0));
    } else {
        emu.pc = 2200752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194b0));
    }
}
#[inline(always)]
pub fn block_0x002194b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2200756u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2200800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194e0));
    } else {
        emu.pc = 2200760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194b8));
    }
}
#[inline(always)]
pub fn block_0x002194b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2200764u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2200768u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2200772u32);
    emu.apc_no_count(1usize, 2200772u32, 4294873088u32, 2200776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002194cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219500));
    } else {
        emu.pc = 2200784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194d0));
    }
}
#[inline(always)]
pub fn block_0x002194d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2200788u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2200792u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2200796u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2200800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219514));
}
#[inline(always)]
pub fn block_0x002194e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219530));
    } else {
        emu.pc = 2200804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194e4));
    }
}
#[inline(always)]
pub fn block_0x002194e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2200804u32, 4294901760u32, 2200808u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002194ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2200816u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2200820u32);
    emu.apc_no_count(1usize, 2200820u32, 4294873088u32, 2200824u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200828u32;
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
pub fn block_0x002194fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2200784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194d0));
    } else {
        emu.pc = 2200832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219500));
    }
}
#[inline(always)]
pub fn block_0x00219500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2200836u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2200840u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2200844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219510));
}
#[inline(always)]
pub fn block_0x0021950c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2200848u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219510));
}
#[inline(always)]
pub fn block_0x00219510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2200852u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219514));
}
#[inline(always)]
pub fn block_0x00219514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2200856u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2200860u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2200864u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2200868u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2200872u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2200876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2200880u32;
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
pub fn block_0x00219530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2200884u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2200784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002194d0));
    } else {
        emu.pc = 2200888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219538));
    }
}
#[inline(always)]
pub fn block_0x00219538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2200892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219500));
}
#[inline(always)]
pub fn block_0x0021953c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2200896u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2200900u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2200904u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2200908u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2201144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219638));
    } else {
        emu.pc = 2200912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219550));
    }
}
#[inline(always)]
pub fn block_0x00219550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 11usize, 12usize, 2200916u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2201144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219638));
    } else {
        emu.pc = 2200920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219558));
    }
}
#[inline(always)]
pub fn block_0x00219558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2200924u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2200928u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2200932u32)?;
    emu.sli_no_count(10usize, 13usize, 1u32, 2200936u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2200944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219570));
    } else {
        emu.pc = 2200940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021956c));
    }
}
#[inline(always)]
pub fn block_0x0021956c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2200944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219570));
}
#[inline(always)]
pub fn block_0x00219570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1025u32, 2200948u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2200952u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2200968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219588));
    } else {
        emu.pc = 2200956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021957c));
    }
}
#[inline(always)]
pub fn block_0x0021957c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2200960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2200976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219590));
    } else {
        emu.pc = 2200964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219584));
    }
}
#[inline(always)]
pub fn block_0x00219584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2200968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2200980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219594));
}
#[inline(always)]
pub fn block_0x00219588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2200972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2200980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219594));
    } else {
        emu.pc = 2200976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219590));
    }
}
#[inline(always)]
pub fn block_0x00219590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2200980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219594));
}
#[inline(always)]
pub fn block_0x00219594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2200988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021959c));
    } else {
        emu.pc = 2200984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219598));
    }
}
#[inline(always)]
pub fn block_0x00219598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2200988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2200988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021959c));
}
#[inline(always)]
pub fn block_0x0021959c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 15usize, 14usize, 2200992u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2200996u32);
    emu.sbr_no_count(11usize, 0usize, 15usize, 2201000u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2201004u32);
    emu.mulhu_no_count(12usize, 11usize, 9usize, 2201008u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2201012u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2201044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195d4));
    } else {
        emu.pc = 2201016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195b8));
    }
}
#[inline(always)]
pub fn block_0x002195b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 11usize, 9usize, 2201020u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2201024u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 11usize, 15usize, 2201028u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2201172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219654));
    } else {
        emu.pc = 2201032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195c8));
    }
}
#[inline(always)]
pub fn block_0x002195c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2201060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195e4));
    } else {
        emu.pc = 2201036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002195cc));
    }
}
#[inline(always)]
pub fn block_0x002195cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2201040u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2201044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2201080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002195f8));
}
#[inline(always)]
pub fn block_0x002195d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201048u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2201052u32);
    emu.apc_no_count(1usize, 2201052u32, 8192u32, 2201056u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002195e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2201064u32)?;
    emu.mul_no_count(11usize, 13usize, 14usize, 2201068u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2201072u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2201076u32)?;
    emu.adi_no_count(10usize, 15usize, 0u32, 2201080u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2201080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002195f8));
}
#[inline(always)]
pub fn block_0x002195f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2201084u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2201088u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2201092u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2201096u32);
    emu.apc_no_count(1usize, 2201096u32, 0u32, 2201100u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00219610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2201108u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2201164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021964c));
    } else {
        emu.pc = 2201112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00219618));
    }
}
#[inline(always)]
pub fn block_0x00219618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2201116u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2201120u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2201124u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2201128u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2201132u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2201136u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2201140u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201144u32;
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
pub fn block_0x00219638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2201148u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201152u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2201156u32);
    emu.apc_no_count(1usize, 2201156u32, 8192u32, 2201160u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021964c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2201168u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2201172u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2201172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00219654));
}
#[inline(always)]
pub fn block_0x00219654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2201176u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1568u32, 2201180u32);
    emu.apc_no_count(1usize, 2201180u32, 8192u32, 2201184u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2201188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
