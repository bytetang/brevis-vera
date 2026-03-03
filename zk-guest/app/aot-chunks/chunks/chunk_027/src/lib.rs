pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2218684u32;
pub const PC_MAX: u32 = 2221316u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 138usize] = [
        block_0x0021dabc,
        block_0x0021db14,
        block_0x0021db70,
        block_0x0021db88,
        block_0x0021dbf0,
        block_0x0021dc00,
        block_0x0021dc08,
        block_0x0021dc1c,
        block_0x0021dc30,
        block_0x0021dc38,
        block_0x0021dc44,
        block_0x0021dc4c,
        block_0x0021dc5c,
        block_0x0021dc64,
        block_0x0021dc6c,
        block_0x0021dc74,
        block_0x0021dc80,
        block_0x0021dc84,
        block_0x0021dc88,
        block_0x0021dc8c,
        block_0x0021dc90,
        block_0x0021dca0,
        block_0x0021dca4,
        block_0x0021dcac,
        block_0x0021dcb0,
        block_0x0021dcb4,
        block_0x0021dcc0,
        block_0x0021dcc8,
        block_0x0021dccc,
        block_0x0021dcd8,
        block_0x0021dce8,
        block_0x0021dcf4,
        block_0x0021dd08,
        block_0x0021dd20,
        block_0x0021dd44,
        block_0x0021dd48,
        block_0x0021dd50,
        block_0x0021dd5c,
        block_0x0021dd64,
        block_0x0021dd78,
        block_0x0021dd80,
        block_0x0021dd88,
        block_0x0021dd94,
        block_0x0021de30,
        block_0x0021de34,
        block_0x0021dea0,
        block_0x0021df14,
        block_0x0021df3c,
        block_0x0021df48,
        block_0x0021df50,
        block_0x0021df78,
        block_0x0021df7c,
        block_0x0021df98,
        block_0x0021dfa8,
        block_0x0021dfbc,
        block_0x0021dfc0,
        block_0x0021dfcc,
        block_0x0021dfd8,
        block_0x0021dff8,
        block_0x0021e000,
        block_0x0021e00c,
        block_0x0021e014,
        block_0x0021e020,
        block_0x0021e048,
        block_0x0021e05c,
        block_0x0021e060,
        block_0x0021e084,
        block_0x0021e0a0,
        block_0x0021e0b8,
        block_0x0021e0d0,
        block_0x0021e0e8,
        block_0x0021e114,
        block_0x0021e14c,
        block_0x0021e158,
        block_0x0021e15c,
        block_0x0021e17c,
        block_0x0021e190,
        block_0x0021e194,
        block_0x0021e1e8,
        block_0x0021e1f0,
        block_0x0021e1f8,
        block_0x0021e208,
        block_0x0021e210,
        block_0x0021e218,
        block_0x0021e220,
        block_0x0021e228,
        block_0x0021e22c,
        block_0x0021e240,
        block_0x0021e248,
        block_0x0021e24c,
        block_0x0021e254,
        block_0x0021e278,
        block_0x0021e290,
        block_0x0021e328,
        block_0x0021e33c,
        block_0x0021e358,
        block_0x0021e360,
        block_0x0021e368,
        block_0x0021e36c,
        block_0x0021e374,
        block_0x0021e37c,
        block_0x0021e38c,
        block_0x0021e390,
        block_0x0021e39c,
        block_0x0021e3a4,
        block_0x0021e3b0,
        block_0x0021e3b4,
        block_0x0021e3c0,
        block_0x0021e3c8,
        block_0x0021e3d0,
        block_0x0021e404,
        block_0x0021e40c,
        block_0x0021e410,
        block_0x0021e418,
        block_0x0021e424,
        block_0x0021e42c,
        block_0x0021e434,
        block_0x0021e438,
        block_0x0021e448,
        block_0x0021e450,
        block_0x0021e45c,
        block_0x0021e460,
        block_0x0021e464,
        block_0x0021e470,
        block_0x0021e474,
        block_0x0021e484,
        block_0x0021e494,
        block_0x0021e4a8,
        block_0x0021e4ac,
        block_0x0021e4b0,
        block_0x0021e4b4,
        block_0x0021e4cc,
        block_0x0021e4e8,
        block_0x0021e4ec,
        block_0x0021e4f0,
        block_0x0021e4f8,
        block_0x0021e500,
        block_0x0021e504,
    ];
    const IDX: [u16; 659usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16,
        0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 17u16, 18u16, 19u16,
        20u16, 21u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 24u16, 25u16, 26u16, 0u16,
        0u16, 27u16, 0u16, 28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 0u16, 37u16,
        0u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16,
        42u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 49u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16, 56u16,
        0u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        59u16, 0u16, 60u16, 0u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16,
        65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 74u16, 75u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16,
        0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 85u16, 0u16, 86u16,
        87u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        97u16, 0u16, 98u16, 99u16, 0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16,
        103u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16,
        108u16, 0u16, 109u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 113u16, 0u16, 114u16, 0u16,
        0u16, 115u16, 0u16, 116u16, 0u16, 117u16, 118u16, 0u16, 0u16, 0u16, 119u16, 0u16,
        120u16, 0u16, 0u16, 121u16, 122u16, 123u16, 0u16, 0u16, 124u16, 125u16, 0u16,
        0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 128u16,
        129u16, 130u16, 131u16, 0u16, 0u16, 0u16, 0u16, 0u16, 132u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 133u16, 134u16, 135u16, 0u16, 136u16, 0u16, 137u16, 138u16,
    ];
    if pc < 2218684u32 || pc > 2221316u32 {
        return None;
    }
    let word_offset = ((pc - 2218684u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021dabc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2218688u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2218692u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2218696u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2218700u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2218704u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2218708u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2218712u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2218716u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218720u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1604u32, 2218724u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2218728u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1616u32, 2218732u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2218736u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2218740u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2218744u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2218748u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2218752u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2218756u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2218760u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2218764u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2218768u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2218864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021db70));
    } else {
        emu.pc = 2218772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021db14));
    }
}
#[inline]
pub fn block_0x0021db14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2218776u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 24u32, 2218784u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2218788u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2218792u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 8u32, 2218796u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2218800u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2218804u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1340u32, 2218808u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2218812u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2218816u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2218820u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2218824u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2218828u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2218832u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2218836u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2218840u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2218844u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2218848u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2218852u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2218856u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2218860u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2218864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2218992u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbf0));
}
#[inline(always)]
pub fn block_0x0021db70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2218868u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2218872u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2218876u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2218880u32);
    emu.apc_no_count(1usize, 2218880u32, 4294889472u32, 2218884u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2218888u32;
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
#[inline(never)]
pub fn block_0x0021db88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2218892u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2218896u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 24u32, 2218900u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2218904u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1320u32, 2218908u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2218912u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2218916u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 8u32, 2218920u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2218924u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2218928u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1376u32, 2218932u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2218936u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2218940u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2218944u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2218948u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2218952u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2218956u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2218960u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2218964u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2218968u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2218972u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2218976u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2218980u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2218984u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2218988u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2218992u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2218992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dbf0));
}
#[inline(always)]
pub fn block_0x0021dbf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2218996u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2219000u32);
    emu.apc_no_count(1usize, 2219000u32, 0u32, 2219004u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219008u32;
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
#[inline(always)]
pub fn block_0x0021dc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2219008u32, 0u32, 2219012u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219016u32;
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
pub fn block_0x0021dc08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2219020u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2219024u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2219028u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2219032u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2219056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc30));
    } else {
        emu.pc = 2219036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc1c));
    }
}
#[inline(always)]
pub fn block_0x0021dc1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2219040u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2219044u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2219048u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2219052u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2219056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219124u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc74));
}
#[inline(always)]
pub fn block_0x0021dc30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2219060u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2219064u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2219064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc38));
}
#[inline(always)]
pub fn block_0x0021dc38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2219068u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2219072u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc4c));
    } else {
        emu.pc = 2219076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc44));
    }
}
#[inline(always)]
pub fn block_0x0021dc44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2219080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2219064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc38));
    } else {
        emu.pc = 2219084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc4c));
    }
}
#[inline(always)]
pub fn block_0x0021dc4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2219088u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2219092u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2219096u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2219108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc64));
    } else {
        emu.pc = 2219100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc5c));
    }
}
#[inline(always)]
pub fn block_0x0021dc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2219104u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc6c));
}
#[inline(always)]
pub fn block_0x0021dc64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2219112u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1408u32, 2219116u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2219116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc6c));
}
#[inline(always)]
pub fn block_0x0021dc6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2219120u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2219124u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2219124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dc74));
}
#[inline(always)]
pub fn block_0x0021dc74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2219128u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2219132u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2219572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de34));
    } else {
        emu.pc = 2219136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc80));
    }
}
#[inline(always)]
pub fn block_0x0021dc80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2219568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021de30));
    } else {
        emu.pc = 2219140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc84));
    }
}
#[inline(always)]
pub fn block_0x0021dc84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2219680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dea0));
    } else {
        emu.pc = 2219144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc88));
    }
}
#[inline(always)]
pub fn block_0x0021dc88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dca4));
    } else {
        emu.pc = 2219148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc8c));
    }
}
#[inline(always)]
pub fn block_0x0021dc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2219172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dca4));
    } else {
        emu.pc = 2219152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dc90));
    }
}
#[inline(always)]
pub fn block_0x0021dc90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2219156u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2219160u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2219164u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dca4));
    } else {
        emu.pc = 2219168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dca0));
    }
}
#[inline(always)]
pub fn block_0x0021dca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 12usize, 0u32, 2219172u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dca4));
}
#[inline(always)]
pub fn block_0x0021dca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2219176u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2219212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dccc));
    } else {
        emu.pc = 2219180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcac));
    }
}
#[inline(always)]
pub fn block_0x0021dcac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2219208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcc8));
    } else {
        emu.pc = 2219184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcb0));
    }
}
#[inline(always)]
pub fn block_0x0021dcb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2219188u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dcb4));
}
#[inline(always)]
pub fn block_0x0021dcb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2219192u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2219196u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcc8));
    } else {
        emu.pc = 2219200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcc0));
    }
}
#[inline(always)]
pub fn block_0x0021dcc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2219204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2219188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcb4));
    } else {
        emu.pc = 2219208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcc8));
    }
}
#[inline(always)]
pub fn block_0x0021dcc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2219224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcd8));
    } else {
        emu.pc = 2219212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dccc));
    }
}
#[inline(always)]
pub fn block_0x0021dccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2219216u32);
    emu.apc_no_count(1usize, 2219216u32, 8192u32, 2219220u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021dcd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2219228u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2219232u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2219236u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2219252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dcf4));
    } else {
        emu.pc = 2219240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dce8));
    }
}
#[inline(always)]
pub fn block_0x0021dce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2219244u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2219248u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2219252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd94));
}
#[inline(always)]
pub fn block_0x0021dcf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2219256u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2219260u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2219264u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2219268u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2219336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd48));
    } else {
        emu.pc = 2219272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd08));
    }
}
#[inline(always)]
pub fn block_0x0021dd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2219276u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2219280u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2219284u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2219288u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2219292u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2219364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd64));
    } else {
        emu.pc = 2219296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd20));
    }
}
#[inline]
pub fn block_0x0021dd20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2219300u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2219304u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2219308u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2219312u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2219316u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2219320u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2219324u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219328u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2219212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dccc));
    } else {
        emu.pc = 2219332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd44));
    }
}
#[inline(always)]
pub fn block_0x0021dd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2219336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219344u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd50));
}
#[inline(always)]
pub fn block_0x0021dd48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2219340u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2219344u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2219344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd50));
}
#[inline(always)]
pub fn block_0x0021dd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2219348u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2219352u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2219384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd78));
    } else {
        emu.pc = 2219356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd5c));
    }
}
#[inline(always)]
pub fn block_0x0021dd5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2219360u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd94));
}
#[inline(always)]
pub fn block_0x0021dd64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2219368u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2219372u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2219376u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2219380u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2219356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd5c));
    } else {
        emu.pc = 2219384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd78));
    }
}
#[inline(always)]
pub fn block_0x0021dd78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2219388u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2219400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd88));
    } else {
        emu.pc = 2219392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dd80));
    }
}
#[inline(always)]
pub fn block_0x0021dd80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2219396u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2219400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2219412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd94));
}
#[inline(always)]
pub fn block_0x0021dd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2219404u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2219408u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2219412u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2219412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dd94));
}
#[inline(never)]
pub fn block_0x0021dd94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2219416u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2219420u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2219424u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2219428u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2219432u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2219436u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966460u32, 2219440u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2219444u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2219448u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 4294966220u32, 2219452u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2219456u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2219460u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 24u32, 2219464u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2219468u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2219472u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2219476u32)?;
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2219480u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1516u32, 2219484u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2219488u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2219492u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2219496u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2219500u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2219504u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2219508u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2219512u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2219516u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2219520u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2219524u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2219528u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2219532u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2219536u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2219540u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2219544u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2219548u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2219552u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2219556u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2219560u32);
    emu.apc_no_count(1usize, 2219560u32, 0u32, 2219564u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219568u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021de30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2219572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021de34));
}
#[inline(never)]
pub fn block_0x0021de34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2219576u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2219580u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219584u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2219588u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2219592u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2219596u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 24u32, 2219600u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2219604u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2219608u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1580u32, 2219612u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2219616u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2219620u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2219624u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2219628u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2219632u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2219636u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2219640u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2219644u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2219648u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2219652u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2219656u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2219660u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2219664u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2219668u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2219672u32);
    emu.apc_no_count(1usize, 2219672u32, 0u32, 2219676u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021dea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2219684u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2219688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2219692u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2219696u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2219700u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2219704u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 24u32, 2219708u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2219712u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2219716u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1428u32, 2219720u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2219724u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2219728u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2219732u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2219736u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2219740u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2219744u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2219748u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2219752u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2219756u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2219760u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2219764u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2219768u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2219772u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2219776u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2219780u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2219784u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2219788u32);
    emu.apc_no_count(1usize, 2219788u32, 0u32, 2219792u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021df14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2219800u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2219804u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2219808u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2219812u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2219816u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2219820u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2219824u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2219828u32);
    emu.adi_no_count(11usize, 0usize, 1280u32, 2219832u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2220164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e084));
    } else {
        emu.pc = 2219836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df3c));
    }
}
#[inline(always)]
pub fn block_0x0021df3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 160u32, 2219840u32)?;
    emu.sri_no_count(19usize, 8usize, 5u32, 2219844u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2219928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df98));
    } else {
        emu.pc = 2219848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df48));
    }
}
#[inline(always)]
pub fn block_0x0021df48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2219852u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2220216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0b8));
    } else {
        emu.pc = 2219856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df50));
    }
}
#[inline]
pub fn block_0x0021df50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2219860u32);
    emu.sli_no_count(13usize, 12usize, 2u32, 2219864u32);
    emu.adr_no_count(12usize, 12usize, 19usize, 2219868u32);
    emu.adr_no_count(14usize, 13usize, 10usize, 2219872u32);
    emu.adi_no_count(13usize, 12usize, 4294967295u32, 2219876u32);
    emu.sli_no_count(15usize, 12usize, 2u32, 2219880u32);
    emu.adi_no_count(12usize, 14usize, 4294967292u32, 2219884u32);
    emu.adr_no_count(14usize, 15usize, 10usize, 2219888u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2219892u32);
    emu.adi_no_count(15usize, 0usize, 39u32, 2219896u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2219896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021df78));
}
#[inline(always)]
pub fn block_0x0021df78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2220192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0a0));
    } else {
        emu.pc = 2219900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df7c));
    }
}
#[inline(always)]
pub fn block_0x0021df7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 12usize, 0u32, 2219904u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2219908u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2219912u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2219916u32);
    emu.sw_no_count(16usize, 14usize, 0u32, 2219920u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2219924u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2219896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df78));
    } else {
        emu.pc = 2219928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021df98));
    }
}
#[inline(always)]
pub fn block_0x0021df98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 31u32, 2219932u32);
    emu.adi_no_count(11usize, 0usize, 32u32, 2219936u32);
    emu.sli_no_count(9usize, 19usize, 2u32, 2219940u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2219968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfc0));
    } else {
        emu.pc = 2219944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfa8));
    }
}
#[inline(always)]
pub fn block_0x0021dfa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2219948u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2219952u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2219956u32);
    emu.apc_no_count(1usize, 2219956u32, 4294885376u32, 2219960u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2219964u32;
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
#[inline(always)]
pub fn block_0x0021dfbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2219968u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2219968u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021dfc0));
}
#[inline(always)]
pub fn block_0x0021dfc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 160u32, 2219972u32)?;
    emu.adr_no_count(14usize, 14usize, 19usize, 2219976u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2220124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e05c));
    } else {
        emu.pc = 2219980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfcc));
    }
}
#[inline(always)]
pub fn block_0x0021dfcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 4294967295u32, 2219984u32);
    emu.adi_no_count(11usize, 0usize, 39u32, 2219988u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2220192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0a0));
    } else {
        emu.pc = 2219992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dfd8));
    }
}
#[inline(always)]
pub fn block_0x0021dfd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2219996u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2220000u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2220004u32)?;
    emu.sbr_no_count(12usize, 0usize, 8usize, 2220008u32);
    emu.srr_no_count(15usize, 11usize, 12usize, 2220012u32);
    emu.sli_no_count(13usize, 14usize, 2u32, 2220016u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2220020u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2220044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e00c));
    } else {
        emu.pc = 2220024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021dff8));
    }
}
#[inline(always)]
pub fn block_0x0021dff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 39u32, 2220028u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2220240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e0d0));
    } else {
        emu.pc = 2220032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e000));
    }
}
#[inline(always)]
pub fn block_0x0021e000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 13usize, 2220036u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2220040u32)?;
    emu.adi_no_count(11usize, 14usize, 1u32, 2220044u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e00c));
}
#[inline(always)]
pub fn block_0x0021e00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 1u32, 2220048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2220104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e048));
    } else {
        emu.pc = 2220052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e014));
    }
}
#[inline(always)]
pub fn block_0x0021e014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 31u32, 2220056u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2220060u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2220064u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e020));
}
#[inline]
pub fn block_0x0021e020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 13usize, 0u32, 2220068u32)?;
    emu.lw_no_count(16usize, 13usize, 4294967292u32, 2220072u32)?;
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2220076u32);
    emu.slr_no_count(15usize, 15usize, 20usize, 2220080u32);
    emu.srr_no_count(16usize, 16usize, 12usize, 2220084u32);
    emu.adi_no_count(17usize, 13usize, 4294967292u32, 2220088u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2220092u32);
    emu.sw_no_count(15usize, 13usize, 0u32, 2220096u32)?;
    emu.adi_no_count(13usize, 17usize, 0u32, 2220100u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2220064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e020));
    } else {
        emu.pc = 2220104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e048));
    }
}
#[inline(always)]
pub fn block_0x0021e048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 10usize, 9usize, 2220108u32);
    emu.lw_no_count(12usize, 9usize, 0u32, 2220112u32)?;
    emu.slr_no_count(12usize, 12usize, 20usize, 2220116u32);
    emu.sw_no_count(12usize, 9usize, 0u32, 2220120u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2220124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e060));
}
#[inline(always)]
pub fn block_0x0021e05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 0u32, 2220128u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e060));
}
#[inline]
pub fn block_0x0021e060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 160u32, 2220132u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2220136u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2220140u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2220144u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2220148u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2220152u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2220156u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2220160u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220164u32;
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
pub fn block_0x0021e084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2220168u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1670u32, 2220172u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220176u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2220180u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2220184u32);
    emu.apc_no_count(1usize, 2220184u32, 0u32, 2220188u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220196u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2220200u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2220204u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2220208u32);
    emu.apc_no_count(1usize, 2220208u32, 0u32, 2220212u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 4294967295u32, 2220220u32);
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220224u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2220228u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2220232u32);
    emu.apc_no_count(1usize, 2220232u32, 0u32, 2220236u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021e0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220244u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2220248u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2220252u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2220256u32);
    emu.apc_no_count(1usize, 2220256u32, 0u32, 2220260u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965580u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021e0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2220268u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2220272u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2220276u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2220280u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2220284u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2220288u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2220292u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2220296u32)?;
    emu.sli_no_count(12usize, 12usize, 2u32, 2220300u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2220304u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2220568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e218));
    } else {
        emu.pc = 2220308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e114));
    }
}
#[inline]
pub fn block_0x0021e114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 10usize, 0u32, 2220312u32);
    emu.adi_no_count(5usize, 0usize, 0u32, 2220316u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2220320u32);
    emu.sli_no_count(6usize, 14usize, 2u32, 2220324u32);
    emu.sltru_no_count(15usize, 0usize, 14usize, 2220328u32);
    emu.adi_no_count(17usize, 14usize, 1u32, 2220332u32);
    emu.adi_no_count(7usize, 14usize, 4294967295u32, 2220336u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2220340u32);
    emu.sli_no_count(15usize, 15usize, 2u32, 2220344u32);
    emu.sli_no_count(28usize, 7usize, 2u32, 2220348u32);
    emu.adr_no_count(7usize, 13usize, 15usize, 2220352u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2220356u32);
    emu.adi_no_count(28usize, 28usize, 1u32, 2220360u32);
    emu.adi_no_count(29usize, 0usize, 40u32, 2220364u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2220364u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e14c));
}
#[inline(always)]
pub fn block_0x0021e14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(9usize, 5usize, 2u32, 2220368u32);
    emu.adr_no_count(9usize, 16usize, 9usize, 2220372u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2220376u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e158));
}
#[inline(always)]
pub fn block_0x0021e158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2220628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e254));
    } else {
        emu.pc = 2220380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e15c));
    }
}
#[inline(always)]
pub fn block_0x0021e15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 5usize, 0u32, 2220384u32);
    emu.adi_no_count(31usize, 9usize, 0u32, 2220388u32);
    emu.lw_no_count(8usize, 15usize, 0u32, 2220392u32)?;
    emu.adi_no_count(5usize, 5usize, 1u32, 2220396u32);
    emu.adi_no_count(11usize, 15usize, 4u32, 2220400u32);
    emu.adi_no_count(9usize, 9usize, 4u32, 2220404u32);
    emu.adi_no_count(15usize, 11usize, 0u32, 2220408u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2220376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e158));
    } else {
        emu.pc = 2220412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e17c));
    }
}
#[inline(always)]
pub fn block_0x0021e17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2220416u32);
    emu.adi_no_count(9usize, 28usize, 0u32, 2220420u32);
    emu.adi_no_count(15usize, 30usize, 0u32, 2220424u32);
    emu.adi_no_count(20usize, 7usize, 0u32, 2220428u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2220432u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2220432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e190));
}
#[inline(always)]
pub fn block_0x0021e190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2220664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e278));
    } else {
        emu.pc = 2220436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e194));
    }
}
#[inline]
pub fn block_0x0021e194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 20usize, 0u32, 2220440u32);
    emu.lw_no_count(21usize, 21usize, 0u32, 2220444u32)?;
    emu.lw_no_count(20usize, 31usize, 0u32, 2220448u32)?;
    emu.mulhu_no_count(22usize, 21usize, 8usize, 2220452u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2220456u32);
    emu.sltru_no_count(20usize, 18usize, 20usize, 2220460u32);
    emu.adr_no_count(22usize, 20usize, 22usize, 2220464u32);
    emu.xrr_no_count(20usize, 19usize, 6usize, 2220468u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2220472u32);
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2220476u32);
    emu.sltru_no_count(20usize, 0usize, 20usize, 2220480u32);
    emu.sli_no_count(20usize, 20usize, 2u32, 2220484u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2220488u32);
    emu.mul_no_count(21usize, 21usize, 8usize, 2220492u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2220496u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2220500u32);
    emu.sw_no_count(21usize, 31usize, 0u32, 2220504u32)?;
    emu.adr_no_count(18usize, 22usize, 18usize, 2220508u32);
    emu.adi_no_count(31usize, 31usize, 4u32, 2220512u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2220516u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2220432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e190));
    } else {
        emu.pc = 2220520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1e8));
    }
}
#[inline(always)]
pub fn block_0x0021e1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 14usize, 0u32, 2220524u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2220552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e208));
    } else {
        emu.pc = 2220528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1f0));
    }
}
#[inline(always)]
pub fn block_0x0021e1f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 30usize, 14usize, 2220532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2220664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e278));
    } else {
        emu.pc = 2220536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e1f8));
    }
}
#[inline(always)]
pub fn block_0x0021e1f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 2u32, 2220540u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2220544u32);
    emu.sw_no_count(18usize, 15usize, 0u32, 2220548u32)?;
    emu.adi_no_count(15usize, 17usize, 0u32, 2220552u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2220552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e208));
}
#[inline(always)]
pub fn block_0x0021e208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 15usize, 30usize, 2220556u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2220364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e14c));
    } else {
        emu.pc = 2220560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e210));
    }
}
#[inline(always)]
pub fn block_0x0021e210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 15usize, 0u32, 2220564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220364u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e14c));
}
#[inline(always)]
pub fn block_0x0021e218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2220572u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2220576u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2220576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e220));
}
#[inline(always)]
pub fn block_0x0021e220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2220580u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2220584u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2220584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e228));
}
#[inline(always)]
pub fn block_0x0021e228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2220628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e254));
    } else {
        emu.pc = 2220588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e22c));
    }
}
#[inline(always)]
pub fn block_0x0021e22c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2220592u32)?;
    emu.adi_no_count(11usize, 14usize, 4u32, 2220596u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2220600u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2220604u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2220584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e228));
    } else {
        emu.pc = 2220608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e240));
    }
}
#[inline(always)]
pub fn block_0x0021e240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(14usize, 13usize, 4294967295u32, 2220612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2220620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e24c));
    } else {
        emu.pc = 2220616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e248));
    }
}
#[inline(always)]
pub fn block_0x0021e248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2220620u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2220620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e24c));
}
#[inline(always)]
pub fn block_0x0021e24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 13usize, 2220624u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e220));
}
#[inline]
pub fn block_0x0021e254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 28u32, 2220632u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2220636u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2220640u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2220644u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2220648u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2220652u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2220656u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2220660u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220664u32;
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
pub fn block_0x0021e278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2281472u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220668u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1628u32, 2220672u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2220676u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2220680u32);
    emu.apc_no_count(1usize, 2220680u32, 4294963200u32, 2220684u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2220688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0021e290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2220692u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2220696u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2220700u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2220704u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2220708u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2220712u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2220716u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2220720u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2220724u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2220728u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2220732u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2220736u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2220740u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2220744u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2220748u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2220752u32);
    emu.adi_no_count(20usize, 0usize, 0u32, 2220756u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2220760u32);
    emu.adi_no_count(25usize, 0usize, 0u32, 2220764u32);
    let a = 0u32.wrapping_add(168431616u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2220768u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2220772u32;
    emu.update_insn_clock();
    emu.lw_no_count(13usize, 10usize, 0u32, 2220776u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2220780u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2220784u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2220788u32)?;
    emu.lw_no_count(22usize, 10usize, 8u32, 2220792u32)?;
    emu.adi_no_count(10usize, 9usize, 4294967295u32, 2220796u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2220800u32)?;
    emu.adi_no_count(10usize, 9usize, 4u32, 2220804u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2220808u32)?;
    emu.sbr_no_count(10usize, 0usize, 8usize, 2220812u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2220816u32)?;
    emu.adi_no_count(27usize, 11usize, 4294965770u32, 2220820u32);
    emu.adi_no_count(19usize, 12usize, 256u32, 2220824u32);
    emu.adi_no_count(24usize, 0usize, 10u32, 2220828u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2220832u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 10usize, 128u32, 2220836u32);
    emu.add_memory_rw_events(38usize);
    let return_addr = 2220840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e360));
}
#[inline(always)]
pub fn block_0x0021e328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2220844u32)?;
    emu.adr_no_count(10usize, 10usize, 26usize, 2220848u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2220852u32);
    emu.adi_no_count(10usize, 10usize, 4294967286u32, 2220856u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2220860u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2220860u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e33c));
}
#[inline(always)]
pub fn block_0x0021e33c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 22usize, 0u32, 2220864u32);
    emu.lw_no_count(10usize, 2usize, 20u32, 2220868u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2220872u32)?;
    emu.sbr_no_count(12usize, 26usize, 20usize, 2220876u32);
    emu.adr_no_count(11usize, 9usize, 20usize, 2220880u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2220884u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2220888u32;
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
pub fn block_0x0021e358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 18usize, 0u32, 2220892u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e500));
    } else {
        emu.pc = 2220896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e360));
    }
}
#[inline(always)]
pub fn block_0x0021e360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 25usize, 1u32, 2220900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2221304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4f8));
    } else {
        emu.pc = 2220904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e368));
    }
}
#[inline(always)]
pub fn block_0x0021e368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2220924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e37c));
    } else {
        emu.pc = 2220908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e36c));
    }
}
#[inline(always)]
pub fn block_0x0021e36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 21usize, 0u32, 2220912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2220916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e4b0));
}
#[inline(always)]
pub fn block_0x0021e374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 26usize, 0u32, 2220920u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2221232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4b0));
    } else {
        emu.pc = 2220924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e37c));
    }
}
#[inline(always)]
pub fn block_0x0021e37c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 8usize, 21usize, 2220928u32);
    emu.adr_no_count(10usize, 9usize, 21usize, 2220932u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2220936u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2220980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3b4));
    } else {
        emu.pc = 2220940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e38c));
    }
}
#[inline(always)]
pub fn block_0x0021e38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2221228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4ac));
    } else {
        emu.pc = 2220944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e390));
    }
}
#[inline(always)]
pub fn block_0x0021e390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2220948u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2220952u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2220956u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2220956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e39c));
}
#[inline(always)]
pub fn block_0x0021e39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 10usize, 0u32, 2220960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2221152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e460));
    } else {
        emu.pc = 2220964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3a4));
    }
}
#[inline(always)]
pub fn block_0x0021e3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2220968u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2220972u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2220956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e39c));
    } else {
        emu.pc = 2220976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3b0));
    }
}
#[inline(always)]
pub fn block_0x0021e3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2220980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e4ac));
}
#[inline(always)]
pub fn block_0x0021e3b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 3u32, 2220984u32);
    emu.ani_no_count(12usize, 12usize, 4294967292u32, 2220988u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2221072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e410));
    } else {
        emu.pc = 2220992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3c0));
    }
}
#[inline(always)]
pub fn block_0x0021e3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2220996u32);
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2221000u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2221000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e3c8));
}
#[inline(always)]
pub fn block_0x0021e3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 8u32, 2221004u32)?;
    emu.adr_no_count(14usize, 14usize, 21usize, 2221008u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2221008u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e3d0));
}
#[inline]
pub fn block_0x0021e3d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2221012u32);
    emu.adr_no_count(16usize, 14usize, 12usize, 2221016u32);
    emu.lw_no_count(15usize, 15usize, 0u32, 2221020u32)?;
    emu.lw_no_count(16usize, 16usize, 0u32, 2221024u32)?;
    emu.xrr_no_count(17usize, 15usize, 27usize, 2221028u32);
    emu.xrr_no_count(16usize, 16usize, 27usize, 2221032u32);
    emu.sbr_no_count(17usize, 19usize, 17usize, 2221036u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2221040u32);
    emu.sbr_no_count(17usize, 19usize, 16usize, 2221044u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2221048u32);
    emu.anr_no_count(15usize, 15usize, 16usize, 2221052u32);
    emu.anr_no_count(15usize, 15usize, 23usize, 2221056u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2221108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e434));
    } else {
        emu.pc = 2221060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e404));
    }
}
#[inline(always)]
pub fn block_0x0021e404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 8u32, 2221064u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2221008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3d0));
    } else {
        emu.pc = 2221068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e40c));
    }
}
#[inline(always)]
pub fn block_0x0021e40c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221072u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e434));
}
#[inline(always)]
pub fn block_0x0021e410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2221076u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2221080u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2221080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e418));
}
#[inline(always)]
pub fn block_0x0021e418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2221084u32);
    emu.lbu_no_count(14usize, 14usize, 0u32, 2221088u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2221156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e464));
    } else {
        emu.pc = 2221092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e424));
    }
}
#[inline(always)]
pub fn block_0x0021e424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2221096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2221080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e418));
    } else {
        emu.pc = 2221100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e42c));
    }
}
#[inline(always)]
pub fn block_0x0021e42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967288u32, 2221104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2221000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e3c8));
    } else {
        emu.pc = 2221108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e434));
    }
}
#[inline(always)]
pub fn block_0x0021e434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2221228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4ac));
    } else {
        emu.pc = 2221112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e438));
    }
}
#[inline(always)]
pub fn block_0x0021e438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 12usize, 2221116u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2221120u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2221124u32)?;
    emu.adr_no_count(12usize, 12usize, 21usize, 2221128u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2221128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e448));
}
#[inline(always)]
pub fn block_0x0021e448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2221132u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2221172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e474));
    } else {
        emu.pc = 2221136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e450));
    }
}
#[inline(always)]
pub fn block_0x0021e450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2221140u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2221144u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2221128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e448));
    } else {
        emu.pc = 2221148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e45c));
    }
}
#[inline(always)]
pub fn block_0x0021e45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221152u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e4ac));
}
#[inline(always)]
pub fn block_0x0021e460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 11usize, 2221156u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221156u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e464));
}
#[inline(always)]
pub fn block_0x0021e464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 13usize, 2221160u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2221164u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2220916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e374));
    } else {
        emu.pc = 2221168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e470));
    }
}
#[inline(always)]
pub fn block_0x0021e470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221172u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e484));
}
#[inline(always)]
pub fn block_0x0021e474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 0usize, 10usize, 2221176u32);
    emu.adr_no_count(10usize, 21usize, 13usize, 2221180u32);
    emu.adi_no_count(26usize, 10usize, 1u32, 2221184u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2220916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e374));
    } else {
        emu.pc = 2221188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e484));
    }
}
#[inline(always)]
pub fn block_0x0021e484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 9usize, 21usize, 2221192u32);
    emu.adr_no_count(13usize, 21usize, 13usize, 2221196u32);
    emu.lbu_no_count(10usize, 13usize, 0u32, 2221200u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2220916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e374));
    } else {
        emu.pc = 2221204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e494));
    }
}
#[inline(always)]
pub fn block_0x0021e494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2221208u32);
    emu.adi_no_count(18usize, 26usize, 0u32, 2221212u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2221216u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2221220u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2221292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4ec));
    } else {
        emu.pc = 2221224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4a8));
    }
}
#[inline(always)]
pub fn block_0x0021e4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2221228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e4cc));
}
#[inline(always)]
pub fn block_0x0021e4ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 8usize, 0u32, 2221232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e4b0));
}
#[inline(always)]
pub fn block_0x0021e4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2221304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4f8));
    } else {
        emu.pc = 2221236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4b4));
    }
}
#[inline(always)]
pub fn block_0x0021e4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 1u32, 2221240u32);
    emu.adi_no_count(18usize, 20usize, 0u32, 2221244u32);
    emu.adi_no_count(21usize, 26usize, 0u32, 2221248u32);
    emu.adi_no_count(26usize, 8usize, 0u32, 2221252u32);
    emu.lbu_no_count(10usize, 22usize, 0u32, 2221256u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2221292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4ec));
    } else {
        emu.pc = 2221260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4cc));
    }
}
#[inline(always)]
pub fn block_0x0021e4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2221264u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2221268u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2221272u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2221276u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2221280u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 616u32, 2221284u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2221288u32;
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
pub fn block_0x0021e4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2221312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e500));
    } else {
        emu.pc = 2221292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4ec));
    }
}
#[inline(always)]
pub fn block_0x0021e4ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2220840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e328));
    } else {
        emu.pc = 2221296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021e4f0));
    }
}
#[inline(always)]
pub fn block_0x0021e4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2221300u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2221304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2220860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e33c));
}
#[inline(always)]
pub fn block_0x0021e4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2221308u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2221312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2221316u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e504));
}
#[inline(always)]
pub fn block_0x0021e500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2221316u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2221316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021e504));
}
#[inline]
pub fn block_0x0021e504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2221320u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2221324u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2221328u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2221332u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2221336u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2221340u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2221344u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2221348u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2221352u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2221356u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2221360u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2221364u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2221368u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2221372u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2221376u32;
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
