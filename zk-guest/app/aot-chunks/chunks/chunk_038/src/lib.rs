pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2243900u32;
pub const PC_MAX: u32 = 2245440u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x00223d3c,
        block_0x00223d50,
        block_0x00223d60,
        block_0x00223d64,
        block_0x00223d6c,
        block_0x00223d70,
        block_0x00223d84,
        block_0x00223db4,
        block_0x00223db8,
        block_0x00223dc0,
        block_0x00223dd4,
        block_0x00223de0,
        block_0x00223de4,
        block_0x00223dec,
        block_0x00223e08,
        block_0x00223e0c,
        block_0x00223e24,
        block_0x00223e38,
        block_0x00223e44,
        block_0x00223e48,
        block_0x00223e5c,
        block_0x00223e84,
        block_0x00223e88,
        block_0x00223e90,
        block_0x00223e98,
        block_0x00223ea8,
        block_0x00223eac,
        block_0x00223ec0,
        block_0x00223ee8,
        block_0x00223eec,
        block_0x00223ef4,
        block_0x00223efc,
        block_0x00223f04,
        block_0x00223f18,
        block_0x00223f40,
        block_0x00223f44,
        block_0x00223f4c,
        block_0x00223f54,
        block_0x00223f5c,
        block_0x00223f6c,
        block_0x00223f70,
        block_0x00223f84,
        block_0x00223f94,
        block_0x00223fa8,
        block_0x00223fb8,
        block_0x00223fcc,
        block_0x00223fdc,
        block_0x00223ff0,
        block_0x00223ff4,
        block_0x00223ffc,
        block_0x0022402c,
        block_0x00224030,
        block_0x00224048,
        block_0x0022404c,
        block_0x00224064,
        block_0x00224068,
        block_0x0022406c,
        block_0x0022407c,
        block_0x002240b0,
        block_0x002240b4,
        block_0x002240c4,
        block_0x002240c8,
        block_0x002240cc,
        block_0x002240dc,
        block_0x002240e0,
        block_0x002240e4,
        block_0x002240f8,
        block_0x002240fc,
        block_0x00224114,
        block_0x00224118,
        block_0x0022411c,
        block_0x0022412c,
        block_0x00224160,
        block_0x00224164,
        block_0x00224174,
        block_0x00224178,
        block_0x0022417c,
        block_0x00224188,
        block_0x0022418c,
        block_0x00224190,
        block_0x002241a4,
        block_0x002241a8,
        block_0x002241c0,
        block_0x002241c4,
        block_0x002241c8,
        block_0x002241d8,
        block_0x0022420c,
        block_0x00224210,
        block_0x00224220,
        block_0x00224224,
        block_0x00224228,
        block_0x00224234,
        block_0x00224238,
        block_0x0022423c,
        block_0x0022424c,
        block_0x00224250,
        block_0x00224268,
        block_0x0022426c,
        block_0x00224270,
        block_0x00224280,
        block_0x002242b4,
        block_0x002242b8,
        block_0x002242c4,
        block_0x002242c8,
        block_0x002242cc,
        block_0x002242d0,
        block_0x002242ec,
        block_0x002242f0,
        block_0x002242f4,
        block_0x0022430c,
        block_0x00224310,
        block_0x00224328,
        block_0x00224338,
        block_0x00224340,
    ];
    const IDX: [u16; 386usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16, 5u16,
        6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 12u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16,
        16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16,
        0u16, 19u16, 20u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 24u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        26u16, 27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 29u16, 30u16, 0u16, 31u16, 0u16, 32u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 36u16, 0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16, 0u16, 40u16,
        41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16,
        0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 53u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 56u16, 57u16,
        0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 0u16, 0u16, 61u16, 62u16, 63u16, 0u16,
        0u16, 0u16, 64u16, 65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16,
        0u16, 0u16, 75u16, 76u16, 77u16, 0u16, 0u16, 78u16, 79u16, 80u16, 0u16, 0u16,
        0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 84u16, 85u16,
        0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 89u16, 90u16, 91u16, 0u16,
        0u16, 92u16, 93u16, 94u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 97u16, 98u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 102u16, 0u16, 0u16,
        103u16, 104u16, 105u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
        108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 111u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 113u16, 0u16, 114u16,
    ];
    if pc < 2243900u32 || pc > 2245440u32 {
        return None;
    }
    let word_offset = ((pc - 2243900u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00223d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1176u32, 2243904u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2243908u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2243912u32);
    emu.apc_no_count(1usize, 2243912u32, 4294860800u32, 2243916u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2243920u32;
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
pub fn block_0x00223d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2243924u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2243928u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2243932u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2243940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d64));
    } else {
        emu.pc = 2243936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d60));
    }
}
#[inline(always)]
pub fn block_0x00223d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2243940u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2243940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223d64));
}
#[inline(always)]
pub fn block_0x00223d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2243944u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2243948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d6c));
    }
}
#[inline(always)]
pub fn block_0x00223d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223dd4));
    } else {
        emu.pc = 2243952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d70));
    }
}
#[inline(always)]
pub fn block_0x00223d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2243956u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2243960u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2243964u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2243968u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2243972u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2243972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223d84));
}
#[inline]
pub fn block_0x00223d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2243976u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2243980u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2243984u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2243988u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2243992u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2243996u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2244000u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2244004u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2244008u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2244012u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2244016u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2243972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223d84));
    } else {
        emu.pc = 2244020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223db4));
    }
}
#[inline(always)]
pub fn block_0x00223db4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223dd4));
    } else {
        emu.pc = 2244024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223db8));
    }
}
#[inline(always)]
pub fn block_0x00223db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2244028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246752u32));
    } else {
        emu.pc = 2244032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223dc0));
    }
}
#[inline(always)]
pub fn block_0x00223dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2244036u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2244040u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2244044u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2244048u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2244052u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2244052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223dd4));
}
#[inline(always)]
pub fn block_0x00223dd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 680u32, 2244056u32)?;
    emu.sw_no_count(10usize, 2usize, 1336u32, 2244060u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2244068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223de4));
    } else {
        emu.pc = 2244064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223de0));
    }
}
#[inline(always)]
pub fn block_0x00223de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2244068u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2244068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223de4));
}
#[inline(always)]
pub fn block_0x00223de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2244072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2244076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223dec));
    }
}
#[inline(always)]
pub fn block_0x00223dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 10usize, 2u32, 2244080u32);
    emu.adi_no_count(13usize, 2usize, 1176u32, 2244084u32);
    emu.adi_no_count(14usize, 2usize, 520u32, 2244088u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2244092u32);
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2244096u32);
    emu.adr_no_count(12usize, 13usize, 15usize, 2244100u32);
    emu.adr_no_count(13usize, 14usize, 15usize, 2244104u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2244104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223e08));
}
#[inline(always)]
pub fn block_0x00223e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f5c));
    } else {
        emu.pc = 2244108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e0c));
    }
}
#[inline(always)]
pub fn block_0x00223e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2244112u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2244116u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2244120u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2244124u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2244128u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2244104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e08));
    } else {
        emu.pc = 2244132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e24));
    }
}
#[inline(always)]
pub fn block_0x00223e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2244136u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2244140u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2244144u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2244148u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2244460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f6c));
    } else {
        emu.pc = 2244152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e38));
    }
}
#[inline(always)]
pub fn block_0x00223e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2244156u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2244160u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2244164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e44));
    }
}
#[inline(always)]
pub fn block_0x00223e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e98));
    } else {
        emu.pc = 2244168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e48));
    }
}
#[inline(always)]
pub fn block_0x00223e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2244172u32);
    emu.sli_no_count(14usize, 10usize, 2u32, 2244176u32);
    emu.adi_no_count(16usize, 2usize, 28u32, 2244180u32);
    emu.adr_no_count(12usize, 16usize, 14usize, 2244184u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2244188u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2244188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223e5c));
}
#[inline]
pub fn block_0x00223e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2244192u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2244196u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2244200u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2244204u32);
    emu.sw_no_count(13usize, 16usize, 0u32, 2244208u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2244212u32);
    emu.sltru_no_count(13usize, 13usize, 17usize, 2244216u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2244220u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2244224u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2244188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e5c));
    } else {
        emu.pc = 2244228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e84));
    }
}
#[inline(always)]
pub fn block_0x00223e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e98));
    } else {
        emu.pc = 2244232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e88));
    }
}
#[inline(always)]
pub fn block_0x00223e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2244236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246752u32));
    } else {
        emu.pc = 2244240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e90));
    }
}
#[inline(always)]
pub fn block_0x00223e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 12usize, 0u32, 2244244u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2244248u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2244248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223e98));
}
#[inline(always)]
pub fn block_0x00223e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 352u32, 2244252u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2244256u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2244260u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2246516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246516u32));
    } else {
        emu.pc = 2244264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ea8));
    }
}
#[inline(always)]
pub fn block_0x00223ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223efc));
    } else {
        emu.pc = 2244268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223eac));
    }
}
#[inline(always)]
pub fn block_0x00223eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2244272u32);
    emu.sli_no_count(14usize, 13usize, 2u32, 2244276u32);
    emu.adi_no_count(16usize, 2usize, 192u32, 2244280u32);
    emu.adr_no_count(10usize, 16usize, 14usize, 2244284u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2244288u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2244288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223ec0));
}
#[inline]
pub fn block_0x00223ec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2244292u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2244296u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2244300u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2244304u32);
    emu.sw_no_count(12usize, 16usize, 0u32, 2244308u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2244312u32);
    emu.sltru_no_count(12usize, 12usize, 17usize, 2244316u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2244320u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2244324u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2244288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ec0));
    } else {
        emu.pc = 2244328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ee8));
    }
}
#[inline(always)]
pub fn block_0x00223ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223efc));
    } else {
        emu.pc = 2244332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223eec));
    }
}
#[inline(always)]
pub fn block_0x00223eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2244336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246752u32));
    } else {
        emu.pc = 2244340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ef4));
    }
}
#[inline(always)]
pub fn block_0x00223ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2244344u32)?;
    emu.adi_no_count(13usize, 13usize, 1u32, 2244348u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2244348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223efc));
}
#[inline(always)]
pub fn block_0x00223efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 352u32, 2244352u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2244436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f54));
    } else {
        emu.pc = 2244356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f04));
    }
}
#[inline(always)]
pub fn block_0x00223f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2244360u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2244364u32);
    emu.adi_no_count(15usize, 2usize, 356u32, 2244368u32);
    emu.adr_no_count(10usize, 15usize, 13usize, 2244372u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2244376u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2244376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223f18));
}
#[inline]
pub fn block_0x00223f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2244380u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2244384u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2244388u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2244392u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2244396u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2244400u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2244404u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2244408u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2244412u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2244376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f18));
    } else {
        emu.pc = 2244416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f40));
    }
}
#[inline(always)]
pub fn block_0x00223f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f54));
    } else {
        emu.pc = 2244420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f44));
    }
}
#[inline(always)]
pub fn block_0x00223f44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2244424u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2246752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246752u32));
    } else {
        emu.pc = 2244428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f4c));
    }
}
#[inline(always)]
pub fn block_0x00223f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2244432u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2244436u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2244436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223f54));
}
#[inline(always)]
pub fn block_0x00223f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2244440u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2244444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2244464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223f70));
}
#[inline(always)]
pub fn block_0x00223f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2244448u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2244452u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2244456u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2244152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223e38));
    } else {
        emu.pc = 2244460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223f6c));
    }
}
#[inline(always)]
pub fn block_0x00223f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2244464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2244464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223f70));
}
#[inline(always)]
pub fn block_0x00223f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2244468u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2244472u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2244476u32);
    emu.apc_no_count(1usize, 2244476u32, 4294860800u32, 2244480u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1376u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223f84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2244488u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2244492u32);
    emu.apc_no_count(1usize, 2244492u32, 4294942720u32, 2244496u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2244504u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2244508u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2244512u32);
    emu.apc_no_count(1usize, 2244512u32, 4294860800u32, 2244516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2244524u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2244528u32);
    emu.apc_no_count(1usize, 2244528u32, 4294942720u32, 2244532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244536u32;
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
#[inline(always)]
pub fn block_0x00223fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2244540u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2244544u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2244548u32);
    emu.apc_no_count(1usize, 2244548u32, 4294860800u32, 2244552u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00223fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2244560u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2244564u32);
    emu.apc_no_count(1usize, 2244564u32, 4294942720u32, 2244568u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2244572u32;
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
pub fn block_0x00223fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 1172u32, 2244576u32)?;
    emu.lw_no_count(21usize, 2usize, 188u32, 2244580u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2244584u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2244588u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2244596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ff4));
    } else {
        emu.pc = 2244592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ff0));
    }
}
#[inline(always)]
pub fn block_0x00223ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2244596u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2244596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00223ff4));
}
#[inline(always)]
pub fn block_0x00223ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2244600u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2244604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00223ffc));
    }
}
#[inline]
pub fn block_0x00223ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 8u32, 2244608u32)?;
    emu.sw_no_count(23usize, 2usize, 4u32, 2244612u32)?;
    emu.adi_no_count(23usize, 0usize, 0u32, 2244616u32);
    emu.lw_no_count(26usize, 2usize, 1008u32, 2244620u32)?;
    emu.lw_no_count(27usize, 2usize, 844u32, 2244624u32)?;
    emu.lw_no_count(18usize, 2usize, 680u32, 2244628u32)?;
    emu.adi_no_count(22usize, 2usize, 24u32, 2244632u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2244636u32);
    emu.adi_no_count(8usize, 0usize, 41u32, 2244640u32);
    emu.adi_no_count(9usize, 0usize, 10u32, 2244644u32);
    emu.sw_no_count(24usize, 2usize, 16u32, 2244648u32)?;
    emu.add_memory_rw_events(12usize);
    let return_addr = 2244652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2244656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224030));
}
#[inline(always)]
pub fn block_0x0022402c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2246420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2244656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224030));
    }
}
#[inline(always)]
pub fn block_0x00224030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 23usize, 0u32, 2244660u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2244664u32);
    emu.sbr_no_count(11usize, 0usize, 12usize, 2244668u32);
    emu.adi_no_count(13usize, 2usize, 1008u32, 2244672u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2244676u32);
    emu.adr_no_count(14usize, 22usize, 12usize, 2244680u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2244680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224048));
}
#[inline(always)]
pub fn block_0x00224048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240c8));
    } else {
        emu.pc = 2244684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022404c));
    }
}
#[inline(always)]
pub fn block_0x0022404c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2244688u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2244692u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2244696u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2244700u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2244704u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2244680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224048));
    } else {
        emu.pc = 2244708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224064));
    }
}
#[inline(always)]
pub fn block_0x00224064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2244812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240cc));
    } else {
        emu.pc = 2244712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224068));
    }
}
#[inline(always)]
pub fn block_0x00224068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240b4));
    } else {
        emu.pc = 2244716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022406c));
    }
}
#[inline(always)]
pub fn block_0x0022406c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1012u32, 2244720u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2244724u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2244728u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2244732u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2244732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022407c));
}
#[inline]
pub fn block_0x0022407c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 0u32, 2244736u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2244740u32)?;
    emu.ani_no_count(14usize, 14usize, 1u32, 2244744u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2244748u32);
    emu.xri_no_count(15usize, 15usize, 4294967295u32, 2244752u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2244756u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2244760u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2244764u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2244768u32);
    emu.sw_no_count(14usize, 13usize, 0u32, 2244772u32)?;
    emu.orr_no_count(14usize, 16usize, 15usize, 2244776u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2244780u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2244732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022407c));
    } else {
        emu.pc = 2244784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240b0));
    }
}
#[inline(always)]
pub fn block_0x002240b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2246488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246488u32));
    } else {
        emu.pc = 2244788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240b4));
    }
}
#[inline(always)]
pub fn block_0x002240b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 188u32, 2244792u32)?;
    emu.adi_no_count(11usize, 0usize, 8u32, 2244796u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2244800u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2244828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240dc));
    } else {
        emu.pc = 2244804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240c4));
    }
}
#[inline(always)]
pub fn block_0x002240c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2244808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2244832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002240e0));
}
#[inline(always)]
pub fn block_0x002240c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224068));
    } else {
        emu.pc = 2244812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240cc));
    }
}
#[inline(always)]
pub fn block_0x002240cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2244816u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2244820u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2244824u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2244832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240e0));
    } else {
        emu.pc = 2244828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240dc));
    }
}
#[inline(always)]
pub fn block_0x002240dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2244832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2244832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002240e0));
}
#[inline(always)]
pub fn block_0x002240e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2246516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246516u32));
    } else {
        emu.pc = 2244836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240e4));
    }
}
#[inline(always)]
pub fn block_0x002240e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 13usize, 2u32, 2244840u32);
    emu.sbr_no_count(14usize, 0usize, 12usize, 2244844u32);
    emu.adi_no_count(15usize, 2usize, 844u32, 2244848u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2244852u32);
    emu.adr_no_count(16usize, 22usize, 12usize, 2244856u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2244856u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002240f8));
}
#[inline(always)]
pub fn block_0x002240f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2244984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224178));
    } else {
        emu.pc = 2244860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240fc));
    }
}
#[inline(always)]
pub fn block_0x002240fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2244864u32)?;
    emu.lw_no_count(5usize, 15usize, 0u32, 2244868u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2244872u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2244876u32);
    emu.adi_no_count(16usize, 16usize, 4294967292u32, 2244880u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2244856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002240f8));
    } else {
        emu.pc = 2244884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224114));
    }
}
#[inline(always)]
pub fn block_0x00224114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2244988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022417c));
    } else {
        emu.pc = 2244888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224118));
    }
}
#[inline(always)]
pub fn block_0x00224118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2244964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224164));
    } else {
        emu.pc = 2244892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022411c));
    }
}
#[inline(always)]
pub fn block_0x0022411c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2244896u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2244900u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2244904u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2244908u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2244908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022412c));
}
#[inline]
pub fn block_0x0022412c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 10usize, 0u32, 2244912u32)?;
    emu.lw_no_count(17usize, 14usize, 0u32, 2244916u32)?;
    emu.ani_no_count(15usize, 15usize, 1u32, 2244920u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2244924u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2244928u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2244932u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2244936u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2244940u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2244944u32);
    emu.sw_no_count(15usize, 14usize, 0u32, 2244948u32)?;
    emu.orr_no_count(15usize, 17usize, 16usize, 2244952u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2244956u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2244908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022412c));
    } else {
        emu.pc = 2244960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224160));
    }
}
#[inline(always)]
pub fn block_0x00224160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2246488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246488u32));
    } else {
        emu.pc = 2244964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224164));
    }
}
#[inline(always)]
pub fn block_0x00224164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 188u32, 2244968u32)?;
    emu.ori_no_count(11usize, 11usize, 4u32, 2244972u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2244976u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2245004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022418c));
    } else {
        emu.pc = 2244980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224174));
    }
}
#[inline(always)]
pub fn block_0x00224174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2244984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245000u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224188));
}
#[inline(always)]
pub fn block_0x00224178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2244888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224118));
    } else {
        emu.pc = 2244988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022417c));
    }
}
#[inline(always)]
pub fn block_0x0022417c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2244992u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2244996u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2245004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022418c));
    } else {
        emu.pc = 2245000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224188));
    }
}
#[inline(always)]
pub fn block_0x00224188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2245004u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022418c));
}
#[inline(always)]
pub fn block_0x0022418c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2245008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224190));
    }
}
#[inline(always)]
pub fn block_0x00224190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 10usize, 2u32, 2245012u32);
    emu.sbr_no_count(12usize, 0usize, 14usize, 2245016u32);
    emu.adi_no_count(15usize, 2usize, 680u32, 2245020u32);
    emu.adr_no_count(15usize, 15usize, 14usize, 2245024u32);
    emu.adr_no_count(16usize, 22usize, 14usize, 2245028u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2245028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002241a4));
}
#[inline(always)]
pub fn block_0x002241a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224224));
    } else {
        emu.pc = 2245032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241a8));
    }
}
#[inline(always)]
pub fn block_0x002241a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2245036u32)?;
    emu.lw_no_count(5usize, 15usize, 0u32, 2245040u32)?;
    emu.adi_no_count(12usize, 12usize, 4u32, 2245044u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2245048u32);
    emu.adi_no_count(16usize, 16usize, 4294967292u32, 2245052u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2245028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241a4));
    } else {
        emu.pc = 2245056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241c0));
    }
}
#[inline(always)]
pub fn block_0x002241c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2245160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224228));
    } else {
        emu.pc = 2245060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241c4));
    }
}
#[inline(always)]
pub fn block_0x002241c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224210));
    } else {
        emu.pc = 2245064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241c8));
    }
}
#[inline(always)]
pub fn block_0x002241c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 684u32, 2245068u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2245072u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2245076u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2245080u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2245080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002241d8));
}
#[inline]
pub fn block_0x002241d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2245084u32)?;
    emu.lw_no_count(17usize, 13usize, 0u32, 2245088u32)?;
    emu.ani_no_count(15usize, 15usize, 1u32, 2245092u32);
    emu.adi_no_count(12usize, 12usize, 4u32, 2245096u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2245100u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2245104u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2245108u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2245112u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2245116u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2245120u32)?;
    emu.orr_no_count(15usize, 17usize, 16usize, 2245124u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2245128u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2245080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241d8));
    } else {
        emu.pc = 2245132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022420c));
    }
}
#[inline(always)]
pub fn block_0x0022420c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2246488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246488u32));
    } else {
        emu.pc = 2245136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224210));
    }
}
#[inline(always)]
pub fn block_0x00224210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 188u32, 2245140u32)?;
    emu.adi_no_count(11usize, 11usize, 2u32, 2245144u32);
    emu.adi_no_count(21usize, 18usize, 0u32, 2245148u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2245176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224238));
    } else {
        emu.pc = 2245152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224220));
    }
}
#[inline(always)]
pub fn block_0x00224220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2245156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224234));
}
#[inline(always)]
pub fn block_0x00224224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002241c4));
    } else {
        emu.pc = 2245160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224228));
    }
}
#[inline(always)]
pub fn block_0x00224228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2245164u32);
    emu.adi_no_count(21usize, 18usize, 0u32, 2245168u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2245176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224238));
    } else {
        emu.pc = 2245172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224234));
    }
}
#[inline(always)]
pub fn block_0x00224234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2245176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224238));
}
#[inline(always)]
pub fn block_0x00224238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2246540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246540u32));
    } else {
        emu.pc = 2245180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022423c));
    }
}
#[inline(always)]
pub fn block_0x0022423c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 21usize, 2u32, 2245184u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2245188u32);
    emu.adr_no_count(14usize, 7usize, 12usize, 2245192u32);
    emu.adr_no_count(15usize, 22usize, 12usize, 2245196u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2245196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022424c));
}
#[inline(always)]
pub fn block_0x0022424c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242c4));
    } else {
        emu.pc = 2245200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224250));
    }
}
#[inline(always)]
pub fn block_0x00224250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2245204u32)?;
    emu.lw_no_count(17usize, 14usize, 0u32, 2245208u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2245212u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2245216u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2245220u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2245196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022424c));
    } else {
        emu.pc = 2245224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224268));
    }
}
#[inline(always)]
pub fn block_0x00224268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2245320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242c8));
    } else {
        emu.pc = 2245228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022426c));
    }
}
#[inline(always)]
pub fn block_0x0022426c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242b8));
    } else {
        emu.pc = 2245232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224270));
    }
}
#[inline(always)]
pub fn block_0x00224270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 520u32, 2245236u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2245240u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2245244u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2245248u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2245248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224280));
}
#[inline]
pub fn block_0x00224280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 10usize, 0u32, 2245252u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2245256u32)?;
    emu.ani_no_count(14usize, 14usize, 1u32, 2245260u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2245264u32);
    emu.xri_no_count(15usize, 15usize, 4294967295u32, 2245268u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2245272u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2245276u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2245280u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2245284u32);
    emu.sw_no_count(14usize, 13usize, 0u32, 2245288u32)?;
    emu.orr_no_count(14usize, 16usize, 15usize, 2245292u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2245296u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2245248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224280));
    } else {
        emu.pc = 2245300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242b4));
    }
}
#[inline(always)]
pub fn block_0x002242b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2246488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246488u32));
    } else {
        emu.pc = 2245304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242b8));
    }
}
#[inline(always)]
pub fn block_0x002242b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 188u32, 2245308u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2245312u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2245316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002242cc));
}
#[inline(always)]
pub fn block_0x002242c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022426c));
    } else {
        emu.pc = 2245320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242c8));
    }
}
#[inline(always)]
pub fn block_0x002242c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2245324u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002242cc));
}
#[inline(always)]
pub fn block_0x002242cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2246776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2246776u32));
    } else {
        emu.pc = 2245328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242d0));
    }
}
#[inline(always)]
pub fn block_0x002242d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 352u32, 2245332u32)?;
    emu.adi_no_count(10usize, 11usize, 48u32, 2245336u32);
    emu.lw_no_count(19usize, 2usize, 20u32, 2245340u32)?;
    emu.adr_no_count(19usize, 19usize, 20usize, 2245344u32);
    emu.sb_no_count(10usize, 19usize, 0u32, 2245348u32);
    emu.adi_no_count(10usize, 25usize, 0u32, 2245352u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2245360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242f0));
    } else {
        emu.pc = 2245356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242ec));
    }
}
#[inline(always)]
pub fn block_0x002242ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2245360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2245360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002242f0));
}
#[inline(always)]
pub fn block_0x002242f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        return Ok(crate::NextStep::Dynamic(2246420u32));
    } else {
        emu.pc = 2245364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002242f4));
    }
}
#[inline(always)]
pub fn block_0x002242f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 20usize, 1u32, 2245368u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2245372u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2245376u32);
    emu.adi_no_count(11usize, 2usize, 188u32, 2245380u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2245384u32);
    emu.adr_no_count(12usize, 22usize, 12usize, 2245388u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2245388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022430c));
}
#[inline(always)]
pub fn block_0x0022430c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2245432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224338));
    } else {
        emu.pc = 2245392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224310));
    }
}
#[inline(always)]
pub fn block_0x00224310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2245396u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2245400u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2245404u32);
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2245408u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2245412u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2245388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022430c));
    } else {
        emu.pc = 2245416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00224328));
    }
}
#[inline(always)]
pub fn block_0x00224328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 14usize, 2245420u32);
    emu.sltru_no_count(11usize, 14usize, 13usize, 2245424u32);
    emu.sbr_no_count(24usize, 11usize, 10usize, 2245428u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2245432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2245440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224340));
}
#[inline(always)]
pub fn block_0x00224338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(24usize, 10usize, 1u32, 2245436u32);
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2245440u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2245440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00224340));
}
#[inline(always)]
pub fn block_0x00224340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1176u32, 2245444u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2245448u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2245452u32);
    emu.apc_no_count(1usize, 2245452u32, 4294860800u32, 2245456u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2245460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
