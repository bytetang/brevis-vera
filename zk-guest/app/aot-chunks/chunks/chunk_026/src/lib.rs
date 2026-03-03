pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2214952u32;
pub const PC_MAX: u32 = 2218636u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x0021cc28,
        block_0x0021cc64,
        block_0x0021cc78,
        block_0x0021cc88,
        block_0x0021cc8c,
        block_0x0021cc9c,
        block_0x0021ccbc,
        block_0x0021ccdc,
        block_0x0021ccfc,
        block_0x0021cd14,
        block_0x0021cd1c,
        block_0x0021cd34,
        block_0x0021cd5c,
        block_0x0021cd68,
        block_0x0021cd88,
        block_0x0021cda0,
        block_0x0021cda8,
        block_0x0021cdc0,
        block_0x0021cde8,
        block_0x0021cdf4,
        block_0x0021ce14,
        block_0x0021ce2c,
        block_0x0021ce34,
        block_0x0021ce4c,
        block_0x0021ce74,
        block_0x0021ce80,
        block_0x0021cea0,
        block_0x0021ceb8,
        block_0x0021cec0,
        block_0x0021ced8,
        block_0x0021cf00,
        block_0x0021cf0c,
        block_0x0021cf30,
        block_0x0021cf50,
        block_0x0021cf8c,
        block_0x0021cf90,
        block_0x0021cf9c,
        block_0x0021cfa0,
        block_0x0021cfc4,
        block_0x0021cfec,
        block_0x0021cff8,
        block_0x0021d020,
        block_0x0021d04c,
        block_0x0021d0bc,
        block_0x0021d0c4,
        block_0x0021d118,
        block_0x0021d11c,
        block_0x0021d120,
        block_0x0021d140,
        block_0x0021d168,
        block_0x0021d178,
        block_0x0021d184,
        block_0x0021d188,
        block_0x0021d1b8,
        block_0x0021d1e4,
        block_0x0021d254,
        block_0x0021d25c,
        block_0x0021d2b0,
        block_0x0021d2b4,
        block_0x0021d2b8,
        block_0x0021d2d8,
        block_0x0021d304,
        block_0x0021d314,
        block_0x0021d320,
        block_0x0021d324,
        block_0x0021d340,
        block_0x0021d36c,
        block_0x0021d3e0,
        block_0x0021d3e8,
        block_0x0021d438,
        block_0x0021d43c,
        block_0x0021d440,
        block_0x0021d454,
        block_0x0021d484,
        block_0x0021d494,
        block_0x0021d498,
        block_0x0021d4d8,
        block_0x0021d4f8,
        block_0x0021d510,
        block_0x0021d538,
        block_0x0021d540,
        block_0x0021d550,
        block_0x0021d570,
        block_0x0021d590,
        block_0x0021d5ac,
        block_0x0021d60c,
        block_0x0021d634,
        block_0x0021d650,
        block_0x0021d6c8,
        block_0x0021d6d8,
        block_0x0021d738,
        block_0x0021d73c,
        block_0x0021d758,
        block_0x0021d764,
        block_0x0021d76c,
        block_0x0021d788,
        block_0x0021d7c8,
        block_0x0021d7d0,
        block_0x0021d7f0,
        block_0x0021d930,
        block_0x0021d940,
        block_0x0021d96c,
        block_0x0021d9a8,
        block_0x0021d9b8,
        block_0x0021d9d0,
        block_0x0021d9f0,
        block_0x0021da2c,
        block_0x0021da8c,
    ];
    const IDX: [u16; 922usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 5u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16,
        0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 20u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 0u16, 0u16,
        37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16,
        0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 58u16, 59u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 72u16, 0u16,
        0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
    ];
    if pc < 2214952u32 || pc > 2218636u32 {
        return None;
    }
    let word_offset = ((pc - 2214952u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021cc28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 25usize, 18u32, 2214956u32);
    emu.sli_no_count(12usize, 25usize, 14u32, 2214960u32);
    emu.sli_no_count(13usize, 25usize, 20u32, 2214964u32);
    emu.ani_no_count(14usize, 25usize, 63u32, 2214968u32);
    emu.adi_no_count(11usize, 11usize, 240u32, 2214972u32);
    emu.sri_no_count(12usize, 12usize, 26u32, 2214976u32);
    emu.sri_no_count(13usize, 13usize, 26u32, 2214980u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2214984u32);
    emu.adi_no_count(12usize, 12usize, 128u32, 2214988u32);
    emu.adi_no_count(13usize, 13usize, 128u32, 2214992u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2214996u32);
    emu.sb_no_count(12usize, 10usize, 1u32, 2215000u32);
    emu.sb_no_count(13usize, 10usize, 2u32, 2215004u32);
    emu.sb_no_count(14usize, 10usize, 3u32, 2215008u32);
    emu.add_memory_rw_events(15usize);
    let return_addr = 2215012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2213276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2213276u32));
}
#[inline(always)]
pub fn block_0x0021cc64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2215016u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2215020u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2215024u32);
    emu.apc_no_count(1usize, 2215024u32, 4294963200u32, 2215028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215032u32;
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
pub fn block_0x0021cc78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2215036u32)?;
    emu.lw_no_count(18usize, 2usize, 24u32, 2215040u32)?;
    emu.adr_no_count(10usize, 18usize, 10usize, 2215044u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a < b {
        emu.pc = 2213268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2213268u32));
    } else {
        emu.pc = 2215048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cc88));
    }
}
#[inline(always)]
pub fn block_0x0021cc88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2214788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2214788u32));
}
#[inline(always)]
pub fn block_0x0021cc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2215056u32)?;
    emu.lw_no_count(11usize, 10usize, 8u32, 2215060u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2215064u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215068u32;
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
pub fn block_0x0021cc9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2215072u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2215076u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2215080u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2215084u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2215088u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2215092u32);
    emu.apc_no_count(6usize, 2215092u32, 24576u32, 2215096u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2215100u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ccbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2215104u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2215108u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2215112u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2215116u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2215120u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2215124u32);
    emu.apc_no_count(6usize, 2215124u32, 24576u32, 2215128u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2215132u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0021ccdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2215136u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2215140u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2215144u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2215148u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2215152u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2215156u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2215160u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2215164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cd14));
}
#[inline(always)]
pub fn block_0x0021ccfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2215168u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215172u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215176u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215180u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215184u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd34));
    } else {
        emu.pc = 2215188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd14));
    }
}
#[inline(always)]
pub fn block_0x0021cd14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2215192u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2215164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ccfc));
    } else {
        emu.pc = 2215196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd1c));
    }
}
#[inline(always)]
pub fn block_0x0021cd1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2215200u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215204u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215208u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215212u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215216u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd14));
    } else {
        emu.pc = 2215220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd34));
    }
}
#[inline]
pub fn block_0x0021cd34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2215224u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2215228u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2215232u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215236u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2215240u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2215244u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2215248u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2215252u32);
    emu.apc_no_count(1usize, 2215252u32, 20480u32, 2215256u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215260u32;
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
pub fn block_0x0021cd5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2215264u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2215268u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215272u32;
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
pub fn block_0x0021cd68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2215276u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2215280u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2215284u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2215288u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2215292u32);
    emu.adi_no_count(11usize, 2usize, 139u32, 2215296u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2215300u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2215304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cda0));
}
#[inline(always)]
pub fn block_0x0021cd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2215308u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215312u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215316u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215320u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215324u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdc0));
    } else {
        emu.pc = 2215328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cda0));
    }
}
#[inline(always)]
pub fn block_0x0021cda0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2215332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2215304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd88));
    } else {
        emu.pc = 2215336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cda8));
    }
}
#[inline(always)]
pub fn block_0x0021cda8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2215340u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215344u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215348u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215352u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215356u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cda0));
    } else {
        emu.pc = 2215360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdc0));
    }
}
#[inline]
pub fn block_0x0021cdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2215364u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2215368u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2215372u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215376u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2215380u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2215384u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2215388u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2215392u32);
    emu.apc_no_count(1usize, 2215392u32, 20480u32, 2215396u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215400u32;
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
pub fn block_0x0021cde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2215404u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2215408u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215412u32;
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
pub fn block_0x0021cdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2215416u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2215420u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2215424u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2215428u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2215432u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2215436u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2215440u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2215444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215468u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ce2c));
}
#[inline(always)]
pub fn block_0x0021ce14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 87u32, 2215448u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215452u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215456u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215460u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215464u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce4c));
    } else {
        emu.pc = 2215468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce2c));
    }
}
#[inline(always)]
pub fn block_0x0021ce2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2215472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2215444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce14));
    } else {
        emu.pc = 2215476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce34));
    }
}
#[inline(always)]
pub fn block_0x0021ce34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2215480u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215484u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215488u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215492u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215496u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce2c));
    } else {
        emu.pc = 2215500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce4c));
    }
}
#[inline]
pub fn block_0x0021ce4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2215504u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2215508u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2215512u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215516u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2215520u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2215524u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2215528u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2215532u32);
    emu.apc_no_count(1usize, 2215532u32, 20480u32, 2215536u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215540u32;
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
pub fn block_0x0021ce74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2215544u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2215548u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215552u32;
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
pub fn block_0x0021ce80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2215556u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2215560u32)?;
    emu.adi_no_count(16usize, 11usize, 0u32, 2215564u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2215568u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2215572u32)?;
    emu.adi_no_count(11usize, 2usize, 139u32, 2215576u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2215580u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2215584u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215608u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ceb8));
}
#[inline(always)]
pub fn block_0x0021cea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 55u32, 2215588u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215592u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215596u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215600u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215604u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ced8));
    } else {
        emu.pc = 2215608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ceb8));
    }
}
#[inline(always)]
pub fn block_0x0021ceb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 10usize, 15u32, 2215612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2215584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cea0));
    } else {
        emu.pc = 2215616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cec0));
    }
}
#[inline(always)]
pub fn block_0x0021cec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 48u32, 2215620u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2215624u32);
    emu.sb_no_count(13usize, 11usize, 0u32, 2215628u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2215632u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215636u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ceb8));
    } else {
        emu.pc = 2215640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ced8));
    }
}
#[inline]
pub fn block_0x0021ced8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2215644u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2215648u32);
    emu.adi_no_count(14usize, 10usize, 128u32, 2215652u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967176u32, 2215660u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2215664u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2215668u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2215672u32);
    emu.apc_no_count(1usize, 2215672u32, 20480u32, 2215676u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215680u32;
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
pub fn block_0x0021cf00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2215684u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2215688u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215692u32;
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
pub fn block_0x0021cf0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2215696u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2215700u32;
    emu.update_insn_clock();
    emu.sbr_no_count(13usize, 13usize, 11usize, 2215704u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2215708u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2215712u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2215716u32);
    emu.xri_no_count(10usize, 10usize, 1u32, 2215720u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2215724u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215728u32;
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
pub fn block_0x0021cf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2215732u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2215736u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2215740u32);
    emu.lbu_no_count(11usize, 10usize, 0u32, 2215744u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2215748u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2215752u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967178u32, 2215756u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2215824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf90));
    } else {
        emu.pc = 2215760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf50));
    }
}
#[inline]
pub fn block_0x0021cf50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2215764u32);
    emu.adi_no_count(14usize, 0usize, 100u32, 2215768u32);
    emu.mul_no_count(12usize, 11usize, 12usize, 2215772u32);
    emu.sri_no_count(12usize, 12usize, 12u32, 2215776u32);
    emu.mul_no_count(14usize, 12usize, 14usize, 2215780u32);
    emu.sbr_no_count(14usize, 11usize, 14usize, 2215784u32);
    emu.sli_no_count(14usize, 14usize, 25u32, 2215788u32);
    emu.sri_no_count(14usize, 14usize, 24u32, 2215792u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2215796u32);
    emu.lbu_no_count(15usize, 14usize, 0u32, 2215800u32);
    emu.lbu_no_count(14usize, 14usize, 1u32, 2215804u32);
    emu.sb_no_count(15usize, 2usize, 10u32, 2215808u32);
    emu.sb_no_count(14usize, 2usize, 11u32, 2215812u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2215816u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2215836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf9c));
    } else {
        emu.pc = 2215820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf8c));
    }
}
#[inline(always)]
pub fn block_0x0021cf8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cfa0));
}
#[inline(always)]
pub fn block_0x0021cf90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 3u32, 2215828u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2215832u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2215840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cfa0));
    } else {
        emu.pc = 2215836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf9c));
    }
}
#[inline(always)]
pub fn block_0x0021cf9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cfc4));
    } else {
        emu.pc = 2215840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cfa0));
    }
}
#[inline]
pub fn block_0x0021cfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2215844u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2215848u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2215852u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2215856u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2215860u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2215864u32);
    emu.adi_no_count(11usize, 2usize, 9u32, 2215868u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2215872u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2215876u32);
    emu.add_memory_rw_events(9usize);
    emu.pc = 2215876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cfc4));
}
#[inline]
pub fn block_0x0021cfc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2215880u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2215884u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2215888u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2215892u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2215896u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2215900u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2215904u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2215908u32);
    emu.apc_no_count(1usize, 2215908u32, 16384u32, 2215912u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215916u32;
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
pub fn block_0x0021cfec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2215920u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2215924u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215928u32;
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
pub fn block_0x0021cff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2215932u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2215936u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2215940u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2215944u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2215948u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2215952u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2215956u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967178u32, 2215960u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2215964u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2216312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d178));
    } else {
        emu.pc = 2215968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d020));
    }
}
#[inline]
pub fn block_0x0021d020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2215972u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215976u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2215980u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2215984u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2215988u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2215992u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2215996u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2216000u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2216004u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2216008u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2216012u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2216012u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d04c));
}
#[inline(never)]
pub fn block_0x0021d04c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2216016u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2216020u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2216024u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2216028u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2216032u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2216036u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2216040u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2216044u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2216048u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2216052u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2216056u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2216060u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2216064u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2216068u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2216072u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2216076u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2216080u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2216084u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2216088u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2216092u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2216096u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2216100u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2216104u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2216108u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2216112u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2216116u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2216120u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2216012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d04c));
    } else {
        emu.pc = 2216124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0bc));
    }
}
#[inline(always)]
pub fn block_0x0021d0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2216128u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2216216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d118));
    } else {
        emu.pc = 2216132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0c4));
    }
}
#[inline]
pub fn block_0x0021d0c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2216136u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2216140u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2216144u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2216148u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2216152u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2216156u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2216160u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2216164u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2216168u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2216172u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2216176u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2216180u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2216184u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2216188u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2216192u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2216196u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2216200u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2216204u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2216208u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2216212u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2216216u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2216216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d118));
}
#[inline(always)]
pub fn block_0x0021d118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d120));
    } else {
        emu.pc = 2216220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d11c));
    }
}
#[inline(always)]
pub fn block_0x0021d11c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d140));
    } else {
        emu.pc = 2216224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d120));
    }
}
#[inline(always)]
pub fn block_0x0021d120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2216228u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2216232u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2216236u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2216240u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2216244u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2216248u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2216252u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2216256u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2216256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d140));
}
#[inline]
pub fn block_0x0021d140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2216260u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2216264u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2216268u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2216272u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2216276u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2216280u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2216284u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2216288u32);
    emu.apc_no_count(1usize, 2216288u32, 16384u32, 2216292u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216296u32;
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
pub fn block_0x0021d168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2216300u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2216304u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2216308u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216312u32;
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
pub fn block_0x0021d178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2216316u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2216320u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2216132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0c4));
    } else {
        emu.pc = 2216324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d184));
    }
}
#[inline(always)]
pub fn block_0x0021d184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2216328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d118));
}
#[inline]
pub fn block_0x0021d188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2216332u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2216336u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2216340u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2216344u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2216348u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2216352u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2216356u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2216360u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2216364u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2216368u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967178u32, 2216372u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2216724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d314));
    } else {
        emu.pc = 2216376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1b8));
    }
}
#[inline]
pub fn block_0x0021d1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2216380u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2216384u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2216388u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2216392u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2216396u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2216400u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2216404u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2216408u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2216412u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2216416u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2216420u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2216420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d1e4));
}
#[inline(never)]
pub fn block_0x0021d1e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2216424u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2216428u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2216432u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2216436u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2216440u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2216444u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2216448u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2216452u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2216456u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2216460u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2216464u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2216468u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2216472u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2216476u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2216480u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2216484u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2216488u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2216492u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2216496u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2216500u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2216504u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2216508u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2216512u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2216516u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2216520u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2216524u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2216528u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2216420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1e4));
    } else {
        emu.pc = 2216532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d254));
    }
}
#[inline(always)]
pub fn block_0x0021d254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2216536u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2216624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2b0));
    } else {
        emu.pc = 2216540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d25c));
    }
}
#[inline]
pub fn block_0x0021d25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2216544u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2216548u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2216552u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2216556u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2216560u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2216564u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2216568u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2216572u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2216576u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2216580u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2216584u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2216588u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2216592u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2216596u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2216600u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2216604u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2216608u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2216612u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2216616u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2216620u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2216624u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2216624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d2b0));
}
#[inline(always)]
pub fn block_0x0021d2b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2b8));
    } else {
        emu.pc = 2216628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2b4));
    }
}
#[inline(always)]
pub fn block_0x0021d2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2d8));
    } else {
        emu.pc = 2216632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2b8));
    }
}
#[inline(always)]
pub fn block_0x0021d2b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2216636u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2216640u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2216644u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2216648u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2216652u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2216656u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2216660u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2216664u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2216664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d2d8));
}
#[inline]
pub fn block_0x0021d2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2216668u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2216672u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2216676u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2216680u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2216684u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2216688u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2216692u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2216696u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2216700u32);
    emu.apc_no_count(1usize, 2216700u32, 16384u32, 2216704u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216708u32;
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
pub fn block_0x0021d304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2216712u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2216716u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2216720u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216724u32;
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
pub fn block_0x0021d314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2216728u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2216732u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2216540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d25c));
    } else {
        emu.pc = 2216736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d320));
    }
}
#[inline(always)]
pub fn block_0x0021d320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2216740u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d2b0));
}
#[inline(always)]
pub fn block_0x0021d324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2216744u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2216748u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2216752u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2216756u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2216760u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967178u32, 2216764u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2217092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d484));
    } else {
        emu.pc = 2216768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d340));
    }
}
#[inline]
pub fn block_0x0021d340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2216772u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2216776u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2216780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2216784u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2216788u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2216792u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2216796u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2216800u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2216804u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2216808u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2216812u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2216812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d36c));
}
#[inline(never)]
pub fn block_0x0021d36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2216816u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2216820u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2216824u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2216828u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2216832u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2216836u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2216840u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2216844u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2216848u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2216852u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2216856u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2216860u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2216864u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2216868u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2216872u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2216876u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2216880u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2216884u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2216888u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2216892u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2216896u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2216900u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2216904u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2216908u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2216912u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2216916u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2216920u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2216924u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2216812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d36c));
    } else {
        emu.pc = 2216928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3e0));
    }
}
#[inline(always)]
pub fn block_0x0021d3e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2216932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2217016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d438));
    } else {
        emu.pc = 2216936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3e8));
    }
}
#[inline]
pub fn block_0x0021d3e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2216940u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2216944u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2216948u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2216952u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2216956u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2216960u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2216964u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2216968u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2216972u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2216976u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2216980u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2216984u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2216988u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2216992u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2216996u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2217000u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2217004u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2217008u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2217012u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2217016u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2217016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d438));
}
#[inline(always)]
pub fn block_0x0021d438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d454));
    } else {
        emu.pc = 2217020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d43c));
    }
}
#[inline(always)]
pub fn block_0x0021d43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2217044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d454));
    } else {
        emu.pc = 2217024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d440));
    }
}
#[inline(always)]
pub fn block_0x0021d440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2217028u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2217032u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2217036u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2217040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217044u32;
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
pub fn block_0x0021d454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2217048u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2217052u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2217056u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2217060u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2217064u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2217068u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2217072u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2217076u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2217080u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2217084u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2217088u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217092u32;
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
pub fn block_0x0021d484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2217096u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2217100u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2217104u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2216936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d3e8));
    } else {
        emu.pc = 2217108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d494));
    }
}
#[inline(always)]
pub fn block_0x0021d494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d438));
}
#[inline]
pub fn block_0x0021d498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2217116u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2217120u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2217124u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2217128u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2217132u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2217136u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2217140u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2217144u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2217148u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2217152u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2217156u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2217160u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2217164u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2217168u32);
    emu.apc_no_count(1usize, 2217168u32, 0u32, 2217172u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217176u32;
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
pub fn block_0x0021d4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2217180u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2217184u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2217188u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2217192u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2217196u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2217200u32);
    emu.apc_no_count(1usize, 2217200u32, 16384u32, 2217204u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217208u32;
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
pub fn block_0x0021d4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2217212u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2217216u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2217220u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2217224u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2217228u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217232u32;
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
pub fn block_0x0021d510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2217236u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2217240u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2217244u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2217248u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2217252u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2217256u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2217260u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2217264u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2217268u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2217280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d540));
    } else {
        emu.pc = 2217272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d538));
    }
}
#[inline(always)]
pub fn block_0x0021d538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2217276u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2217280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d550));
}
#[inline(always)]
pub fn block_0x0021d540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 0usize, 10usize, 2217284u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2217288u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2217292u32);
    emu.sbr_no_count(11usize, 13usize, 11usize, 2217296u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2217296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d550));
}
#[inline(always)]
pub fn block_0x0021d550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slti_no_count(12usize, 12usize, 0u32, 2217300u32);
    emu.xri_no_count(9usize, 12usize, 1u32, 2217304u32);
    emu.adi_no_count(18usize, 2usize, 8u32, 2217308u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2217312u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2217316u32);
    emu.adi_no_count(19usize, 0usize, 20u32, 2217320u32);
    emu.apc_no_count(1usize, 2217320u32, 0u32, 2217324u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217328u32;
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
pub fn block_0x0021d570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2217332u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2217336u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2217340u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2217344u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2217348u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2217352u32);
    emu.apc_no_count(1usize, 2217352u32, 16384u32, 2217356u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217360u32;
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
pub fn block_0x0021d590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2217364u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2217368u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2217372u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2217376u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2217380u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2217384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217388u32;
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
pub fn block_0x0021d5ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2217392u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2217396u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2217400u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2217404u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2217408u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2217412u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2217416u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2217420u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2217424u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2217428u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2217432u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2217436u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2217440u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2217444u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2217448u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2217452u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2217456u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2217460u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2217464u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2217468u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2217472u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967178u32, 2217476u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2217480u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2217788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d73c));
    } else {
        emu.pc = 2217484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d60c));
    }
}
#[inline]
pub fn block_0x0021d60c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2217488u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2217492u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2217496u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2217500u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2217504u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2217508u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2217512u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2217516u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2217520u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2217524u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2217524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d634));
}
#[inline(always)]
pub fn block_0x0021d634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2217528u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2217532u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2217536u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2217540u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2217544u32);
    emu.apc_no_count(1usize, 2217544u32, 32768u32, 2217548u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021d650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2217556u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2217560u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2217564u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2217568u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2217572u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2217576u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2217580u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2217584u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2217588u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2217592u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2217596u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2217600u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2217604u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2217608u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2217612u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2217616u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2217620u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2217624u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2217628u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2217632u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2217636u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2217640u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2217644u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2217648u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2217652u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2217656u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2217660u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2217664u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2217668u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2217524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d634));
    } else {
        emu.pc = 2217672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d6c8));
    }
}
#[inline(always)]
pub fn block_0x0021d6c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2217676u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2217680u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2217684u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2217816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d758));
    } else {
        emu.pc = 2217688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d6d8));
    }
}
#[inline]
pub fn block_0x0021d6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2217692u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2217696u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2217700u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2217704u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2217708u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2217712u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2217716u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2217720u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2217724u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2217728u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2217732u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2217736u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2217740u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2217744u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2217748u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2217752u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2217756u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2217760u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2217764u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2217768u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2217772u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2217776u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2217780u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2217828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d764));
    } else {
        emu.pc = 2217784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d738));
    }
}
#[inline(always)]
pub fn block_0x0021d738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2217788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2217836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d76c));
}
#[inline(always)]
pub fn block_0x0021d73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2217792u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2217796u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2217800u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2217804u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2217808u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2217812u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2217688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d6d8));
    } else {
        emu.pc = 2217816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d758));
    }
}
#[inline(always)]
pub fn block_0x0021d758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2217820u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2217824u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2217836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d76c));
    } else {
        emu.pc = 2217828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d764));
    }
}
#[inline(always)]
pub fn block_0x0021d764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2217832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2217864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d788));
    } else {
        emu.pc = 2217836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d76c));
    }
}
#[inline(always)]
pub fn block_0x0021d76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2217840u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2217844u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2217848u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2217852u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2217856u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2217860u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2217864u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2217864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d788));
}
#[inline]
pub fn block_0x0021d788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2217868u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2217872u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2217876u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2217880u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2217884u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2217888u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2217892u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2217896u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2217900u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2217904u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2217908u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2217912u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2217916u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2217920u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2217924u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217928u32;
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
pub fn block_0x0021d7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 128u32, 2217932u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2217968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d7f0));
    } else {
        emu.pc = 2217936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d7d0));
    }
}
#[inline(always)]
pub fn block_0x0021d7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967199u32, 2217940u32);
    emu.sltiu_no_count(14usize, 14usize, 26u32, 2217944u32);
    emu.sli_no_count(14usize, 14usize, 5u32, 2217948u32);
    emu.xrr_no_count(11usize, 14usize, 11usize, 2217952u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2217956u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2217960u32)?;
    emu.sw_no_count(0usize, 10usize, 8u32, 2217964u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2217968u32;
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
pub fn block_0x0021d7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 80u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2217972u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967194u32, 2217976u32);
    emu.sltru_no_count(12usize, 11usize, 12usize, 2217980u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2217984u32);
    emu.ani_no_count(13usize, 12usize, 763u32, 2217988u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2217992u32);
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2217996u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 100u32, 2218000u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218004u32);
    emu.adi_no_count(14usize, 14usize, 2047u32, 2218008u32);
    emu.lw_no_count(14usize, 14usize, 1001u32, 2218012u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218016u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218020u32);
    emu.ani_no_count(14usize, 14usize, 381u32, 2218024u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218028u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218032u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218036u32);
    emu.lw_no_count(14usize, 14usize, 1528u32, 2218040u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218044u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218048u32);
    emu.ani_no_count(14usize, 14usize, 191u32, 2218052u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218056u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218060u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218064u32);
    emu.lw_no_count(14usize, 14usize, 760u32, 2218068u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218072u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218076u32);
    emu.ani_no_count(14usize, 14usize, 95u32, 2218080u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218084u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218088u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218092u32);
    emu.lw_no_count(14usize, 14usize, 384u32, 2218096u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218100u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218104u32);
    emu.ani_no_count(14usize, 14usize, 48u32, 2218108u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218112u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218116u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218120u32);
    emu.lw_no_count(14usize, 14usize, 192u32, 2218124u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218128u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218132u32);
    emu.ani_no_count(14usize, 14usize, 24u32, 2218136u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218140u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218144u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218148u32);
    emu.lw_no_count(14usize, 14usize, 96u32, 2218152u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218156u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218160u32);
    emu.ani_no_count(14usize, 14usize, 12u32, 2218164u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218168u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218172u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218176u32);
    emu.lw_no_count(14usize, 14usize, 48u32, 2218180u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218184u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218188u32);
    emu.ani_no_count(14usize, 14usize, 6u32, 2218192u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218196u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218200u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218204u32);
    emu.lw_no_count(14usize, 14usize, 24u32, 2218208u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218212u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2218216u32);
    emu.ani_no_count(14usize, 14usize, 3u32, 2218220u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218224u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218228u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218232u32);
    emu.lw_no_count(14usize, 14usize, 8u32, 2218236u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218240u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2218244u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218248u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218252u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2218256u32);
    emu.lw_no_count(14usize, 14usize, 8u32, 2218260u32)?;
    emu.sltru_no_count(14usize, 11usize, 14usize, 2218264u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2218268u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2218272u32);
    emu.sli_no_count(14usize, 13usize, 3u32, 2218276u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2218280u32);
    emu.lw_no_count(14usize, 14usize, 0u32, 2218284u32)?;
    emu.add_memory_rw_events(79usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2218408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9a8));
    } else {
        emu.pc = 2218288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d930));
    }
}
#[inline(always)]
pub fn block_0x0021d930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 14usize, 11usize, 2218292u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2218296u32);
    emu.adi_no_count(11usize, 0usize, 1525u32, 2218300u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2218424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9b8));
    } else {
        emu.pc = 2218304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d940));
    }
}
#[inline]
pub fn block_0x0021d940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 3u32, 2218308u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2218312u32);
    emu.lw_no_count(11usize, 12usize, 4u32, 2218316u32)?;
    emu.adi_no_count(12usize, 0usize, 27u32, 2218320u32);
    let a = 0u32.wrapping_add(4293853184u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2218324u32;
    emu.update_insn_clock();
    emu.sli_no_count(12usize, 12usize, 11u32, 2218328u32);
    emu.xrr_no_count(12usize, 11usize, 12usize, 2218332u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2218336u32);
    let a = 0u32.wrapping_add(4293857280u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2218340u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965248u32, 2218344u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2218408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d9a8));
    } else {
        emu.pc = 2218348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d96c));
    }
}
#[inline]
pub fn block_0x0021d96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 10u32, 2218352u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2218356u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 20u32, 2218360u32);
    emu.sri_no_count(11usize, 11usize, 10u32, 2218364u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2218368u32);
    emu.sli_no_count(11usize, 11usize, 4u32, 2218372u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2218376u32);
    emu.adr_no_count(13usize, 12usize, 11usize, 2218380u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2218384u32)?;
    emu.lw_no_count(12usize, 13usize, 4u32, 2218388u32)?;
    emu.lw_no_count(13usize, 13usize, 8u32, 2218392u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2218396u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2218400u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2218404u32)?;
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218408u32;
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
pub fn block_0x0021d9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2218412u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2218416u32)?;
    emu.sw_no_count(0usize, 10usize, 8u32, 2218420u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218424u32;
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
pub fn block_0x0021d9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2269184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2218428u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 84u32, 2218432u32);
    emu.adi_no_count(10usize, 0usize, 1526u32, 2218436u32);
    emu.adi_no_count(11usize, 0usize, 1526u32, 2218440u32);
    emu.apc_no_count(1usize, 2218440u32, 0u32, 2218444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218448u32;
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
pub fn block_0x0021d9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2218452u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2218456u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2218460u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2218464u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2218468u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2218472u32);
    emu.apc_no_count(1usize, 2218472u32, 4294955008u32, 2218476u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218480u32;
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
pub fn block_0x0021d9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2218484u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2218488u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2218492u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2218496u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2218500u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2218504u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2218508u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2218512u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2218516u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2218520u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2218524u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2218528u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2218532u32);
    emu.apc_no_count(1usize, 2218532u32, 0u32, 2218536u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218540u32;
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
pub fn block_0x0021da2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2218544u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2218548u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2218552u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2218556u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218560u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2218564u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2218568u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2218572u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1264u32, 2218576u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2218580u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2218584u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2218588u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2218592u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2218596u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2218600u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2218604u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2218608u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2218612u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2218616u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2218620u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2218624u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2218628u32);
    emu.apc_no_count(1usize, 2218628u32, 0u32, 2218632u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218636u32;
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
pub fn block_0x0021da8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2218640u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2218644u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2218648u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2218652u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2218656u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2218660u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1280u32, 2218664u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2218668u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2218672u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2218676u32);
    emu.apc_no_count(1usize, 2218676u32, 0u32, 2218680u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218684u32;
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
