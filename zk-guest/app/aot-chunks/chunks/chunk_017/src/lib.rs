pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2158064u32;
pub const PC_MAX: u32 = 2162004u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0020edf0,
        block_0x0020ee00,
        block_0x0020ee20,
        block_0x0020ee40,
        block_0x0020ee60,
        block_0x0020ee78,
        block_0x0020ee80,
        block_0x0020ee98,
        block_0x0020eec0,
        block_0x0020eecc,
        block_0x0020eeec,
        block_0x0020ef04,
        block_0x0020ef0c,
        block_0x0020ef24,
        block_0x0020ef4c,
        block_0x0020ef58,
        block_0x0020ef78,
        block_0x0020ef90,
        block_0x0020ef98,
        block_0x0020efb0,
        block_0x0020efd8,
        block_0x0020efe4,
        block_0x0020f004,
        block_0x0020f01c,
        block_0x0020f024,
        block_0x0020f03c,
        block_0x0020f064,
        block_0x0020f070,
        block_0x0020f094,
        block_0x0020f0b4,
        block_0x0020f0f0,
        block_0x0020f0f4,
        block_0x0020f100,
        block_0x0020f104,
        block_0x0020f128,
        block_0x0020f150,
        block_0x0020f15c,
        block_0x0020f184,
        block_0x0020f1b0,
        block_0x0020f220,
        block_0x0020f228,
        block_0x0020f27c,
        block_0x0020f280,
        block_0x0020f284,
        block_0x0020f2a4,
        block_0x0020f2cc,
        block_0x0020f2dc,
        block_0x0020f2e8,
        block_0x0020f2ec,
        block_0x0020f31c,
        block_0x0020f348,
        block_0x0020f3b8,
        block_0x0020f3c0,
        block_0x0020f414,
        block_0x0020f418,
        block_0x0020f41c,
        block_0x0020f43c,
        block_0x0020f468,
        block_0x0020f478,
        block_0x0020f484,
        block_0x0020f488,
        block_0x0020f4a4,
        block_0x0020f4d0,
        block_0x0020f544,
        block_0x0020f54c,
        block_0x0020f59c,
        block_0x0020f5a0,
        block_0x0020f5a4,
        block_0x0020f5b8,
        block_0x0020f5e8,
        block_0x0020f5f8,
        block_0x0020f5fc,
        block_0x0020f63c,
        block_0x0020f65c,
        block_0x0020f674,
        block_0x0020f69c,
        block_0x0020f6a4,
        block_0x0020f6b4,
        block_0x0020f6d4,
        block_0x0020f6f4,
        block_0x0020f710,
        block_0x0020f770,
        block_0x0020f798,
        block_0x0020f7b4,
        block_0x0020f82c,
        block_0x0020f83c,
        block_0x0020f89c,
        block_0x0020f8a0,
        block_0x0020f8bc,
        block_0x0020f8c8,
        block_0x0020f8d0,
        block_0x0020f8ec,
        block_0x0020f92c,
        block_0x0020f934,
        block_0x0020f954,
        block_0x0020fa94,
        block_0x0020faa4,
        block_0x0020fad0,
        block_0x0020fb0c,
        block_0x0020fb1c,
        block_0x0020fb34,
        block_0x0020fb54,
        block_0x0020fb90,
        block_0x0020fbf0,
        block_0x0020fc20,
        block_0x0020fc78,
        block_0x0020fcd4,
        block_0x0020fcec,
        block_0x0020fd54,
    ];
    const IDX: [u16; 986usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 16u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 27u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 32u16, 0u16,
        0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 54u16, 55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16,
        0u16, 0u16, 59u16, 0u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 68u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 89u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 100u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
    ];
    if pc < 2158064u32 || pc > 2162004u32 {
        return None;
    }
    let word_offset = ((pc - 2158064u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020edf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2158068u32)?;
    emu.lw_no_count(11usize, 10usize, 8u32, 2158072u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2158076u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158080u32;
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
pub fn block_0x0020ee00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2158084u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2158088u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2158092u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2158096u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158100u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2158104u32);
    emu.apc_no_count(6usize, 2158104u32, 24576u32, 2158108u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2158112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ee20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2158116u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2158120u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2158124u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2158128u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2158132u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2158136u32);
    emu.apc_no_count(6usize, 2158136u32, 24576u32, 2158140u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2158144u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ee40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2158148u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2158152u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2158156u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2158160u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2158164u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2158168u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2158172u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2158176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158200u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ee78));
}
#[inline(always)]
pub fn block_0x0020ee60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2158180u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158184u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158188u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158192u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158196u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    } else {
        emu.pc = 2158200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee78));
    }
}
#[inline(always)]
pub fn block_0x0020ee78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2158204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2158176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee60));
    } else {
        emu.pc = 2158208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee80));
    }
}
#[inline(always)]
pub fn block_0x0020ee80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2158212u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158216u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158220u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158224u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158228u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee78));
    } else {
        emu.pc = 2158232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee98));
    }
}
#[inline]
pub fn block_0x0020ee98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2158236u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2158240u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2158244u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158248u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967048u32, 2158252u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2158256u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2158260u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2158264u32);
    emu.apc_no_count(1usize, 2158264u32, 20480u32, 2158268u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020eec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2158276u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2158280u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158284u32;
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
pub fn block_0x0020eecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2158288u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2158292u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2158296u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2158300u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2158304u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2158308u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2158312u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2158316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158340u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ef04));
}
#[inline(always)]
pub fn block_0x0020eeec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2158320u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158324u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158328u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158332u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158336u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef24));
    } else {
        emu.pc = 2158340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef04));
    }
}
#[inline(always)]
pub fn block_0x0020ef04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2158344u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2158316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eeec));
    } else {
        emu.pc = 2158348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef0c));
    }
}
#[inline(always)]
pub fn block_0x0020ef0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2158352u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158356u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158360u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158364u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158368u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef04));
    } else {
        emu.pc = 2158372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef24));
    }
}
#[inline]
pub fn block_0x0020ef24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2158376u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2158380u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2158384u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158388u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967048u32, 2158392u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2158396u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2158400u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2158404u32);
    emu.apc_no_count(1usize, 2158404u32, 20480u32, 2158408u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158412u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965736u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ef4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2158416u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2158420u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158424u32;
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
pub fn block_0x0020ef58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2158428u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2158432u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2158436u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2158440u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2158444u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2158448u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2158452u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2158456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ef90));
}
#[inline(always)]
pub fn block_0x0020ef78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2158460u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158464u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158468u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158472u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158476u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efb0));
    } else {
        emu.pc = 2158480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef90));
    }
}
#[inline(always)]
pub fn block_0x0020ef90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2158484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2158456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef78));
    } else {
        emu.pc = 2158488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef98));
    }
}
#[inline(always)]
pub fn block_0x0020ef98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2158492u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158496u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158500u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158504u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158508u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef90));
    } else {
        emu.pc = 2158512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efb0));
    }
}
#[inline]
pub fn block_0x0020efb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2158516u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2158520u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2158524u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158528u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967048u32, 2158532u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2158536u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2158540u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2158544u32);
    emu.apc_no_count(1usize, 2158544u32, 20480u32, 2158548u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020efd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2158556u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2158560u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158564u32;
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
pub fn block_0x0020efe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2158568u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2158572u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2158576u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2158580u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2158584u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2158588u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2158592u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2158596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f01c));
}
#[inline(always)]
pub fn block_0x0020f004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2158600u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158604u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158608u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158612u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158616u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f03c));
    } else {
        emu.pc = 2158620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f01c));
    }
}
#[inline(always)]
pub fn block_0x0020f01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2158624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2158596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f004));
    } else {
        emu.pc = 2158628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f024));
    }
}
#[inline(always)]
pub fn block_0x0020f024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2158632u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2158636u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2158640u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2158644u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2158648u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f01c));
    } else {
        emu.pc = 2158652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f03c));
    }
}
#[inline]
pub fn block_0x0020f03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2158656u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2158660u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2158664u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158668u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967048u32, 2158672u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2158676u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2158680u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2158684u32);
    emu.apc_no_count(1usize, 2158684u32, 20480u32, 2158688u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2158696u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2158700u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158704u32;
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
pub fn block_0x0020f070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2158708u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2158712u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2158716u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2158720u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2158724u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2158728u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2158732u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2158736u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158740u32;
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
pub fn block_0x0020f094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2158744u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2158748u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2158752u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2158756u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2158760u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158764u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967050u32, 2158768u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2158836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f4));
    } else {
        emu.pc = 2158772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b4));
    }
}
#[inline]
pub fn block_0x0020f0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2158776u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2158780u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2158784u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2158788u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2158792u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2158796u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2158800u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2158804u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2158808u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2158812u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2158816u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2158820u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2158824u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2158828u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2158848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f100));
    } else {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    }
}
#[inline(always)]
pub fn block_0x0020f0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f104));
}
#[inline(always)]
pub fn block_0x0020f0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2158840u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2158844u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2158852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f104));
    } else {
        emu.pc = 2158848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f100));
    }
}
#[inline(always)]
pub fn block_0x0020f100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f128));
    } else {
        emu.pc = 2158852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f104));
    }
}
#[inline]
pub fn block_0x0020f104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2158856u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2158860u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2158864u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2158868u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2158872u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2158876u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2158880u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2158884u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2158888u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2158888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f128));
}
#[inline]
pub fn block_0x0020f128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2158892u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2158896u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2158900u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2158904u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2158908u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2158912u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2158916u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2158920u32);
    emu.apc_no_count(1usize, 2158920u32, 16384u32, 2158924u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158928u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2158932u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2158936u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158940u32;
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
pub fn block_0x0020f15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2158944u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2158948u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2158952u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2158956u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2158960u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2158964u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158968u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967050u32, 2158972u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2158976u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2159324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2dc));
    } else {
        emu.pc = 2158980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f184));
    }
}
#[inline]
pub fn block_0x0020f184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2158984u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158988u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2158992u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2158996u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2159000u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2159004u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2159008u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2159012u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2159016u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2159020u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2159024u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2159024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f1b0));
}
#[inline(never)]
pub fn block_0x0020f1b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2159028u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2159032u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2159036u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2159040u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2159044u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2159048u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2159052u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2159056u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2159060u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2159064u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2159068u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2159072u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2159076u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2159080u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2159084u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2159088u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2159092u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2159096u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2159100u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2159104u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2159108u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2159112u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2159116u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2159120u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2159124u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2159128u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2159132u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2159024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f1b0));
    } else {
        emu.pc = 2159136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f220));
    }
}
#[inline(always)]
pub fn block_0x0020f220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2159140u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2159228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f27c));
    } else {
        emu.pc = 2159144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f228));
    }
}
#[inline]
pub fn block_0x0020f228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2159148u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2159152u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2159156u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2159160u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2159164u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2159168u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2159172u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2159176u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2159180u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2159184u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2159188u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2159192u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2159196u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2159200u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2159204u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2159208u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2159212u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2159216u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2159220u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2159224u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2159228u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2159228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f27c));
}
#[inline(always)]
pub fn block_0x0020f27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2159236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f284));
    } else {
        emu.pc = 2159232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f280));
    }
}
#[inline(always)]
pub fn block_0x0020f280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2159268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2a4));
    } else {
        emu.pc = 2159236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f284));
    }
}
#[inline(always)]
pub fn block_0x0020f284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2159240u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2159244u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2159248u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2159252u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2159256u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2159260u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2159264u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2159268u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2159268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f2a4));
}
#[inline]
pub fn block_0x0020f2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2159272u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2159276u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2159280u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2159284u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2159288u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2159292u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2159296u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2159300u32);
    emu.apc_no_count(1usize, 2159300u32, 16384u32, 2159304u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2159312u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2159316u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2159320u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159324u32;
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
pub fn block_0x0020f2dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2159328u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2159332u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2159144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f228));
    } else {
        emu.pc = 2159336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2e8));
    }
}
#[inline(always)]
pub fn block_0x0020f2e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159340u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f27c));
}
#[inline]
pub fn block_0x0020f2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2159344u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2159348u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2159352u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2159356u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2159360u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2159364u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2159368u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2159372u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2159376u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2159380u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967050u32, 2159384u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2159736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f478));
    } else {
        emu.pc = 2159388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f31c));
    }
}
#[inline]
pub fn block_0x0020f31c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2159392u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2159396u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2159400u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2159404u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159408u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2159412u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2159416u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2159420u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2159424u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2159428u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2159432u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2159432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f348));
}
#[inline(never)]
pub fn block_0x0020f348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2159436u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2159440u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2159444u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2159448u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2159452u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2159456u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2159460u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2159464u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2159468u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2159472u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2159476u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2159480u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2159484u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2159488u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2159492u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2159496u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2159500u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2159504u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2159508u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2159512u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2159516u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2159520u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2159524u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2159528u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2159532u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2159536u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2159540u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2159432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f348));
    } else {
        emu.pc = 2159544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3b8));
    }
}
#[inline(always)]
pub fn block_0x0020f3b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2159548u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2159636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f414));
    } else {
        emu.pc = 2159552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3c0));
    }
}
#[inline]
pub fn block_0x0020f3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2159556u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2159560u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2159564u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2159568u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2159572u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2159576u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2159580u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2159584u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2159588u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2159592u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2159596u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2159600u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2159604u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2159608u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2159612u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2159616u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2159620u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2159624u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2159628u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2159632u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2159636u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2159636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f414));
}
#[inline(always)]
pub fn block_0x0020f414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2159644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f41c));
    } else {
        emu.pc = 2159640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f418));
    }
}
#[inline(always)]
pub fn block_0x0020f418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2159676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f43c));
    } else {
        emu.pc = 2159644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f41c));
    }
}
#[inline(always)]
pub fn block_0x0020f41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2159648u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2159652u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2159656u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2159660u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2159664u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2159668u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2159672u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2159676u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2159676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f43c));
}
#[inline]
pub fn block_0x0020f43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2159680u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2159684u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2159688u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2159692u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2159696u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2159700u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2159704u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2159708u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2159712u32);
    emu.apc_no_count(1usize, 2159712u32, 16384u32, 2159716u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2159724u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2159728u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2159732u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159736u32;
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
pub fn block_0x0020f478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2159740u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2159744u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2159552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3c0));
    } else {
        emu.pc = 2159748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f484));
    }
}
#[inline(always)]
pub fn block_0x0020f484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f414));
}
#[inline(always)]
pub fn block_0x0020f488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2159756u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2159760u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2159764u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2159768u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2159772u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967050u32, 2159776u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2160104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5e8));
    } else {
        emu.pc = 2159780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4a4));
    }
}
#[inline]
pub fn block_0x0020f4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2159784u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2159788u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2159792u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2159796u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2159800u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2159804u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2159808u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2159812u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2159816u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2159820u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2159824u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2159824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4d0));
}
#[inline(never)]
pub fn block_0x0020f4d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2159828u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2159832u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2159836u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2159840u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2159844u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2159848u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2159852u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2159856u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2159860u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2159864u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2159868u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2159872u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2159876u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2159880u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2159884u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2159888u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2159892u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2159896u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2159900u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2159904u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2159908u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2159912u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2159916u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2159920u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2159924u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2159928u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2159932u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2159936u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2159824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4d0));
    } else {
        emu.pc = 2159940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f544));
    }
}
#[inline(always)]
pub fn block_0x0020f544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2159944u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2160028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f59c));
    } else {
        emu.pc = 2159948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f54c));
    }
}
#[inline]
pub fn block_0x0020f54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2159952u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2159956u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2159960u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2159964u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2159968u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2159972u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2159976u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2159980u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2159984u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2159988u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2159992u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2159996u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2160000u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2160004u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2160008u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2160012u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2160016u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2160020u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2160024u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2160028u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2160028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f59c));
}
#[inline(always)]
pub fn block_0x0020f59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5b8));
    } else {
        emu.pc = 2160032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5a0));
    }
}
#[inline(always)]
pub fn block_0x0020f5a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2160056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5b8));
    } else {
        emu.pc = 2160036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5a4));
    }
}
#[inline(always)]
pub fn block_0x0020f5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2160040u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2160044u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2160048u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2160052u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160056u32;
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
pub fn block_0x0020f5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2160060u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2160064u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2160068u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2160072u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2160076u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2160080u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2160084u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160088u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2160092u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2160096u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2160100u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160104u32;
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
pub fn block_0x0020f5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2160108u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2160112u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2160116u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2159948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f54c));
    } else {
        emu.pc = 2160120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5f8));
    }
}
#[inline(always)]
pub fn block_0x0020f5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160028u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f59c));
}
#[inline]
pub fn block_0x0020f5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2160128u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2160132u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2160136u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2160140u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2160144u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2160148u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2160152u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2160156u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2160160u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2160164u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2160168u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2160172u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2160176u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2160180u32);
    emu.apc_no_count(1usize, 2160180u32, 0u32, 2160184u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2160192u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2160196u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2160200u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2160204u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2160208u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2160212u32);
    emu.apc_no_count(1usize, 2160212u32, 16384u32, 2160216u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f65c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2160224u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2160228u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2160232u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2160236u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2160240u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160244u32;
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
pub fn block_0x0020f674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2160248u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2160252u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2160256u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2160260u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2160264u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2160268u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2160272u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2160276u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2160280u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6a4));
    } else {
        emu.pc = 2160284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f69c));
    }
}
#[inline(always)]
pub fn block_0x0020f69c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2160288u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f6b4));
}
#[inline(always)]
pub fn block_0x0020f6a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 0usize, 10usize, 2160296u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2160300u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2160304u32);
    emu.sbr_no_count(11usize, 13usize, 11usize, 2160308u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2160308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f6b4));
}
#[inline(always)]
pub fn block_0x0020f6b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slti_no_count(12usize, 12usize, 0u32, 2160312u32);
    emu.xri_no_count(9usize, 12usize, 1u32, 2160316u32);
    emu.adi_no_count(18usize, 2usize, 8u32, 2160320u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2160324u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2160328u32);
    emu.adi_no_count(19usize, 0usize, 20u32, 2160332u32);
    emu.apc_no_count(1usize, 2160332u32, 0u32, 2160336u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(68u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2160344u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2160348u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2160352u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2160356u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2160360u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2160364u32);
    emu.apc_no_count(1usize, 2160364u32, 16384u32, 2160368u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2160376u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2160380u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2160384u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2160388u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2160392u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2160396u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160400u32;
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
pub fn block_0x0020f710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2160404u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2160408u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2160412u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2160416u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2160420u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2160424u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2160428u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2160432u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2160436u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2160440u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2160444u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2160448u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2160452u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2160456u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2160460u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2160464u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2160468u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2160472u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2160476u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2160480u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2160484u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967050u32, 2160488u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2160492u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2160800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8a0));
    } else {
        emu.pc = 2160496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f770));
    }
}
#[inline]
pub fn block_0x0020f770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2160500u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160504u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2160508u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2160512u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160516u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2160520u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2160524u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2160528u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2160532u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2160536u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2160536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f798));
}
#[inline(always)]
pub fn block_0x0020f798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2160540u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2160544u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2160548u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2160552u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2160556u32);
    emu.apc_no_count(1usize, 2160556u32, 32768u32, 2160560u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160564u32;
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
#[inline(never)]
pub fn block_0x0020f7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2160568u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2160572u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2160576u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2160580u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2160584u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2160588u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2160592u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2160596u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2160600u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2160604u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2160608u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2160612u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2160616u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2160620u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2160624u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2160628u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2160632u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2160636u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2160640u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2160644u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2160648u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2160652u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2160656u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2160660u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2160664u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2160668u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2160672u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2160676u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2160680u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2160536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f798));
    } else {
        emu.pc = 2160684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f82c));
    }
}
#[inline(always)]
pub fn block_0x0020f82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2160688u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2160692u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2160696u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2160828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8bc));
    } else {
        emu.pc = 2160700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f83c));
    }
}
#[inline]
pub fn block_0x0020f83c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2160704u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2160708u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2160712u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2160716u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2160720u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2160724u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2160728u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2160732u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2160736u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2160740u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2160744u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2160748u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2160752u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2160756u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2160760u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2160764u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2160768u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2160772u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2160776u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2160780u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2160784u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2160788u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2160792u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2160840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8c8));
    } else {
        emu.pc = 2160796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f89c));
    }
}
#[inline(always)]
pub fn block_0x0020f89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8d0));
}
#[inline(always)]
pub fn block_0x0020f8a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2160804u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2160808u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2160812u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2160816u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2160820u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2160824u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2160700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f83c));
    } else {
        emu.pc = 2160828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8bc));
    }
}
#[inline(always)]
pub fn block_0x0020f8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2160832u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2160836u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2160848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8d0));
    } else {
        emu.pc = 2160840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8c8));
    }
}
#[inline(always)]
pub fn block_0x0020f8c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2160844u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2160876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8ec));
    } else {
        emu.pc = 2160848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8d0));
    }
}
#[inline(always)]
pub fn block_0x0020f8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2160852u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2160856u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2160860u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2160864u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2160868u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2160872u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2160876u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2160876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8ec));
}
#[inline]
pub fn block_0x0020f8ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2160880u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2160884u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2160888u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2160892u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2160896u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2160900u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2160904u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2160908u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2160912u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2160916u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2160920u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2160924u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2160928u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2160932u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2160936u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160940u32;
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
pub fn block_0x0020f92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 128u32, 2160944u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f954));
    } else {
        emu.pc = 2160948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f934));
    }
}
#[inline(always)]
pub fn block_0x0020f934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967199u32, 2160952u32);
    emu.sltiu_no_count(14usize, 14usize, 26u32, 2160956u32);
    emu.sli_no_count(14usize, 14usize, 5u32, 2160960u32);
    emu.xrr_no_count(11usize, 14usize, 11usize, 2160964u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2160968u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2160972u32)?;
    emu.sw_no_count(0usize, 10usize, 8u32, 2160976u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160980u32;
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
pub fn block_0x0020f954(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 80u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160984u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967194u32, 2160988u32);
    emu.sltru_no_count(12usize, 11usize, 12usize, 2160992u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2160996u32);
    emu.ani_no_count(13usize, 12usize, 763u32, 2161000u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161004u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161008u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967268u32, 2161012u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161016u32);
    emu.adi_no_count(14usize, 14usize, 2047u32, 2161020u32);
    emu.lw_no_count(14usize, 14usize, 1001u32, 2161024u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161028u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161032u32);
    emu.ani_no_count(14usize, 14usize, 381u32, 2161036u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161040u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161044u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161048u32);
    emu.lw_no_count(14usize, 14usize, 1528u32, 2161052u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161056u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161060u32);
    emu.ani_no_count(14usize, 14usize, 191u32, 2161064u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161068u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161072u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161076u32);
    emu.lw_no_count(14usize, 14usize, 760u32, 2161080u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161084u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161088u32);
    emu.ani_no_count(14usize, 14usize, 95u32, 2161092u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161096u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161100u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161104u32);
    emu.lw_no_count(14usize, 14usize, 384u32, 2161108u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161112u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161116u32);
    emu.ani_no_count(14usize, 14usize, 48u32, 2161120u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161124u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161128u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161132u32);
    emu.lw_no_count(14usize, 14usize, 192u32, 2161136u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161140u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161144u32);
    emu.ani_no_count(14usize, 14usize, 24u32, 2161148u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161152u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161156u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161160u32);
    emu.lw_no_count(14usize, 14usize, 96u32, 2161164u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161168u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161172u32);
    emu.ani_no_count(14usize, 14usize, 12u32, 2161176u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161180u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161184u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161188u32);
    emu.lw_no_count(14usize, 14usize, 48u32, 2161192u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161196u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161200u32);
    emu.ani_no_count(14usize, 14usize, 6u32, 2161204u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161208u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161212u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161216u32);
    emu.lw_no_count(14usize, 14usize, 24u32, 2161220u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161224u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2161228u32);
    emu.ani_no_count(14usize, 14usize, 3u32, 2161232u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161236u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161240u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161244u32);
    emu.lw_no_count(14usize, 14usize, 8u32, 2161248u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161252u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2161256u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161260u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161264u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2161268u32);
    emu.lw_no_count(14usize, 14usize, 8u32, 2161272u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2161276u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2161280u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2161284u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2161288u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2161292u32);
    emu.lw_no_count(14usize, 14usize, 0u32, 2161296u32)?;
    emu.add_memory_rw_events(79usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2161420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb0c));
    } else {
        emu.pc = 2161300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa94));
    }
}
#[inline(always)]
pub fn block_0x0020fa94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 14usize, 11usize, 2161304u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2161308u32);
    emu.adi_no_count(11usize, 0usize, 1525u32, 2161312u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2161436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb1c));
    } else {
        emu.pc = 2161316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020faa4));
    }
}
#[inline]
pub fn block_0x0020faa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 3u32, 2161320u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2161324u32);
    emu.lw_no_count(11usize, 12usize, 4u32, 2161328u32)?;
    emu.adi_no_count(12usize, 0usize, 27u32, 2161332u32);
    let a = 0u32.wrapping_add(4293853184u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161336u32;
    emu.update_insn_clock();
    emu.sli_no_count(12usize, 12usize, 11u32, 2161340u32);
    emu.xrr_no_count(12usize, 11usize, 12usize, 2161344u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2161348u32);
    let a = 0u32.wrapping_add(4293857280u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161352u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965248u32, 2161356u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2161420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb0c));
    } else {
        emu.pc = 2161360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fad0));
    }
}
#[inline]
pub fn block_0x0020fad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 10u32, 2161364u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161368u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967188u32, 2161372u32);
    emu.sri_no_count(11usize, 11usize, 10u32, 2161376u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2161380u32);
    emu.sli_no_count(11usize, 11usize, 4u32, 2161384u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2161388u32);
    emu.adr_no_count(13usize, 12usize, 11usize, 2161392u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2161396u32)?;
    emu.lw_no_count(12usize, 13usize, 4u32, 2161400u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2161404u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2161408u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2161412u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2161416u32)?;
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161420u32;
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
pub fn block_0x0020fb0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2161424u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2161428u32)?;
    emu.sw_no_count(0usize, 10usize, 8u32, 2161432u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161436u32;
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
pub fn block_0x0020fb1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161440u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967252u32, 2161444u32);
    emu.adi_no_count(10usize, 0usize, 1526u32, 2161448u32);
    emu.adi_no_count(11usize, 0usize, 1526u32, 2161452u32);
    emu.apc_no_count(1usize, 2161452u32, 0u32, 2161456u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2161464u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2161468u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2161472u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2161476u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2161480u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2161484u32);
    emu.apc_no_count(1usize, 2161484u32, 4294955008u32, 2161488u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2161496u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2161500u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2161504u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2161508u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2161512u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2161516u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2161520u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2161524u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2161528u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2161532u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2161536u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2161540u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161544u32);
    emu.apc_no_count(1usize, 2161544u32, 0u32, 2161548u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161552u32;
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
#[inline]
pub fn block_0x0020fb90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2161556u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2161560u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2161564u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2161568u32);
    let a = 0u32.wrapping_add(2158592u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 348u32, 2161576u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2161580u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161584u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1136u32, 2161588u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2161592u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2161596u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2161600u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2161604u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2161608u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2161612u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2161616u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2161620u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2161624u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2161628u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2161632u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2161636u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161640u32);
    emu.apc_no_count(1usize, 2161640u32, 0u32, 2161644u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161648u32;
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
#[inline]
pub fn block_0x0020fbf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2161652u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2161656u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2161660u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2161664u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2161668u32)?;
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161672u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1152u32, 2161676u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2161680u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2161684u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2161688u32);
    emu.apc_no_count(1usize, 2161688u32, 0u32, 2161692u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161696u32;
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
#[inline]
pub fn block_0x0020fc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2161700u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2161704u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2161708u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2161712u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2161716u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2161720u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2161724u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2161728u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161732u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1476u32, 2161736u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161740u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1488u32, 2161744u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2161748u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2161752u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2161756u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2161760u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2161764u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2161768u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2161772u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2161776u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2161780u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2161876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fcd4));
    } else {
        emu.pc = 2161784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc78));
    }
}
#[inline]
pub fn block_0x0020fc78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2161788u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161792u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 144u32, 2161796u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2161800u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161804u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2161808u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2161812u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2161816u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1212u32, 2161820u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2161824u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2161828u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2161832u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2161836u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2161840u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2161844u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2161848u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2161852u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2161856u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2161860u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2161864u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2161868u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2161872u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2161876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd54));
}
#[inline(always)]
pub fn block_0x0020fcd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2161880u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2161884u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2161888u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2161892u32);
    emu.apc_no_count(1usize, 2161892u32, 4294926336u32, 2161896u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965688u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020fcec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2161904u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161908u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 144u32, 2161912u32);
    let a = 0u32.wrapping_add(2174976u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161916u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1676u32, 2161920u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2161924u32);
    let a = 0u32.wrapping_add(2183168u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161928u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 128u32, 2161932u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2161936u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2161940u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1248u32, 2161944u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2161948u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2161952u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2161956u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2161960u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2161964u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2161968u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2161972u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2161976u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2161980u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2161984u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2161988u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2161992u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2161996u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2162000u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2162004u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2162004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fd54));
}
#[inline(always)]
pub fn block_0x0020fd54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2162008u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2162012u32);
    emu.apc_no_count(1usize, 2162012u32, 0u32, 2162016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
