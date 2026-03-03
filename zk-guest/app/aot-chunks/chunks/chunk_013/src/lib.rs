pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2147980u32;
pub const PC_MAX: u32 = 2150684u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020c68c,
        block_0x0020c6a8,
        block_0x0020c708,
        block_0x0020c72c,
        block_0x0020c75c,
        block_0x0020c76c,
        block_0x0020c770,
        block_0x0020c7a0,
        block_0x0020c7b0,
        block_0x0020c7bc,
        block_0x0020c82c,
        block_0x0020c860,
        block_0x0020c86c,
        block_0x0020c878,
        block_0x0020c8cc,
        block_0x0020c8d8,
        block_0x0020c8f0,
        block_0x0020c910,
        block_0x0020c920,
        block_0x0020c924,
        block_0x0020c948,
        block_0x0020c958,
        block_0x0020c964,
        block_0x0020c974,
        block_0x0020c98c,
        block_0x0020c9a4,
        block_0x0020c9a8,
        block_0x0020c9ac,
        block_0x0020c9b8,
        block_0x0020c9bc,
        block_0x0020c9c8,
        block_0x0020c9f4,
        block_0x0020ca24,
        block_0x0020ca48,
        block_0x0020ca98,
        block_0x0020caa4,
        block_0x0020cab0,
        block_0x0020cb00,
        block_0x0020cb08,
        block_0x0020cb28,
        block_0x0020cb48,
        block_0x0020cb90,
        block_0x0020cb9c,
        block_0x0020cba8,
        block_0x0020cbc4,
        block_0x0020cbc8,
        block_0x0020cbe8,
        block_0x0020cbec,
        block_0x0020cbf4,
        block_0x0020cc00,
        block_0x0020cc68,
        block_0x0020ccb4,
        block_0x0020cccc,
        block_0x0020ccd8,
        block_0x0020ccdc,
        block_0x0020cd10,
        block_0x0020cd28,
        block_0x0020cd2c,
        block_0x0020cd3c,
        block_0x0020cd44,
        block_0x0020cd74,
        block_0x0020cd7c,
        block_0x0020cd84,
        block_0x0020cd88,
        block_0x0020cd94,
        block_0x0020cda4,
        block_0x0020cdf8,
        block_0x0020ce08,
        block_0x0020ce0c,
        block_0x0020ce18,
        block_0x0020ce34,
        block_0x0020ce50,
        block_0x0020ce64,
        block_0x0020ce6c,
        block_0x0020ce7c,
        block_0x0020ce80,
        block_0x0020ceac,
        block_0x0020ceb4,
        block_0x0020ceb8,
        block_0x0020cec0,
        block_0x0020cecc,
        block_0x0020cedc,
        block_0x0020ceec,
        block_0x0020cef4,
        block_0x0020cf14,
        block_0x0020cf30,
        block_0x0020cf34,
        block_0x0020cf50,
        block_0x0020cf58,
        block_0x0020cf84,
        block_0x0020cfbc,
        block_0x0020cfe8,
        block_0x0020d018,
        block_0x0020d02c,
        block_0x0020d054,
        block_0x0020d074,
        block_0x0020d080,
        block_0x0020d084,
        block_0x0020d0a0,
        block_0x0020d0a4,
        block_0x0020d0b4,
        block_0x0020d0bc,
        block_0x0020d0c4,
        block_0x0020d0d4,
        block_0x0020d0f4,
        block_0x0020d108,
        block_0x0020d11c,
    ];
    const IDX: [u16; 677usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        5u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 15u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 26u16, 27u16, 28u16, 0u16, 0u16, 29u16, 30u16, 0u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        38u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16,
        43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 46u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 49u16, 0u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 0u16,
        59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 64u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16,
        68u16, 69u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 74u16,
        0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 77u16, 0u16, 78u16, 79u16, 0u16, 80u16, 0u16, 0u16, 81u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        96u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16,
        100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16,
        104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16,
        106u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2147980u32 || pc > 2150684u32 {
        return None;
    }
    let word_offset = ((pc - 2147980u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020c68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2147984u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2147988u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2147992u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2147996u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2148000u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148004u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2148140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c72c));
    } else {
        emu.pc = 2148008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6a8));
    }
}
#[inline]
pub fn block_0x0020c6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2148012u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2148016u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2148020u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2148024u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2148028u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2148032u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2148036u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2148040u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2148044u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2148048u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2148052u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2148056u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2148060u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2148064u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2148068u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2148072u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2148076u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2148080u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148084u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965856u32, 2148088u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2148092u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2148096u32);
    emu.apc_no_count(1usize, 2148096u32, 28672u32, 2148100u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2148108u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2148112u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2148116u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2148120u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2148124u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2148128u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2148132u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2148136u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2148140u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2148140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c72c));
}
#[inline]
pub fn block_0x0020c72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2148144u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2148148u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2148152u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2148156u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2148160u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2148164u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2148168u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2148172u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2148176u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2148180u32)?;
    emu.apc_no_count(1usize, 2148180u32, 4294934528u32, 2148184u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2148192u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2148196u32);
    emu.apc_no_count(1usize, 2148196u32, 4294922240u32, 2148200u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7a0));
    } else {
        emu.pc = 2148208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c770));
    }
}
#[inline]
pub fn block_0x0020c770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2148212u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2148216u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2148220u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148224u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965880u32, 2148228u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2148232u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2148236u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2148240u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2148244u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2148248u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2148252u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148256u32;
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
pub fn block_0x0020c7a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2148260u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2148264u32);
    emu.apc_no_count(1usize, 2148264u32, 4096u32, 2148268u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2148276u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148280u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2148448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c860));
    } else {
        emu.pc = 2148284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7bc));
    }
}
#[inline(never)]
pub fn block_0x0020c7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2148288u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2148292u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2148296u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2148300u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2148304u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2148308u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2148312u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2148316u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2148320u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2148324u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2148328u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2148332u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2148336u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2148340u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2148344u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2148348u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2148352u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2148356u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2148360u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2148364u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2148368u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965856u32, 2148376u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2148380u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2148384u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2148388u32);
    emu.apc_no_count(1usize, 2148388u32, 28672u32, 2148392u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148396u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2148400u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2148404u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2148408u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2148412u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2148416u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2148420u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2148424u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2148428u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2148432u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2148436u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2148440u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2148444u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2148448u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2148448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c860));
}
#[inline(always)]
pub fn block_0x0020c860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965880u32, 2148456u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148460u32;
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
pub fn block_0x0020c86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2148464u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2148468u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2148568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8d8));
    } else {
        emu.pc = 2148472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c878));
    }
}
#[inline]
pub fn block_0x0020c878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2148476u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2148480u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2148484u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2148488u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2148492u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2148496u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2148500u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2148504u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2148508u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2148512u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2148516u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2148520u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2148524u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2148528u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2148532u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2148536u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2148540u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2148544u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2148548u32);
    emu.apc_no_count(1usize, 2148548u32, 28672u32, 2148552u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c8cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2148560u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2148564u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148568u32;
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
pub fn block_0x0020c8d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2148572u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2148576u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2148580u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2148584u32);
    emu.apc_no_count(6usize, 2148584u32, 32768u32, 2148588u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2148592u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020c8f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2148596u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2148600u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2148604u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2148608u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2148612u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2148616u32)?;
    emu.apc_no_count(1usize, 2148616u32, 4294934528u32, 2148620u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2148628u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2148632u32);
    emu.apc_no_count(1usize, 2148632u32, 4294922240u32, 2148636u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c948));
    } else {
        emu.pc = 2148644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c924));
    }
}
#[inline]
pub fn block_0x0020c924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148648u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965896u32, 2148652u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2148656u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2148660u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2148664u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2148668u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2148672u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2148676u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148680u32;
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
pub fn block_0x0020c948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2148684u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2148688u32);
    emu.apc_no_count(1usize, 2148688u32, 4096u32, 2148692u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148696u32;
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
pub fn block_0x0020c958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965896u32, 2148704u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148708u32;
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
pub fn block_0x0020c964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2148712u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2148716u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2148720u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148724u32;
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
pub fn block_0x0020c974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2148728u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2148732u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2148736u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2148740u32);
    emu.apc_no_count(6usize, 2148740u32, 32768u32, 2148744u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2148748u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c98c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2148752u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2148756u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2148760u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2148764u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2148768u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2148792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9b8));
    } else {
        emu.pc = 2148772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9a4));
    }
}
#[inline(always)]
pub fn block_0x0020c9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9f4));
    } else {
        emu.pc = 2148776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9a8));
    }
}
#[inline(always)]
pub fn block_0x0020c9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2148852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9f4));
    } else {
        emu.pc = 2148780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9ac));
    }
}
#[inline(always)]
pub fn block_0x0020c9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2148784u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2148788u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2148792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9c8));
}
#[inline(always)]
pub fn block_0x0020c9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2148852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9f4));
    } else {
        emu.pc = 2148796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9bc));
    }
}
#[inline(always)]
pub fn block_0x0020c9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2148800u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2148804u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2148808u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9c8));
}
#[inline]
pub fn block_0x0020c9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2148812u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2148816u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2148820u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2148824u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2148828u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2148832u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965912u32, 2148840u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2148844u32);
    emu.apc_no_count(1usize, 2148844u32, 0u32, 2148848u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148852u32;
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
#[inline]
pub fn block_0x0020c9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2148856u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2148860u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2148864u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2148868u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2148872u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2148876u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2148880u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965940u32, 2148888u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2148892u32);
    emu.apc_no_count(1usize, 2148892u32, 0u32, 2148896u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ca24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2148904u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2148908u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2148912u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2148916u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2148920u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2148924u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2148928u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2148932u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2148936u32;
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
pub fn block_0x0020ca48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2148940u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2148944u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2148948u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2148952u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2148956u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2148960u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2148964u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2148968u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2148972u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2148976u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2148980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2148984u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2148988u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2148992u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2148996u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2149000u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2149004u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2149008u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2149012u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caa4));
    } else {
        emu.pc = 2149016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca98));
    }
}
#[inline(always)]
pub fn block_0x0020ca98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2149020u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2149024u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2149028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb08));
}
#[inline(always)]
pub fn block_0x0020caa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2149032u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2149036u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2149040u32;
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
pub fn block_0x0020cab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2149044u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2149048u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2149052u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2149056u32)?;
    let a = 0u32.wrapping_add(4151074816u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965826u32, 2149064u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2149068u32);
    let a = 0u32.wrapping_add(228253696u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149072u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 203u32, 2149076u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2149080u32);
    let a = 0u32.wrapping_add(1618087936u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149084u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1443u32, 2149088u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2149092u32);
    let a = 0u32.wrapping_add(4257644544u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149096u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 861u32, 2149100u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2149104u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2149108u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2149112u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2149116u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb28));
    } else {
        emu.pc = 2149120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb00));
    }
}
#[inline(always)]
pub fn block_0x0020cb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2149124u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2149128u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb08));
}
#[inline(always)]
pub fn block_0x0020cb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2149132u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2149136u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2149140u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2149144u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149148u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149152u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149156u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149160u32;
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
pub fn block_0x0020cb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149164u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965968u32, 2149168u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2149172u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2149176u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2149180u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2149184u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2149188u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149192u32;
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
pub fn block_0x0020cb48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2149196u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2149200u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2149204u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2149208u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2149212u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2149216u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2149220u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2149224u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2149228u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2149232u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2149236u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2149240u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2149244u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2149248u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2149252u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2149256u32);
    emu.apc_no_count(1usize, 2149256u32, 4096u32, 2149260u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149264u32;
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
pub fn block_0x0020cb90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2149268u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2149272u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbec));
    } else {
        emu.pc = 2149276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb9c));
    }
}
#[inline(always)]
pub fn block_0x0020cb9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149280u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294967292u32, 2149284u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2149768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd88));
    } else {
        emu.pc = 2149288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cba8));
    }
}
#[inline(always)]
pub fn block_0x0020cba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294967292u32, 2149292u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2149296u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2149300u32);
    emu.sw_no_count(11usize, 10usize, 4294967292u32, 2149304u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2149308u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2149312u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2149592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccd8));
    } else {
        emu.pc = 2149316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbc4));
    }
}
#[inline(always)]
pub fn block_0x0020cbc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2149320u32;
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
pub fn block_0x0020cbc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2149324u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2149328u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149332u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2149336u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2149340u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2149344u32);
    emu.apc_no_count(1usize, 2149344u32, 0u32, 2149348u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cbe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2149356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd10));
}
#[inline(always)]
pub fn block_0x0020cbec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2149360u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2149480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc68));
    } else {
        emu.pc = 2149364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbf4));
    }
}
#[inline(always)]
pub fn block_0x0020cbf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2149368u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2149372u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2149376u32;
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
#[inline(never)]
pub fn block_0x0020cc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2149380u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2149384u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149388u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1992u32, 2149392u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2149396u32);
    let a = 0u32.wrapping_add(2150400u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2149400u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1296u32, 2149404u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2149408u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966096u32, 2149412u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2149416u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2149420u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2149424u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2149428u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2149432u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2149436u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2149440u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2149444u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2149448u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2149452u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2149456u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2149460u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2149464u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2149468u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2149472u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2149476u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2149480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccb4));
}
#[inline]
pub fn block_0x0020cc68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2149484u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149488u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1992u32, 2149492u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2149496u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2149500u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966924u32, 2149504u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2149508u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966008u32, 2149512u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2149516u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2149520u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2149524u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2149528u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2149532u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2149536u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2149540u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2149544u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2149548u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2149552u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2149556u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2149556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccb4));
}
#[inline(always)]
pub fn block_0x0020ccb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2149560u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2149564u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2149568u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2149572u32);
    emu.apc_no_count(1usize, 2149572u32, 0u32, 2149576u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2149584u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2149588u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2149592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd7c));
}
#[inline(always)]
pub fn block_0x0020ccd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2149596u32;
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
pub fn block_0x0020ccdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149600u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2149604u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2149608u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2149612u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2149616u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2149620u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2149624u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2149628u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2149632u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2149636u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2149640u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2149644u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2149648u32;
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
pub fn block_0x0020cd10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149652u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294967292u32, 2149656u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2149660u32);
    emu.sw_no_count(11usize, 10usize, 4294967292u32, 2149664u32)?;
    emu.apc_no_count(1usize, 2149664u32, 4096u32, 2149668u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cd28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2149692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd3c));
    } else {
        emu.pc = 2149676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cd2c));
    }
}
#[inline(always)]
pub fn block_0x0020cd2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2149680u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2149684u32);
    emu.apc_no_count(1usize, 2149684u32, 0u32, 2149688u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cd3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149696u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966168u32, 2149700u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd44));
}
#[inline]
pub fn block_0x0020cd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2149704u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2149708u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2149712u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2149716u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2149720u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2149724u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2149728u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2149732u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2149736u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2149740u32);
    emu.apc_no_count(1usize, 2149740u32, 0u32, 2149744u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cd74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2149752u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2149756u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2149756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd7c));
}
#[inline(always)]
pub fn block_0x0020cd7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2149756u32, 4294963200u32, 2149760u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cd84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2149764u32));
}
#[inline(always)]
pub fn block_0x0020cd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2149772u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966316u32, 2149776u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2149780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2149700u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cd44));
}
#[inline(always)]
pub fn block_0x0020cd94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2149784u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2149788u32)?;
    emu.apc_no_count(1usize, 2149788u32, 4294934528u32, 2149792u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149796u32;
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
#[inline]
pub fn block_0x0020cda4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2149800u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2149804u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149808u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2149812u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2149816u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966240u32, 2149820u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2149824u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2149828u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2149832u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2149836u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2149840u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2149844u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2149848u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2149852u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2149856u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2149860u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2149864u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2149868u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2149872u32);
    emu.apc_no_count(1usize, 2149872u32, 0u32, 2149876u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2149884u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2149888u32)?;
    emu.apc_no_count(1usize, 2149888u32, 4294963200u32, 2149892u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ce08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2149896u32));
}
#[inline(always)]
pub fn block_0x0020ce0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2149904u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2149908u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149912u32;
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
pub fn block_0x0020ce18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2149916u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2149920u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2149924u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2149928u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2149932u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2149936u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149940u32;
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
pub fn block_0x0020ce34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2149944u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2149948u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2149952u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2149956u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2149960u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2149964u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2149968u32;
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
pub fn block_0x0020ce50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2149972u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2149976u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966324u32, 2149980u32);
    emu.apc_no_count(6usize, 2149980u32, 28672u32, 2149984u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2149988u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ce64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2149992u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2150012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce7c));
    } else {
        emu.pc = 2149996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce6c));
    }
}
#[inline(always)]
pub fn block_0x0020ce6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2150000u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2150004u32);
    emu.apc_no_count(6usize, 2150004u32, 4294922240u32, 2150008u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2150012u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ce7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150016u32;
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
pub fn block_0x0020ce80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2150020u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2150024u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2150028u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2150032u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2150036u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2150040u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2150044u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2150048u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2150052u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2150056u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2150068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceb4));
    } else {
        emu.pc = 2150060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceac));
    }
}
#[inline(always)]
pub fn block_0x0020ceac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2150064u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cecc));
}
#[inline(always)]
pub fn block_0x0020ceb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cec0));
    } else {
        emu.pc = 2150072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceb8));
    }
}
#[inline(always)]
pub fn block_0x0020ceb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2150076u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150080u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cecc));
}
#[inline(always)]
pub fn block_0x0020cec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2150084u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2150088u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2150092u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2150092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cecc));
}
#[inline(always)]
pub fn block_0x0020cecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2150096u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2150100u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2150104u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2150132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cef4));
    } else {
        emu.pc = 2150108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cedc));
    }
}
#[inline(always)]
pub fn block_0x0020cedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2150112u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2150116u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2150120u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2150192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf30));
    } else {
        emu.pc = 2150124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceec));
    }
}
#[inline(always)]
pub fn block_0x0020ceec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2150128u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfbc));
}
#[inline(always)]
pub fn block_0x0020cef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2150136u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2150140u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2150144u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2150148u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2150152u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2150156u32);
    emu.apc_no_count(1usize, 2150156u32, 4294959104u32, 2150160u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150164u32;
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
pub fn block_0x0020cf14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2150168u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2150172u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2150176u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2150180u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2150184u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2150188u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2150124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ceec));
    } else {
        emu.pc = 2150192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf30));
    }
}
#[inline(always)]
pub fn block_0x0020cf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf50));
    } else {
        emu.pc = 2150196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf34));
    }
}
#[inline(always)]
pub fn block_0x0020cf34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2150200u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2150204u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2150208u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2150212u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2150216u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2150220u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2150224u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfbc));
}
#[inline(always)]
pub fn block_0x0020cf50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2150228u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2150276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf84));
    } else {
        emu.pc = 2150232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf58));
    }
}
#[inline]
pub fn block_0x0020cf58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2150236u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2150240u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2150244u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2150248u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2150252u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2150256u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2150260u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2150264u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2150268u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2150272u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2150276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfbc));
}
#[inline]
pub fn block_0x0020cf84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2150280u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2150284u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2150288u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2150292u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2150296u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2150300u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2150304u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2150308u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2150312u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2150316u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2150320u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2150324u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2150328u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2150332u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2150332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cfbc));
}
#[inline]
pub fn block_0x0020cfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2150336u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2150340u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2150344u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150348u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150352u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150356u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150360u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150364u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2150368u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150372u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150376u32;
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
pub fn block_0x0020cfe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2150380u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2150384u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2150388u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2150392u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2150396u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2150400u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2150404u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2150408u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2150412u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2150416u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2150420u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2150484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d054));
    } else {
        emu.pc = 2150424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d018));
    }
}
#[inline(always)]
pub fn block_0x0020d018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2150428u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2150432u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2150436u32);
    emu.apc_no_count(1usize, 2150436u32, 4294934528u32, 2150440u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020d02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2150448u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2150452u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2150456u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2150460u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2150464u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2150468u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2150472u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2150476u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2150480u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150484u32;
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
pub fn block_0x0020d054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2150488u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2150492u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2150496u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2150500u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2150504u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2150508u32);
    emu.apc_no_count(1usize, 2150508u32, 4294959104u32, 2150512u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2150520u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2150524u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2150528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d018));
}
#[inline(always)]
pub fn block_0x0020d080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2150528u32));
}
#[inline(always)]
pub fn block_0x0020d084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2150536u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2150540u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2150544u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2150548u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2150552u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2150556u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2150560u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2150560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0a0));
}
#[inline(always)]
pub fn block_0x0020d0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2150588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0bc));
    } else {
        emu.pc = 2150564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a4));
    }
}
#[inline(always)]
pub fn block_0x0020d0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2150568u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2150572u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2150576u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2150560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0a0));
    } else {
        emu.pc = 2150580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d0b4));
    }
}
#[inline(always)]
pub fn block_0x0020d0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2150584u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2150588u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0c4));
}
#[inline(always)]
pub fn block_0x0020d0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2150592u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2150596u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2150596u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d0c4));
}
#[inline(always)]
pub fn block_0x0020d0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2150600u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2150604u32);
    emu.apc_no_count(1usize, 2150604u32, 4294934528u32, 2150608u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2150616u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2150620u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2150624u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2150628u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2150632u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2150636u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2150640u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150644u32;
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
pub fn block_0x0020d0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2150648u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2150652u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2150656u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2150660u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2150684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d11c));
    } else {
        emu.pc = 2150664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d108));
    }
}
#[inline(always)]
pub fn block_0x0020d108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2150668u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2150672u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2150676u32);
    emu.apc_no_count(1usize, 2150676u32, 4294934528u32, 2150680u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150684u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d11c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2150688u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2150692u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2150696u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2150700u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2150704u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150708u32;
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
