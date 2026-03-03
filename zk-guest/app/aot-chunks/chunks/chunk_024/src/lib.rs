pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2209148u32;
pub const PC_MAX: u32 = 2211216u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0021b57c,
        block_0x0021b58c,
        block_0x0021b594,
        block_0x0021b5a4,
        block_0x0021b5bc,
        block_0x0021b5dc,
        block_0x0021b5f0,
        block_0x0021b618,
        block_0x0021b624,
        block_0x0021b658,
        block_0x0021b660,
        block_0x0021b664,
        block_0x0021b670,
        block_0x0021b674,
        block_0x0021b698,
        block_0x0021b718,
        block_0x0021b720,
        block_0x0021b72c,
        block_0x0021b744,
        block_0x0021b754,
        block_0x0021b75c,
        block_0x0021b760,
        block_0x0021b764,
        block_0x0021b778,
        block_0x0021b780,
        block_0x0021b788,
        block_0x0021b798,
        block_0x0021b7ac,
        block_0x0021b800,
        block_0x0021b80c,
        block_0x0021b810,
        block_0x0021b820,
        block_0x0021b828,
        block_0x0021b830,
        block_0x0021b840,
        block_0x0021b854,
        block_0x0021b884,
        block_0x0021b89c,
        block_0x0021b8b0,
        block_0x0021b8c0,
        block_0x0021b8c8,
        block_0x0021b8d0,
        block_0x0021b8e8,
        block_0x0021b8f4,
        block_0x0021b900,
        block_0x0021b938,
        block_0x0021b964,
        block_0x0021b968,
        block_0x0021b96c,
        block_0x0021b97c,
        block_0x0021b980,
        block_0x0021b990,
        block_0x0021b9ac,
        block_0x0021b9b4,
        block_0x0021b9d8,
        block_0x0021b9ec,
        block_0x0021ba04,
        block_0x0021ba14,
        block_0x0021ba1c,
        block_0x0021ba20,
        block_0x0021ba34,
        block_0x0021ba40,
        block_0x0021ba48,
        block_0x0021ba4c,
        block_0x0021ba5c,
        block_0x0021ba78,
        block_0x0021ba80,
        block_0x0021baa0,
        block_0x0021bab8,
        block_0x0021bad4,
        block_0x0021bae4,
        block_0x0021baec,
        block_0x0021bb00,
        block_0x0021bb04,
        block_0x0021bb14,
        block_0x0021bb18,
        block_0x0021bb20,
        block_0x0021bb30,
        block_0x0021bb34,
        block_0x0021bb40,
        block_0x0021bb44,
        block_0x0021bb48,
        block_0x0021bb64,
        block_0x0021bb6c,
        block_0x0021bb70,
        block_0x0021bb74,
        block_0x0021bb80,
        block_0x0021bb88,
        block_0x0021bb9c,
        block_0x0021bbb0,
        block_0x0021bbb8,
        block_0x0021bbc8,
        block_0x0021bbcc,
        block_0x0021bbfc,
        block_0x0021bc10,
        block_0x0021bc38,
        block_0x0021bc50,
        block_0x0021bc5c,
        block_0x0021bc88,
        block_0x0021bc90,
        block_0x0021bc94,
        block_0x0021bc9c,
        block_0x0021bca8,
        block_0x0021bcb8,
        block_0x0021bcc8,
        block_0x0021bcd0,
        block_0x0021bce8,
        block_0x0021bd04,
        block_0x0021bd08,
        block_0x0021bd24,
        block_0x0021bd2c,
        block_0x0021bd58,
        block_0x0021bd90,
    ];
    const IDX: [u16; 518usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        8u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 0u16, 0u16, 13u16, 14u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        16u16, 0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16,
        0u16, 0u16, 20u16, 0u16, 21u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16,
        0u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 28u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 30u16, 31u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 49u16, 0u16, 0u16, 0u16, 50u16,
        51u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16,
        59u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 62u16, 0u16, 63u16,
        64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 75u16,
        76u16, 0u16, 77u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 80u16, 81u16,
        82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 85u16, 86u16,
        0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16, 93u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 97u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 99u16, 0u16, 100u16, 101u16, 0u16, 102u16, 0u16, 0u16, 103u16,
        0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 109u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16,
    ];
    if pc < 2209148u32 || pc > 2211216u32 {
        return None;
    }
    let word_offset = ((pc - 2209148u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0021b57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209152u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 256u32, 2209156u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2209160u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2209188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5a4));
    } else {
        emu.pc = 2209164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b58c));
    }
}
#[inline(always)]
pub fn block_0x0021b58c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 8u32, 2209168u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5f0));
    } else {
        emu.pc = 2209172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b594));
    }
}
#[inline(always)]
pub fn block_0x0021b594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 12u32, 2209176u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2209180u32);
    emu.apc_no_count(6usize, 2209180u32, 0u32, 2209184u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2209188u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021b5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209192u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 256u32, 2209196u32);
    emu.lw_no_count(11usize, 12usize, 8u32, 2209200u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2209204u32)?;
    emu.orr_no_count(13usize, 11usize, 12usize, 2209208u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2209304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b618));
    } else {
        emu.pc = 2209212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5bc));
    }
}
#[inline(always)]
pub fn block_0x0021b5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2209216u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2209220u32;
    emu.update_insn_clock();
    emu.lw_no_count(14usize, 14usize, 252u32, 2209224u32)?;
    emu.lw_no_count(13usize, 13usize, 248u32, 2209228u32)?;
    emu.xrr_no_count(12usize, 14usize, 12usize, 2209232u32);
    emu.xrr_no_count(11usize, 13usize, 11usize, 2209236u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2209240u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2209304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b618));
    } else {
        emu.pc = 2209244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5dc));
    }
}
#[inline(always)]
pub fn block_0x0021b5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209248u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 604u32, 2209252u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2209256u32);
    emu.apc_no_count(6usize, 2209256u32, 0u32, 2209260u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2209264u32;
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
#[inline]
pub fn block_0x0021b5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2209268u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2209272u32)?;
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2209276u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 256u32, 2209280u32);
    emu.lw_no_count(14usize, 13usize, 12u32, 2209284u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2209288u32)?;
    emu.xrr_no_count(12usize, 12usize, 14usize, 2209292u32);
    emu.xrr_no_count(11usize, 11usize, 13usize, 2209296u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2209300u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b5dc));
    } else {
        emu.pc = 2209304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b618));
    }
}
#[inline(always)]
pub fn block_0x0021b618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2209308u32);
    emu.apc_no_count(6usize, 2209308u32, 0u32, 2209312u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2209316u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2209320u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2209324u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2209328u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2209332u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2209336u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2209340u32)?;
    emu.sw_no_count(20usize, 2usize, 616u32, 2209344u32)?;
    emu.sw_no_count(21usize, 2usize, 612u32, 2209348u32)?;
    emu.sw_no_count(22usize, 2usize, 608u32, 2209352u32)?;
    emu.sw_no_count(23usize, 2usize, 604u32, 2209356u32)?;
    emu.sw_no_count(24usize, 2usize, 600u32, 2209360u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2209364u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b664));
    } else {
        emu.pc = 2209368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b658));
    }
}
#[inline(always)]
pub fn block_0x0021b658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2209372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b670));
    } else {
        emu.pc = 2209376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b660));
    }
}
#[inline(always)]
pub fn block_0x0021b660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b674));
}
#[inline(always)]
pub fn block_0x0021b664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2209384u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966836u32, 2209388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2209396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b674));
    } else {
        emu.pc = 2209392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b670));
    }
}
#[inline(always)]
pub fn block_0x0021b670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2209396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2209396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b674));
}
#[inline]
pub fn block_0x0021b674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 8u32, 2209400u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2209404u32)?;
    emu.adi_no_count(9usize, 2usize, 16u32, 2209408u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2209412u32);
    emu.adi_no_count(12usize, 0usize, 512u32, 2209416u32);
    emu.adi_no_count(18usize, 0usize, 512u32, 2209420u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2209424u32);
    emu.apc_no_count(1usize, 2209424u32, 4294897664u32, 2209428u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021b698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 528u32, 2209436u32)?;
    emu.sw_no_count(18usize, 2usize, 532u32, 2209440u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2209444u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2209448u32)?;
    emu.lw_no_count(21usize, 8usize, 0u32, 2209452u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2209456u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2209460u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2209464u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 940u32, 2209468u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2209472u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1636u32, 2209476u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2209480u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294966892u32, 2209484u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2209488u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2209492u32)?;
    emu.adi_no_count(11usize, 2usize, 576u32, 2209496u32);
    emu.adi_no_count(9usize, 0usize, 3u32, 2209500u32);
    emu.sw_no_count(10usize, 2usize, 576u32, 2209504u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2209508u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2209512u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2209516u32)?;
    emu.sw_no_count(19usize, 2usize, 592u32, 2209520u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2209524u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2209528u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2209532u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2209536u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2209540u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2209544u32);
    emu.adi_no_count(11usize, 2usize, 528u32, 2209548u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2209552u32);
    emu.apc_no_count(1usize, 2209552u32, 4294959104u32, 2209556u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2209564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2209632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b760));
    } else {
        emu.pc = 2209568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b720));
    }
}
#[inline(always)]
pub fn block_0x0021b720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 536u32, 2209572u32)?;
    emu.adi_no_count(10usize, 0usize, 513u32, 2209576u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2209924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b884));
    } else {
        emu.pc = 2209580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b72c));
    }
}
#[inline(always)]
pub fn block_0x0021b72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2209584u32)?;
    emu.lw_no_count(11usize, 8usize, 8u32, 2209588u32)?;
    emu.lw_no_count(14usize, 10usize, 28u32, 2209592u32)?;
    emu.adi_no_count(10usize, 2usize, 576u32, 2209596u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2209600u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2209604u32;
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
pub fn block_0x0021b744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 576u32, 2209608u32);
    emu.lw_no_count(8usize, 2usize, 580u32, 2209612u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2209616u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2209808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b810));
    } else {
        emu.pc = 2209620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b754));
    }
}
#[inline(always)]
pub fn block_0x0021b754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2209624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2209876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b854));
    } else {
        emu.pc = 2209628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b75c));
    }
}
#[inline(always)]
pub fn block_0x0021b75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2209632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2209808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b810));
}
#[inline(always)]
pub fn block_0x0021b760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2209708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b7ac));
    } else {
        emu.pc = 2209636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b764));
    }
}
#[inline(always)]
pub fn block_0x0021b764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 548u32, 2209640u32)?;
    emu.lw_no_count(24usize, 9usize, 4u32, 2209644u32)?;
    emu.lw_no_count(11usize, 24usize, 0u32, 2209648u32)?;
    emu.lw_no_count(18usize, 9usize, 0u32, 2209652u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b780));
    } else {
        emu.pc = 2209656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b778));
    }
}
#[inline(always)]
pub fn block_0x0021b778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2209660u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2209664u32;
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
pub fn block_0x0021b780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 24usize, 4u32, 2209668u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b798));
    } else {
        emu.pc = 2209672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b788));
    }
}
#[inline(always)]
pub fn block_0x0021b788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 24usize, 8u32, 2209676u32)?;
    emu.adi_no_count(10usize, 18usize, 0u32, 2209680u32);
    emu.apc_no_count(1usize, 2209680u32, 4294864896u32, 2209684u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2209692u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2209696u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2209700u32);
    emu.apc_no_count(1usize, 2209700u32, 4294860800u32, 2209704u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 8u32, 2209712u32)?;
    emu.lw_no_count(10usize, 8usize, 12u32, 2209716u32)?;
    emu.adi_no_count(12usize, 2usize, 8u32, 2209720u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2209724u32);
    emu.sw_no_count(0usize, 2usize, 568u32, 2209728u32)?;
    emu.adi_no_count(13usize, 2usize, 576u32, 2209732u32);
    emu.lw_no_count(14usize, 10usize, 36u32, 2209736u32)?;
    emu.sw_no_count(12usize, 2usize, 576u32, 2209740u32)?;
    emu.sw_no_count(20usize, 2usize, 580u32, 2209744u32)?;
    emu.sw_no_count(21usize, 2usize, 584u32, 2209748u32)?;
    emu.sw_no_count(23usize, 2usize, 588u32, 2209752u32)?;
    emu.adi_no_count(9usize, 0usize, 3u32, 2209756u32);
    emu.sw_no_count(19usize, 2usize, 592u32, 2209760u32)?;
    emu.sw_no_count(20usize, 2usize, 596u32, 2209764u32)?;
    emu.sw_no_count(22usize, 2usize, 552u32, 2209768u32)?;
    emu.sw_no_count(18usize, 2usize, 556u32, 2209772u32)?;
    emu.sw_no_count(13usize, 2usize, 560u32, 2209776u32)?;
    emu.sw_no_count(9usize, 2usize, 564u32, 2209780u32)?;
    emu.adi_no_count(10usize, 2usize, 544u32, 2209784u32);
    emu.adi_no_count(12usize, 2usize, 552u32, 2209788u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2209792u32;
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
pub fn block_0x0021b800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 544u32, 2209796u32);
    emu.lw_no_count(8usize, 2usize, 548u32, 2209800u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2209808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b810));
    } else {
        emu.pc = 2209804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b80c));
    }
}
#[inline(always)]
pub fn block_0x0021b80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2209876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b854));
    } else {
        emu.pc = 2209808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b810));
    }
}
#[inline(always)]
pub fn block_0x0021b810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 8usize, 4u32, 2209812u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2209816u32)?;
    emu.lw_no_count(9usize, 8usize, 0u32, 2209820u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b828));
    } else {
        emu.pc = 2209824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b820));
    }
}
#[inline(always)]
pub fn block_0x0021b820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2209828u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2209832u32;
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
pub fn block_0x0021b828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2209836u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2209856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b840));
    } else {
        emu.pc = 2209840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b830));
    }
}
#[inline(always)]
pub fn block_0x0021b830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2209844u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2209848u32);
    emu.apc_no_count(1usize, 2209848u32, 4294860800u32, 2209852u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209856u32;
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
pub fn block_0x0021b840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2209860u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2209864u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2209868u32);
    emu.apc_no_count(1usize, 2209868u32, 4294860800u32, 2209872u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 636u32, 2209880u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2209884u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2209888u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2209892u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2209896u32)?;
    emu.lw_no_count(20usize, 2usize, 616u32, 2209900u32)?;
    emu.lw_no_count(21usize, 2usize, 612u32, 2209904u32)?;
    emu.lw_no_count(22usize, 2usize, 608u32, 2209908u32)?;
    emu.lw_no_count(23usize, 2usize, 604u32, 2209912u32)?;
    emu.lw_no_count(24usize, 2usize, 600u32, 2209916u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2209920u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209924u32;
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
pub fn block_0x0021b884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2209928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966848u32, 2209932u32);
    emu.adi_no_count(11usize, 0usize, 512u32, 2209936u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2209940u32);
    emu.apc_no_count(1usize, 2209940u32, 40960u32, 2209944u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209952u32;
    emu.update_insn_clock();
    emu.lw_no_count(12usize, 11usize, 272u32, 2209956u32)?;
    emu.adi_no_count(13usize, 12usize, 1u32, 2209960u32);
    emu.sw_no_count(13usize, 11usize, 272u32, 2209964u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2209992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8c8));
    } else {
        emu.pc = 2209968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8b0));
    }
}
#[inline(always)]
pub fn block_0x0021b8b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2209972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 272u32, 2209976u32);
    emu.lbu_no_count(12usize, 11usize, 8u32, 2209980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2210000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8d0));
    } else {
        emu.pc = 2209984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b8c0));
    }
}
#[inline(always)]
pub fn block_0x0021b8c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2209988u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2209992u32;
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
pub fn block_0x0021b8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2209996u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210000u32;
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
pub fn block_0x0021b8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 4u32, 2210004u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2210008u32);
    emu.sw_no_count(12usize, 11usize, 4u32, 2210012u32)?;
    emu.sb_no_count(10usize, 11usize, 8u32, 2210016u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2210020u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210024u32;
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
pub fn block_0x0021b8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210028u32;
    emu.update_insn_clock();
    emu.sb_no_count(0usize, 10usize, 280u32, 2210032u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210036u32;
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
pub fn block_0x0021b8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210040u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 276u32, 2210044u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210048u32;
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
pub fn block_0x0021b900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2210052u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2210056u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2210060u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966944u32, 2210064u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2210068u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2210072u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2210076u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2210080u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2210084u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2210088u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2210092u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2210096u32);
    emu.apc_no_count(1usize, 2210096u32, 8192u32, 2210100u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021b938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2210108u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2210112u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2210116u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2210120u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2210124u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2210128u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2210132u32)?;
    emu.sli_no_count(18usize, 10usize, 1u32, 2210136u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2210140u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2210144u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2210152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b968));
    } else {
        emu.pc = 2210148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b964));
    }
}
#[inline(always)]
pub fn block_0x0021b964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 8u32, 2210152u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b968));
}
#[inline(always)]
pub fn block_0x0021b968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2210172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b97c));
    } else {
        emu.pc = 2210156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b96c));
    }
}
#[inline(always)]
pub fn block_0x0021b96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2210160u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2210164u32);
    emu.apc_no_count(1usize, 2210164u32, 0u32, 2210168u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b990));
    } else {
        emu.pc = 2210176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b980));
    }
}
#[inline(always)]
pub fn block_0x0021b980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 4u32, 2210180u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2210184u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2210188u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2210192u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2210192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021b990));
}
#[inline(always)]
pub fn block_0x0021b990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2210196u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2210200u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2210204u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2210208u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2210212u32);
    emu.apc_no_count(1usize, 2210212u32, 0u32, 2210216u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021b9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2210224u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b9d8));
    } else {
        emu.pc = 2210228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021b9b4));
    }
}
#[inline]
pub fn block_0x0021b9b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2210232u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2210236u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2210240u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2210244u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2210248u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2210252u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2210256u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2210260u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210264u32;
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
pub fn block_0x0021b9d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2210268u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2210272u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2210276u32);
    emu.apc_no_count(1usize, 2210276u32, 0u32, 2210280u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210284u32;
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
pub fn block_0x0021b9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2210288u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2210292u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2210296u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2210300u32)?;
    emu.adr_no_count(9usize, 11usize, 12usize, 2210304u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2210336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba20));
    } else {
        emu.pc = 2210308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba04));
    }
}
#[inline(always)]
pub fn block_0x0021ba04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2210312u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2210316u32)?;
    emu.sli_no_count(11usize, 10usize, 1u32, 2210320u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2210356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba34));
    } else {
        emu.pc = 2210324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba14));
    }
}
#[inline(always)]
pub fn block_0x0021ba14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2210328u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2210368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba40));
    } else {
        emu.pc = 2210332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba1c));
    }
}
#[inline(always)]
pub fn block_0x0021ba1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2210376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba48));
    } else {
        emu.pc = 2210336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba20));
    }
}
#[inline(always)]
pub fn block_0x0021ba20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2210340u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210344u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966952u32, 2210348u32);
    emu.apc_no_count(1usize, 2210348u32, 0u32, 2210352u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ba34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2210360u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2210364u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2210332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba1c));
    } else {
        emu.pc = 2210368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba40));
    }
}
#[inline(always)]
pub fn block_0x0021ba40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 8u32, 2210372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2210336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba20));
    } else {
        emu.pc = 2210376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba48));
    }
}
#[inline(always)]
pub fn block_0x0021ba48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba5c));
    } else {
        emu.pc = 2210380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba4c));
    }
}
#[inline(always)]
pub fn block_0x0021ba4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2210384u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2210388u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2210392u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2210396u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2210396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ba5c));
}
#[inline(always)]
pub fn block_0x0021ba5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2210400u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2210404u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2210408u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2210412u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2210416u32);
    emu.apc_no_count(1usize, 2210416u32, 0u32, 2210420u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(72u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ba78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2210428u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2210464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baa0));
    } else {
        emu.pc = 2210432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ba80));
    }
}
#[inline(always)]
pub fn block_0x0021ba80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2210436u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2210440u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2210444u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2210448u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2210452u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2210456u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2210460u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210464u32;
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
pub fn block_0x0021baa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2210468u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2210472u32)?;
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2210476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966952u32, 2210480u32);
    emu.apc_no_count(1usize, 2210480u32, 0u32, 2210484u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210488u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0021bab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2210492u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2210496u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2210500u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2210504u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2210508u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2210512u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2210624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb40));
    } else {
        emu.pc = 2210516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bad4));
    }
}
#[inline(always)]
pub fn block_0x0021bad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2210520u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2210524u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2210528u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2210580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb14));
    } else {
        emu.pc = 2210532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bae4));
    }
}
#[inline(always)]
pub fn block_0x0021bae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2210536u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2210580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb14));
    } else {
        emu.pc = 2210540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021baec));
    }
}
#[inline(always)]
pub fn block_0x0021baec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2210544u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2210548u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2210552u32);
    emu.apc_no_count(1usize, 2210552u32, 4294860800u32, 2210556u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb34));
    } else {
        emu.pc = 2210564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb04));
    }
}
#[inline(always)]
pub fn block_0x0021bb04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2210568u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2210572u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2210576u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2210580u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb48));
}
#[inline(always)]
pub fn block_0x0021bb14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb64));
    } else {
        emu.pc = 2210584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb18));
    }
}
#[inline(always)]
pub fn block_0x0021bb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2210584u32, 4294893568u32, 2210588u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bb20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2210596u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2210600u32);
    emu.apc_no_count(1usize, 2210600u32, 4294860800u32, 2210604u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bb30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb04));
    } else {
        emu.pc = 2210612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb34));
    }
}
#[inline(always)]
pub fn block_0x0021bb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2210616u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2210620u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2210624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb44));
}
#[inline(always)]
pub fn block_0x0021bb40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2210628u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb44));
}
#[inline(always)]
pub fn block_0x0021bb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2210632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2210632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb48));
}
#[inline(always)]
pub fn block_0x0021bb48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2210636u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2210640u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2210644u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2210648u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2210652u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2210656u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210660u32;
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
pub fn block_0x0021bb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2210664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2210564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb04));
    } else {
        emu.pc = 2210668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb6c));
    }
}
#[inline(always)]
pub fn block_0x0021bb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2210672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bb34));
}
#[inline(always)]
pub fn block_0x0021bb70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb80));
    } else {
        emu.pc = 2210676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bb74));
    }
}
#[inline(always)]
pub fn block_0x0021bb74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2210680u32);
    emu.apc_no_count(1usize, 2210680u32, 0u32, 2210684u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210688u32;
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
pub fn block_0x0021bb80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2210688u32, 0u32, 2210692u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210696u32;
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
pub fn block_0x0021bb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2210700u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2210704u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2210708u32);
    emu.apc_no_count(1usize, 2210708u32, 4294893568u32, 2210712u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210716u32;
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
pub fn block_0x0021bb9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2210720u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2210724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2210728u32);
    emu.apc_no_count(6usize, 2210728u32, 24576u32, 2210732u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2210736u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965660u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bbb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2210740u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2210760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbc8));
    } else {
        emu.pc = 2210744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbb8));
    }
}
#[inline(always)]
pub fn block_0x0021bbb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2210748u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2210752u32);
    emu.apc_no_count(6usize, 2210752u32, 4294860800u32, 2210756u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2210760u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bbc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210764u32;
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
pub fn block_0x0021bbcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2210768u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2210772u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2210776u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2210780u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2210784u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2210788u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2210792u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2210796u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2210800u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2210804u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2210808u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2210872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc38));
    } else {
        emu.pc = 2210812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bbfc));
    }
}
#[inline(always)]
pub fn block_0x0021bbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2210816u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2210820u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2210824u32);
    emu.apc_no_count(1usize, 2210824u32, 4294897664u32, 2210828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021bc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2210836u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2210840u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2210844u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2210848u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2210852u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2210856u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2210860u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2210864u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2210868u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210872u32;
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
pub fn block_0x0021bc38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2210876u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2210880u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2210884u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2210888u32);
    emu.apc_no_count(1usize, 2210888u32, 0u32, 2210892u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2210896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bc50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2210900u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2210904u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2210908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210812u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bbfc));
}
#[inline]
pub fn block_0x0021bc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2210912u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2210916u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2210920u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2210924u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2210928u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2210932u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2210936u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2210940u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2210944u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2210948u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2210960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc90));
    } else {
        emu.pc = 2210952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc88));
    }
}
#[inline(always)]
pub fn block_0x0021bc88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2210956u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2210960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bca8));
}
#[inline(always)]
pub fn block_0x0021bc90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2210972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc9c));
    } else {
        emu.pc = 2210964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bc94));
    }
}
#[inline(always)]
pub fn block_0x0021bc94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2210968u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2210972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2210984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bca8));
}
#[inline(always)]
pub fn block_0x0021bc9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2210976u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2210980u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2210984u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2210984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bca8));
}
#[inline(always)]
pub fn block_0x0021bca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2210988u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2210992u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2210996u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2211024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bcd0));
    } else {
        emu.pc = 2211000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bcb8));
    }
}
#[inline(always)]
pub fn block_0x0021bcb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2211004u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2211008u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2211012u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2211076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd04));
    } else {
        emu.pc = 2211016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bcc8));
    }
}
#[inline(always)]
pub fn block_0x0021bcc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2211020u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2211024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd90));
}
#[inline(always)]
pub fn block_0x0021bcd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2211028u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2211032u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2211036u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2211040u32);
    emu.apc_no_count(1usize, 2211040u32, 0u32, 2211044u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021bce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2211052u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2211056u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2211060u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2211064u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2211068u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2211072u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2211016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bcc8));
    } else {
        emu.pc = 2211076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd04));
    }
}
#[inline(always)]
pub fn block_0x0021bd04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2211108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd24));
    } else {
        emu.pc = 2211080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd08));
    }
}
#[inline(always)]
pub fn block_0x0021bd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2211084u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2211088u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2211092u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2211096u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2211100u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2211104u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2211108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd90));
}
#[inline(always)]
pub fn block_0x0021bd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2211112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2211160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd58));
    } else {
        emu.pc = 2211116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021bd2c));
    }
}
#[inline]
pub fn block_0x0021bd2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2211120u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2211124u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2211128u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2211132u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2211136u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2211140u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2211144u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2211148u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2211152u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2211156u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2211160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2211216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd90));
}
#[inline]
pub fn block_0x0021bd58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2211164u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2211168u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2211172u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2211176u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2211180u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2211184u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2211188u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2211192u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2211196u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2211200u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2211204u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2211208u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2211212u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2211216u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2211216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021bd90));
}
#[inline]
pub fn block_0x0021bd90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2211220u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2211224u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2211228u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2211232u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2211236u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2211240u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2211244u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2211248u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2211252u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2211256u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2211260u32;
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
