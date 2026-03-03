pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2134144u32;
pub const PC_MAX: u32 = 2136388u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00209080,
        block_0x00209094,
        block_0x002090a8,
        block_0x002090d4,
        block_0x002090e0,
        block_0x002090e8,
        block_0x00209148,
        block_0x0020914c,
        block_0x00209164,
        block_0x00209178,
        block_0x00209184,
        block_0x002091a4,
        block_0x002091a8,
        block_0x002091c0,
        block_0x002091f4,
        block_0x002091fc,
        block_0x00209204,
        block_0x00209208,
        block_0x00209210,
        block_0x00209214,
        block_0x0020921c,
        block_0x00209220,
        block_0x00209228,
        block_0x0020922c,
        block_0x00209230,
        block_0x00209234,
        block_0x0020923c,
        block_0x00209258,
        block_0x00209260,
        block_0x00209264,
        block_0x00209274,
        block_0x00209278,
        block_0x00209280,
        block_0x00209290,
        block_0x0020929c,
        block_0x002092a0,
        block_0x002092a8,
        block_0x002092b0,
        block_0x002092cc,
        block_0x002092d0,
        block_0x002092ec,
        block_0x00209300,
        block_0x00209314,
        block_0x0020932c,
        block_0x00209368,
        block_0x00209384,
        block_0x00209398,
        block_0x002093b8,
        block_0x002093f4,
        block_0x002093fc,
        block_0x00209428,
        block_0x00209448,
        block_0x00209470,
        block_0x00209490,
        block_0x002094d4,
        block_0x002094dc,
        block_0x00209508,
        block_0x0020952c,
        block_0x00209554,
        block_0x00209598,
        block_0x002095a0,
        block_0x002095b8,
        block_0x002095c0,
        block_0x002095c8,
        block_0x002095d0,
        block_0x002095f0,
        block_0x002095f4,
        block_0x00209640,
        block_0x00209648,
        block_0x00209658,
        block_0x0020966c,
        block_0x00209688,
        block_0x00209690,
        block_0x002096b4,
        block_0x002096bc,
        block_0x002096c4,
        block_0x002096d4,
        block_0x002096dc,
        block_0x002096e4,
        block_0x002096f4,
        block_0x0020970c,
        block_0x00209734,
        block_0x00209748,
        block_0x0020978c,
        block_0x00209794,
        block_0x002097ac,
        block_0x002097b4,
        block_0x002097bc,
        block_0x002097c4,
        block_0x002097e4,
        block_0x002097e8,
        block_0x00209834,
        block_0x0020983c,
        block_0x0020984c,
        block_0x00209860,
        block_0x0020987c,
        block_0x00209884,
        block_0x002098a8,
        block_0x002098b0,
        block_0x002098b8,
        block_0x002098c8,
        block_0x002098d0,
        block_0x002098d8,
        block_0x002098e8,
        block_0x00209900,
        block_0x00209928,
        block_0x0020993c,
        block_0x00209944,
    ];
    const IDX: [u16; 562usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16,
        6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 8u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16,
        11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 17u16, 18u16, 0u16, 19u16, 20u16, 0u16,
        21u16, 22u16, 0u16, 23u16, 24u16, 25u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 29u16, 30u16, 0u16, 0u16, 0u16, 31u16, 32u16,
        0u16, 33u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 35u16, 36u16, 0u16, 37u16,
        0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16,
        75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 79u16, 0u16,
        0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 84u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16, 0u16,
        88u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 100u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16,
    ];
    if pc < 2134144u32 || pc > 2136388u32 {
        return None;
    }
    let word_offset = ((pc - 2134144u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00209080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2134148u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2134152u32);
    emu.adi_no_count(14usize, 0usize, 23u32, 2134156u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2134160u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2134184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002090a8));
    } else {
        emu.pc = 2134164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209094));
    }
}
#[inline(always)]
pub fn block_0x00209094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134168u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 612u32, 2134172u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134176u32);
    emu.apc_no_count(6usize, 2134176u32, 102400u32, 2134180u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2134184u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002090a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2134188u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2134192u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2134196u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2134200u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 592u32, 2134204u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134208u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966500u32, 2134212u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2134216u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2134220u32);
    emu.apc_no_count(1usize, 2134220u32, 102400u32, 2134224u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002090d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2134232u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2134236u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134240u32;
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
pub fn block_0x002090e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2134240u32, 4294934528u32, 2134244u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2134248u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002090e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2134252u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2134256u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2134260u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2134264u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2134268u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2134272u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2134276u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2134280u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2134284u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2134288u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2134292u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2134296u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2134300u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2134304u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2134308u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2134312u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2134316u32);
    emu.lw_no_count(18usize, 11usize, 16u32, 2134320u32)?;
    emu.lw_no_count(19usize, 11usize, 12u32, 2134324u32)?;
    emu.lw_no_count(11usize, 11usize, 8u32, 2134328u32)?;
    emu.sw_no_count(20usize, 2usize, 0u32, 2134332u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2134336u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2134340u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2134344u32);
    emu.add_memory_rw_events(24usize);
    emu.pc = 2134344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209148));
}
#[inline(always)]
pub fn block_0x00209148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2134784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209300));
    } else {
        emu.pc = 2134348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020914c));
    }
}
#[inline(always)]
pub fn block_0x0020914c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 18usize, 11usize, 2134352u32);
    emu.sw_no_count(22usize, 20usize, 8u32, 2134356u32)?;
    emu.sw_no_count(19usize, 20usize, 12u32, 2134360u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2134364u32);
    emu.apc_no_count(1usize, 2134364u32, 4294930432u32, 2134368u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2134376u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2134380u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2134384u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2134388u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2134344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209148));
    } else {
        emu.pc = 2134392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209178));
    }
}
#[inline(always)]
pub fn block_0x00209178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 12usize, 0u32, 2134396u32);
    emu.apc_no_count(1usize, 2134396u32, 0u32, 2134400u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134408u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2134412u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2134416u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2134420u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2134424u32);
    emu.adi_no_count(21usize, 0usize, 8u32, 2134428u32);
    emu.apc_no_count(1usize, 2134428u32, 8192u32, 2134432u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002091a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2134916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209384));
    } else {
        emu.pc = 2134440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091a8));
    }
}
#[inline(always)]
pub fn block_0x002091a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 10usize, 0u32, 2134444u32);
    emu.lw_no_count(11usize, 20usize, 12u32, 2134448u32)?;
    emu.sw_no_count(21usize, 2usize, 0u32, 2134452u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2134456u32)?;
    emu.sw_no_count(18usize, 2usize, 8u32, 2134460u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209314));
    } else {
        emu.pc = 2134464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091c0));
    }
}
#[inline]
pub fn block_0x002091c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 20usize, 0u32, 2134468u32)?;
    emu.lw_no_count(18usize, 20usize, 4u32, 2134472u32)?;
    emu.lw_no_count(19usize, 20usize, 8u32, 2134476u32)?;
    emu.lw_no_count(27usize, 20usize, 16u32, 2134480u32)?;
    emu.adi_no_count(21usize, 0usize, 1u32, 2134484u32);
    emu.adi_no_count(22usize, 0usize, 4294967293u32, 2134488u32);
    emu.adi_no_count(14usize, 0usize, 4294967232u32, 2134492u32);
    emu.adi_no_count(24usize, 0usize, 57u32, 2134496u32);
    emu.adi_no_count(25usize, 0usize, 15u32, 2134500u32);
    emu.adi_no_count(15usize, 0usize, 43u32, 2134504u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2134508u32);
    emu.adi_no_count(27usize, 27usize, 1u32, 2134512u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2134516u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2134516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002091f4));
}
#[inline(always)]
pub fn block_0x002091f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(23usize, 21usize, 11usize, 2134520u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2134524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209208));
}
#[inline(always)]
pub fn block_0x002091fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967248u32, 2134528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2134672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209290));
    } else {
        emu.pc = 2134532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209204));
    }
}
#[inline(always)]
pub fn block_0x00209204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002092a0));
}
#[inline(always)]
pub fn block_0x00209208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 19usize, 2u32, 2134540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2134672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209290));
    } else {
        emu.pc = 2134544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209210));
    }
}
#[inline(always)]
pub fn block_0x00209210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2134888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209368));
    } else {
        emu.pc = 2134548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209214));
    }
}
#[inline(always)]
pub fn block_0x00209214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 9usize, 19usize, 2134552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2134576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209230));
    } else {
        emu.pc = 2134556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020921c));
    }
}
#[inline(always)]
pub fn block_0x0020921c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2134572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020922c));
    } else {
        emu.pc = 2134560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209220));
    }
}
#[inline(always)]
pub fn block_0x00209220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(12usize, 11usize, 0u32, 2134564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2134576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209230));
    } else {
        emu.pc = 2134568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209228));
    }
}
#[inline(always)]
pub fn block_0x00209228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134888u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209368));
}
#[inline(always)]
pub fn block_0x0020922c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2134888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209368));
    } else {
        emu.pc = 2134576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209230));
    }
}
#[inline(always)]
pub fn block_0x00209230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2134588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020923c));
    } else {
        emu.pc = 2134580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209234));
    }
}
#[inline(always)]
pub fn block_0x00209234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(12usize, 11usize, 2u32, 2134584u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2134888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209368));
    } else {
        emu.pc = 2134588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020923c));
    }
}
#[inline(always)]
pub fn block_0x0020923c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 11usize, 0u32, 2134592u32);
    emu.adi_no_count(12usize, 11usize, 4294967253u32, 2134596u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2134600u32);
    emu.adr_no_count(13usize, 9usize, 19usize, 2134604u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2134608u32);
    emu.lbu_no_count(13usize, 12usize, 0u32, 2134612u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2134628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209264));
    } else {
        emu.pc = 2134616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209258));
    }
}
#[inline(always)]
pub fn block_0x00209258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 13usize, 4294967248u32, 2134620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2134644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209274));
    } else {
        emu.pc = 2134624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209260));
    }
}
#[inline(always)]
pub fn block_0x00209260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209290));
}
#[inline(always)]
pub fn block_0x00209264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967231u32, 2134632u32);
    emu.ani_no_count(26usize, 13usize, 4294967263u32, 2134636u32);
    emu.adi_no_count(26usize, 26usize, 10u32, 2134640u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2134672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209290));
    } else {
        emu.pc = 2134644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209274));
    }
}
#[inline(always)]
pub fn block_0x00209274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2134696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092a8));
    } else {
        emu.pc = 2134648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209278));
    }
}
#[inline(always)]
pub fn block_0x00209278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 12usize, 1u32, 2134652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2134524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091fc));
    } else {
        emu.pc = 2134656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209280));
    }
}
#[inline(always)]
pub fn block_0x00209280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967231u32, 2134660u32);
    emu.ani_no_count(11usize, 11usize, 4294967263u32, 2134664u32);
    emu.adi_no_count(11usize, 11usize, 10u32, 2134668u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2134688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092a0));
    } else {
        emu.pc = 2134672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209290));
    }
}
#[inline(always)]
pub fn block_0x00209290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 23usize, 1u32, 2134676u32);
    emu.adr_no_count(19usize, 19usize, 27usize, 2134680u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2134536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209208));
    } else {
        emu.pc = 2134684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020929c));
    }
}
#[inline(always)]
pub fn block_0x0020929c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209314));
}
#[inline(always)]
pub fn block_0x002092a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(26usize, 26usize, 4u32, 2134692u32);
    emu.orr_no_count(26usize, 26usize, 11usize, 2134696u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2134696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002092a8));
}
#[inline(always)]
pub fn block_0x002092a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 0u32, 2134700u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2134736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092d0));
    } else {
        emu.pc = 2134704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092b0));
    }
}
#[inline(always)]
pub fn block_0x002092b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 23usize, 2134708u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2134712u32);
    emu.adr_no_count(12usize, 10usize, 20usize, 2134716u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2134720u32);
    emu.sb_no_count(26usize, 12usize, 0u32, 2134724u32);
    emu.sw_no_count(20usize, 2usize, 8u32, 2134728u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2134516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002091f4));
    } else {
        emu.pc = 2134732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002092cc));
    }
}
#[inline(always)]
pub fn block_0x002092cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2134736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209314));
}
#[inline(always)]
pub fn block_0x002092d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2134740u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2134744u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2134748u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2134752u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2134756u32);
    emu.apc_no_count(1usize, 2134756u32, 4294946816u32, 2134760u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002092ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2134768u32);
    emu.adi_no_count(15usize, 0usize, 43u32, 2134772u32);
    emu.adi_no_count(14usize, 0usize, 4294967232u32, 2134776u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2134780u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2134784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002092b0));
}
#[inline(always)]
pub fn block_0x00209300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2134788u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2134792u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2134796u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2134800u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2134804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2134828u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020932c));
}
#[inline(always)]
pub fn block_0x00209314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2134808u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2134812u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2134816u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2134820u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2134824u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2134828u32)?;
    emu.add_memory_rw_events(6usize);
    emu.pc = 2134828u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020932c));
}
#[inline]
pub fn block_0x0020932c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2134832u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2134836u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2134840u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2134844u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2134848u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2134852u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2134856u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2134860u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2134864u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2134868u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2134872u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2134876u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2134880u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2134884u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134888u32;
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
pub fn block_0x00209368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2134892u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966812u32, 2134896u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2134900u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2134904u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2134908u32);
    emu.apc_no_count(1usize, 2134908u32, 86016u32, 2134912u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2134920u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2134924u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2134928u32);
    emu.apc_no_count(1usize, 2134928u32, 73728u32, 2134932u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2134936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2134940u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2134944u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2134948u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2134952u32)?;
    emu.ani_no_count(13usize, 12usize, 1u32, 2134956u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2134960u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2134964u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2135080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209428));
    } else {
        emu.pc = 2134968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002093b8));
    }
}
#[inline]
pub fn block_0x002093b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2134972u32);
    emu.adi_no_count(14usize, 2usize, 12u32, 2134976u32);
    emu.adi_no_count(9usize, 10usize, 2u32, 2134980u32);
    emu.sw_no_count(11usize, 2usize, 32u32, 2134984u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2134988u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2134992u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2134996u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2135000u32)?;
    emu.sw_no_count(14usize, 2usize, 48u32, 2135004u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135008u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 508u32, 2135012u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2135016u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2135020u32);
    emu.apc_no_count(1usize, 2135020u32, 0u32, 2135024u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135028u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002093f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2135032u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209448));
    } else {
        emu.pc = 2135036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002093fc));
    }
}
#[inline]
pub fn block_0x002093fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2135040u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2135044u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2135048u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2135052u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2135056u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2135060u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135064u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135068u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135072u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135076u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135080u32;
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
pub fn block_0x00209428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135084u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2135088u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2135092u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135096u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135100u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135104u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135108u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135112u32;
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
pub fn block_0x00209448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 16u32, 2135116u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135120u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 8usize, 0u32, 2135124u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2135128u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2135132u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135136u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135140u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135144u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135148u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135152u32;
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
pub fn block_0x00209470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2135156u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2135160u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2135164u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2135168u32)?;
    emu.lw_no_count(12usize, 11usize, 8u32, 2135172u32)?;
    emu.ani_no_count(13usize, 12usize, 1u32, 2135176u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2135180u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2135304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209508));
    } else {
        emu.pc = 2135184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209490));
    }
}
#[inline]
pub fn block_0x00209490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2135188u32)?;
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135192u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 2u32, 2135196u32);
    emu.adi_no_count(14usize, 2usize, 12u32, 2135200u32);
    emu.adi_no_count(9usize, 11usize, 2u32, 2135204u32);
    emu.sw_no_count(9usize, 2usize, 12u32, 2135208u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2135212u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2135216u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2135220u32)?;
    emu.sw_no_count(0usize, 2usize, 44u32, 2135224u32)?;
    emu.sw_no_count(14usize, 2usize, 48u32, 2135228u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 508u32, 2135236u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2135240u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2135244u32);
    emu.apc_no_count(1usize, 2135244u32, 0u32, 2135248u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135252u32;
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
pub fn block_0x002094d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2135256u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020952c));
    } else {
        emu.pc = 2135260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002094dc));
    }
}
#[inline]
pub fn block_0x002094dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2135264u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2135268u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2135272u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2135276u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2135280u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2135284u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135288u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135292u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135296u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135300u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135304u32;
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
pub fn block_0x00209508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135308u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135312u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2135316u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2135320u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135324u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135328u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135332u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135336u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135340u32;
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
pub fn block_0x0020952c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 16u32, 2135344u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2135348u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 8usize, 0u32, 2135352u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2135356u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2135360u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2135364u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2135368u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2135372u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2135376u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135380u32;
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
pub fn block_0x00209554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2135384u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2135388u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2135392u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2135396u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2135400u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2135404u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2135408u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2135412u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2135416u32)?;
    emu.adi_no_count(19usize, 12usize, 0u32, 2135420u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2135424u32);
    emu.lw_no_count(12usize, 11usize, 16u32, 2135428u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2135432u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2135436u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2135440u32);
    emu.apc_no_count(1usize, 2135440u32, 4294942720u32, 2135444u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135448u32;
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
pub fn block_0x00209598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2135452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209658));
    } else {
        emu.pc = 2135456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095a0));
    }
}
#[inline(always)]
pub fn block_0x002095a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2135460u32);
    emu.lw_no_count(10usize, 18usize, 16u32, 2135464u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2135468u32)?;
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135472u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2u32, 2135476u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c8));
    } else {
        emu.pc = 2135480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095b8));
    }
}
#[inline(always)]
pub fn block_0x002095b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2135484u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c8));
    } else {
        emu.pc = 2135488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c0));
    }
}
#[inline(always)]
pub fn block_0x002095c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 8u32, 2135492u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096e4));
    } else {
        emu.pc = 2135496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095c8));
    }
}
#[inline(always)]
pub fn block_0x002095c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2135496u32, 0u32, 2135500u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135508u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2135512u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2135516u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2135520u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2135524u32);
    emu.adi_no_count(22usize, 0usize, 8u32, 2135528u32);
    emu.apc_no_count(1usize, 2135528u32, 4096u32, 2135532u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002095f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2135860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209734));
    } else {
        emu.pc = 2135540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002095f4));
    }
}
#[inline]
pub fn block_0x002095f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2135544u32);
    emu.sb_no_count(20usize, 10usize, 0u32, 2135548u32);
    emu.sw_no_count(22usize, 2usize, 12u32, 2135552u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2135556u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2135560u32)?;
    emu.lw_no_count(12usize, 18usize, 16u32, 2135564u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2135568u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2135572u32)?;
    emu.lw_no_count(13usize, 18usize, 8u32, 2135576u32)?;
    emu.lw_no_count(14usize, 18usize, 12u32, 2135580u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2135584u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2135588u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2135592u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2135596u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2135600u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2135604u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2135608u32);
    emu.apc_no_count(1usize, 2135608u32, 4294942720u32, 2135612u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135616u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00209640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2135620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096f4));
    } else {
        emu.pc = 2135624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209648));
    }
}
#[inline(always)]
pub fn block_0x00209648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2135628u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2135632u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 2u32, 2135636u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2135640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2135740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002096bc));
}
#[inline(always)]
pub fn block_0x00209658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2135644u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2135648u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2135652u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2135656u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2135660u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2135820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020970c));
}
#[inline(always)]
pub fn block_0x0020966c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2135664u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2135668u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2135672u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2135676u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2135680u32);
    emu.apc_no_count(1usize, 2135680u32, 4294946816u32, 2135684u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(188u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 16u32, 2135692u32)?;
    emu.adi_no_count(11usize, 20usize, 0u32, 2135696u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2135696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209690));
}
#[inline]
pub fn block_0x00209690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 9usize, 18usize, 2135700u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2135704u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2135708u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2135712u32);
    emu.sw_no_count(18usize, 2usize, 20u32, 2135716u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2135720u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2135724u32);
    emu.apc_no_count(1usize, 2135724u32, 4294942720u32, 2135728u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135732u32;
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
pub fn block_0x002096b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2135736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096f4));
    } else {
        emu.pc = 2135740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096bc));
    }
}
#[inline(always)]
pub fn block_0x002096bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2135744u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2135696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209690));
    } else {
        emu.pc = 2135748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096c4));
    }
}
#[inline(always)]
pub fn block_0x002096c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2135752u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2135756u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2135760u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020966c));
    } else {
        emu.pc = 2135764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096d4));
    }
}
#[inline(always)]
pub fn block_0x002096d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2135768u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020966c));
    } else {
        emu.pc = 2135772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096dc));
    }
}
#[inline(always)]
pub fn block_0x002096dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2135776u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020966c));
    } else {
        emu.pc = 2135780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002096e4));
    }
}
#[inline(always)]
pub fn block_0x002096e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2135784u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965308u32, 2135788u32);
    emu.apc_no_count(1usize, 2135788u32, 98304u32, 2135792u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002096f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2135800u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2135804u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2135808u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2135812u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2135816u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2135820u32)?;
    emu.add_memory_rw_events(6usize);
    emu.pc = 2135820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020970c));
}
#[inline]
pub fn block_0x0020970c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2135824u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2135828u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2135832u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2135836u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2135840u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2135844u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2135848u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2135852u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2135856u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135860u32;
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
pub fn block_0x00209734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2135864u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2135868u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2135872u32);
    emu.apc_no_count(1usize, 2135872u32, 73728u32, 2135876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2135884u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2135888u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2135892u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2135896u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2135900u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2135904u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2135908u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2135912u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2135916u32)?;
    emu.adi_no_count(19usize, 12usize, 0u32, 2135920u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2135924u32);
    emu.lw_no_count(12usize, 11usize, 16u32, 2135928u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2135932u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2135936u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2135940u32);
    emu.apc_no_count(1usize, 2135940u32, 4294942720u32, 2135944u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2135948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(92u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020978c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2135952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020984c));
    } else {
        emu.pc = 2135956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209794));
    }
}
#[inline(always)]
pub fn block_0x00209794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2135960u32);
    emu.lw_no_count(10usize, 18usize, 16u32, 2135964u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2135968u32)?;
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2135972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 2u32, 2135976u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2135996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097bc));
    } else {
        emu.pc = 2135980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097ac));
    }
}
#[inline(always)]
pub fn block_0x002097ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2135984u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2135996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097bc));
    } else {
        emu.pc = 2135988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097b4));
    }
}
#[inline(always)]
pub fn block_0x002097b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 8u32, 2135992u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098d8));
    } else {
        emu.pc = 2135996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097bc));
    }
}
#[inline(always)]
pub fn block_0x002097bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2135996u32, 0u32, 2136000u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136004u32;
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
#[inline(always)]
pub fn block_0x002097c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2136008u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2136012u32);
    emu.adi_no_count(21usize, 0usize, 1u32, 2136016u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2136020u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2136024u32);
    emu.adi_no_count(22usize, 0usize, 8u32, 2136028u32);
    emu.apc_no_count(1usize, 2136028u32, 4096u32, 2136032u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136036u32;
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
pub fn block_0x002097e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2136360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209928));
    } else {
        emu.pc = 2136040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002097e8));
    }
}
#[inline]
pub fn block_0x002097e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2136044u32);
    emu.sb_no_count(20usize, 10usize, 0u32, 2136048u32);
    emu.sw_no_count(22usize, 2usize, 12u32, 2136052u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2136056u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2136060u32)?;
    emu.lw_no_count(12usize, 18usize, 16u32, 2136064u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2136068u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2136072u32)?;
    emu.lw_no_count(13usize, 18usize, 8u32, 2136076u32)?;
    emu.lw_no_count(14usize, 18usize, 12u32, 2136080u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2136084u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2136088u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2136092u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2136096u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2136100u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2136104u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2136108u32);
    emu.apc_no_count(1usize, 2136108u32, 4294942720u32, 2136112u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2136120u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e8));
    } else {
        emu.pc = 2136124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020983c));
    }
}
#[inline(always)]
pub fn block_0x0020983c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2136128u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2136132u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 2u32, 2136136u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2136140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002098b0));
}
#[inline(always)]
pub fn block_0x0020984c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2136144u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2136148u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2136152u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2136156u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2136160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2136320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209900));
}
#[inline(always)]
pub fn block_0x00209860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2136164u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2136168u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2136172u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2136176u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2136180u32);
    emu.apc_no_count(1usize, 2136180u32, 4294946816u32, 2136184u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020987c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 16u32, 2136192u32)?;
    emu.adi_no_count(11usize, 20usize, 0u32, 2136196u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2136196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209884));
}
#[inline]
pub fn block_0x00209884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 9usize, 18usize, 2136200u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2136204u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2136208u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2136212u32);
    emu.sw_no_count(18usize, 2usize, 20u32, 2136216u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2136220u32);
    emu.adi_no_count(11usize, 2usize, 47u32, 2136224u32);
    emu.apc_no_count(1usize, 2136224u32, 4294942720u32, 2136228u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136232u32;
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
pub fn block_0x002098a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2136236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098e8));
    } else {
        emu.pc = 2136240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098b0));
    }
}
#[inline(always)]
pub fn block_0x002098b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2136244u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2136196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209884));
    } else {
        emu.pc = 2136248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098b8));
    }
}
#[inline(always)]
pub fn block_0x002098b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2136252u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2136256u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2136260u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098c8));
    }
}
#[inline(always)]
pub fn block_0x002098c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2136268u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098d0));
    }
}
#[inline(always)]
pub fn block_0x002098d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2136276u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2136160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209860));
    } else {
        emu.pc = 2136280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002098d8));
    }
}
#[inline(always)]
pub fn block_0x002098d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2136284u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965308u32, 2136288u32);
    emu.apc_no_count(1usize, 2136288u32, 98304u32, 2136292u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002098e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2136300u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2136304u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2136308u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2136312u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2136316u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2136320u32)?;
    emu.add_memory_rw_events(6usize);
    emu.pc = 2136320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209900));
}
#[inline]
pub fn block_0x00209900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2136324u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2136328u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2136332u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2136336u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2136340u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2136344u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2136348u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2136352u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2136356u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136360u32;
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
pub fn block_0x00209928(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2136364u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2136368u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2136372u32);
    emu.apc_no_count(1usize, 2136372u32, 73728u32, 2136376u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136380u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020993c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2136380u32, 65536u32, 2136384u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2136388u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00209944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2136392u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2136396u32;
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
