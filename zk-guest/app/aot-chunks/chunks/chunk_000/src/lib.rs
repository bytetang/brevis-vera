pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2099200u32;
pub const PC_MAX: u32 = 2101528u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 157usize] = [
        block_0x00200800,
        block_0x00200810,
        block_0x0020081c,
        block_0x00200824,
        block_0x00200828,
        block_0x00200834,
        block_0x00200838,
        block_0x0020083c,
        block_0x00200840,
        block_0x00200850,
        block_0x0020086c,
        block_0x00200878,
        block_0x0020087c,
        block_0x00200890,
        block_0x0020089c,
        block_0x002008a8,
        block_0x002008b4,
        block_0x002008bc,
        block_0x002008d0,
        block_0x002008d8,
        block_0x002008dc,
        block_0x002008f4,
        block_0x00200904,
        block_0x0020091c,
        block_0x00200920,
        block_0x00200934,
        block_0x0020094c,
        block_0x00200990,
        block_0x002009c8,
        block_0x002009d8,
        block_0x002009ec,
        block_0x002009fc,
        block_0x00200a0c,
        block_0x00200a10,
        block_0x00200a18,
        block_0x00200a20,
        block_0x00200a2c,
        block_0x00200a40,
        block_0x00200a44,
        block_0x00200a58,
        block_0x00200a68,
        block_0x00200a70,
        block_0x00200a88,
        block_0x00200a90,
        block_0x00200aa8,
        block_0x00200aac,
        block_0x00200ac4,
        block_0x00200ac8,
        block_0x00200ae0,
        block_0x00200ae4,
        block_0x00200afc,
        block_0x00200b00,
        block_0x00200b10,
        block_0x00200b14,
        block_0x00200b24,
        block_0x00200b2c,
        block_0x00200b38,
        block_0x00200b48,
        block_0x00200b54,
        block_0x00200b60,
        block_0x00200b88,
        block_0x00200bcc,
        block_0x00200bd4,
        block_0x00200be0,
        block_0x00200bfc,
        block_0x00200c04,
        block_0x00200c1c,
        block_0x00200c28,
        block_0x00200c40,
        block_0x00200c48,
        block_0x00200c64,
        block_0x00200c6c,
        block_0x00200c70,
        block_0x00200c80,
        block_0x00200c90,
        block_0x00200ca0,
        block_0x00200ca4,
        block_0x00200cb4,
        block_0x00200cd4,
        block_0x00200cd8,
        block_0x00200ce8,
        block_0x00200d2c,
        block_0x00200d30,
        block_0x00200d58,
        block_0x00200d6c,
        block_0x00200d74,
        block_0x00200d80,
        block_0x00200d88,
        block_0x00200d8c,
        block_0x00200da0,
        block_0x00200dac,
        block_0x00200db8,
        block_0x00200dc8,
        block_0x00200dd8,
        block_0x00200de4,
        block_0x00200e08,
        block_0x00200e10,
        block_0x00200e24,
        block_0x00200e28,
        block_0x00200e38,
        block_0x00200e3c,
        block_0x00200e44,
        block_0x00200e5c,
        block_0x00200e60,
        block_0x00200e6c,
        block_0x00200e70,
        block_0x00200e84,
        block_0x00200e90,
        block_0x00200e9c,
        block_0x00200eb0,
        block_0x00200ec8,
        block_0x00200ed4,
        block_0x00200ef4,
        block_0x00200f00,
        block_0x00200f14,
        block_0x00200f18,
        block_0x00200f24,
        block_0x00200f28,
        block_0x00200f30,
        block_0x00200f48,
        block_0x00200f4c,
        block_0x00200f54,
        block_0x00200f58,
        block_0x00200f6c,
        block_0x00200f78,
        block_0x00200f7c,
        block_0x00200f80,
        block_0x00200f94,
        block_0x00200fb0,
        block_0x00200fb8,
        block_0x00200fcc,
        block_0x00200fd0,
        block_0x00200fd4,
        block_0x00200fdc,
        block_0x00200fe0,
        block_0x00200fe8,
        block_0x00201000,
        block_0x00201004,
        block_0x00201018,
        block_0x00201024,
        block_0x00201028,
        block_0x00201038,
        block_0x00201050,
        block_0x00201054,
        block_0x00201058,
        block_0x00201064,
        block_0x00201074,
        block_0x00201084,
        block_0x0020108c,
        block_0x002010a0,
        block_0x002010a4,
        block_0x002010a8,
        block_0x002010b0,
        block_0x002010e4,
        block_0x002010f8,
        block_0x0020110c,
        block_0x00201118,
    ];
    const IDX: [u16; 583usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16, 5u16, 0u16, 0u16,
        6u16, 7u16, 8u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 11u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16,
        15u16, 0u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        19u16, 0u16, 20u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 26u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 33u16, 34u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 41u16,
        0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 45u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16,
        0u16, 0u16, 53u16, 54u16, 0u16, 0u16, 0u16, 55u16, 0u16, 56u16, 0u16, 0u16,
        57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        63u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 72u16,
        73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16,
        77u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        80u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16,
        86u16, 0u16, 0u16, 87u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 0u16, 91u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16,
        0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 100u16,
        101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16,
        105u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16,
        109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16,
        0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16,
        114u16, 0u16, 0u16, 0u16, 0u16, 115u16, 116u16, 0u16, 0u16, 117u16, 118u16, 0u16,
        119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 121u16, 0u16, 122u16, 123u16, 0u16,
        0u16, 0u16, 0u16, 124u16, 0u16, 0u16, 125u16, 126u16, 127u16, 0u16, 0u16, 0u16,
        0u16, 128u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 129u16, 0u16, 130u16, 0u16,
        0u16, 0u16, 0u16, 131u16, 132u16, 133u16, 0u16, 134u16, 135u16, 0u16, 136u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 137u16, 138u16, 0u16, 0u16, 0u16, 0u16, 139u16,
        0u16, 0u16, 140u16, 141u16, 0u16, 0u16, 0u16, 142u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 143u16, 144u16, 145u16, 0u16, 0u16, 146u16, 0u16, 0u16, 0u16, 147u16, 0u16,
        0u16, 0u16, 148u16, 0u16, 149u16, 0u16, 0u16, 0u16, 0u16, 150u16, 151u16, 152u16,
        0u16, 153u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 154u16, 0u16, 0u16, 0u16, 0u16, 155u16, 0u16, 0u16, 0u16, 0u16, 156u16,
        0u16, 0u16, 157u16,
    ];
    if pc < 2099200u32 || pc > 2101528u32 {
        return None;
    }
    let word_offset = ((pc - 2099200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00200800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2099204u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2099208u32)?;
    emu.adi_no_count(13usize, 11usize, 2u32, 2099212u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2099380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008b4));
    } else {
        emu.pc = 2099216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200810));
    }
}
#[inline(always)]
pub fn block_0x00200810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2099220u32)?;
    emu.adi_no_count(12usize, 0usize, 4294967293u32, 2099224u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2099420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008dc));
    } else {
        emu.pc = 2099228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020081c));
    }
}
#[inline(always)]
pub fn block_0x0020081c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 11usize, 2099232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2099260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020083c));
    } else {
        emu.pc = 2099236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200824));
    }
}
#[inline(always)]
pub fn block_0x00200824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2099256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200838));
    } else {
        emu.pc = 2099240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200828));
    }
}
#[inline(always)]
pub fn block_0x00200828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 12usize, 0u32, 2099244u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2099248u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2099260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020083c));
    } else {
        emu.pc = 2099252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200834));
    }
}
#[inline(always)]
pub fn block_0x00200834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002008dc));
}
#[inline(always)]
pub fn block_0x00200838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2099420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008dc));
    } else {
        emu.pc = 2099260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020083c));
    }
}
#[inline(always)]
pub fn block_0x0020083c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2099280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200850));
    } else {
        emu.pc = 2099264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200840));
    }
}
#[inline(always)]
pub fn block_0x00200840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 10usize, 13usize, 2099268u32);
    emu.lb_no_count(14usize, 14usize, 0u32, 2099272u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2099276u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2099420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008dc));
    } else {
        emu.pc = 2099280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200850));
    }
}
#[inline(always)]
pub fn block_0x00200850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 12usize, 0u32, 2099284u32);
    emu.adi_no_count(10usize, 13usize, 4294967253u32, 2099288u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2099292u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2099296u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2099300u32);
    emu.adi_no_count(11usize, 0usize, 57u32, 2099304u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2099324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020087c));
    } else {
        emu.pc = 2099308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020086c));
    }
}
#[inline(always)]
pub fn block_0x0020086c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 4294967248u32, 2099312u32);
    emu.adi_no_count(10usize, 0usize, 15u32, 2099316u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2099344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200890));
    } else {
        emu.pc = 2099320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200878));
    }
}
#[inline(always)]
pub fn block_0x00200878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2099380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002008b4));
}
#[inline(always)]
pub fn block_0x0020087c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967231u32, 2099328u32);
    emu.ani_no_count(11usize, 10usize, 4294967263u32, 2099332u32);
    emu.adi_no_count(11usize, 11usize, 10u32, 2099336u32);
    emu.adi_no_count(10usize, 0usize, 15u32, 2099340u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2099380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008b4));
    } else {
        emu.pc = 2099344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200890));
    }
}
#[inline(always)]
pub fn block_0x00200890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 43u32, 2099348u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2099352u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2099416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008d8));
    } else {
        emu.pc = 2099356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020089c));
    }
}
#[inline(always)]
pub fn block_0x0020089c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 12usize, 1u32, 2099360u32);
    emu.adi_no_count(13usize, 0usize, 57u32, 2099364u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2099388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008bc));
    } else {
        emu.pc = 2099368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008a8));
    }
}
#[inline(always)]
pub fn block_0x002008a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967248u32, 2099372u32);
    emu.adi_no_count(13usize, 0usize, 15u32, 2099376u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2099408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008d0));
    } else {
        emu.pc = 2099380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008b4));
    }
}
#[inline(always)]
pub fn block_0x002008b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2099384u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099388u32;
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
pub fn block_0x002008bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4294967231u32, 2099392u32);
    emu.ani_no_count(12usize, 12usize, 4294967263u32, 2099396u32);
    emu.adi_no_count(12usize, 12usize, 10u32, 2099400u32);
    emu.adi_no_count(13usize, 0usize, 15u32, 2099404u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2099380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008b4));
    } else {
        emu.pc = 2099408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002008d0));
    }
}
#[inline(always)]
pub fn block_0x002008d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 4u32, 2099412u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2099416u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2099416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002008d8));
}
#[inline(always)]
pub fn block_0x002008d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099420u32;
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
pub fn block_0x002008dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2099424u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966812u32, 2099428u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2099432u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2099436u32);
    emu.apc_no_count(1usize, 2099436u32, 118784u32, 2099440u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099444u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x002008f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2099448u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2099452u32)?;
    emu.apc_no_count(1usize, 2099452u32, 36864u32, 2099456u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(80u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099464u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2099468u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2099472u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2099476u32);
    emu.apc_no_count(1usize, 2099476u32, 40960u32, 2099480u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020091c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200934));
    } else {
        emu.pc = 2099488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200920));
    }
}
#[inline(always)]
pub fn block_0x00200920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2099492u32);
    emu.adi_no_count(10usize, 0usize, 64u32, 2099496u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2099500u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2099504u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099508u32;
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
pub fn block_0x00200934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099512u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966532u32, 2099516u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2099520u32);
    emu.adi_no_count(11usize, 0usize, 64u32, 2099524u32);
    emu.apc_no_count(1usize, 2099524u32, 110592u32, 2099528u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020094c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2099536u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2099540u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2099544u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2099548u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2099552u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2099556u32)?;
    emu.sw_no_count(20usize, 2usize, 616u32, 2099560u32)?;
    emu.sw_no_count(21usize, 2usize, 612u32, 2099564u32)?;
    emu.sw_no_count(22usize, 2usize, 608u32, 2099568u32)?;
    emu.sw_no_count(23usize, 2usize, 604u32, 2099572u32)?;
    emu.sw_no_count(24usize, 2usize, 600u32, 2099576u32)?;
    emu.sw_no_count(25usize, 2usize, 596u32, 2099580u32)?;
    emu.sw_no_count(26usize, 2usize, 592u32, 2099584u32)?;
    emu.sw_no_count(27usize, 2usize, 588u32, 2099588u32)?;
    emu.adi_no_count(10usize, 2usize, 380u32, 2099592u32);
    emu.apc_no_count(1usize, 2099592u32, 36864u32, 2099596u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00200990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 384u32, 2099604u32)?;
    emu.lw_no_count(11usize, 2usize, 388u32, 2099608u32)?;
    emu.sw_no_count(10usize, 2usize, 460u32, 2099612u32)?;
    emu.sw_no_count(11usize, 2usize, 464u32, 2099616u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2099620u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965964u32, 2099624u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2099628u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966052u32, 2099632u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2099636u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2099640u32);
    emu.adi_no_count(13usize, 0usize, 12u32, 2099644u32);
    emu.adi_no_count(15usize, 0usize, 5u32, 2099648u32);
    emu.apc_no_count(1usize, 2099648u32, 8192u32, 2099652u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099656u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1608u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002009c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 188u32, 2099660u32)?;
    emu.lw_no_count(8usize, 2usize, 192u32, 2099664u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2099668u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2103620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2103620u32));
    } else {
        emu.pc = 2099672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009d8));
    }
}
#[inline(always)]
pub fn block_0x002009d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 196u32, 2099676u32);
    emu.adi_no_count(10usize, 2usize, 76u32, 2099680u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2099684u32);
    emu.apc_no_count(1usize, 2099684u32, 40960u32, 2099688u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966008u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002009ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 104u32, 2099696u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2099700u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2099704u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2099728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a10));
    } else {
        emu.pc = 2099708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002009fc));
    }
}
#[inline(always)]
pub fn block_0x002009fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 64u32, 2099712u32)?;
    emu.adi_no_count(18usize, 0usize, 0u32, 2099716u32);
    emu.lw_no_count(26usize, 2usize, 88u32, 2099720u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2100352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c80));
    } else {
        emu.pc = 2099724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a0c));
    }
}
#[inline(always)]
pub fn block_0x00200a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2102760u32));
}
#[inline(always)]
pub fn block_0x00200a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 112u32, 2099732u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2099984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b10));
    } else {
        emu.pc = 2099736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a18));
    }
}
#[inline(always)]
pub fn block_0x00200a18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2099740u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2099984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b10));
    } else {
        emu.pc = 2099744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a20));
    }
}
#[inline(always)]
pub fn block_0x00200a20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 128u32, 2099748u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099752u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2099780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a44));
    } else {
        emu.pc = 2099756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a2c));
    }
}
#[inline(always)]
pub fn block_0x00200a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2099760u32);
    emu.lw_no_count(11usize, 2usize, 140u32, 2099764u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099768u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 0usize, 1u32, 2099772u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2100004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b24));
    } else {
        emu.pc = 2099776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a40));
    }
}
#[inline(always)]
pub fn block_0x00200a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2099780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200c70));
}
#[inline(always)]
pub fn block_0x00200a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 132u32, 2099784u32)?;
    emu.lw_no_count(12usize, 2usize, 136u32, 2099788u32)?;
    emu.adi_no_count(10usize, 2usize, 188u32, 2099792u32);
    emu.apc_no_count(1usize, 2099792u32, 110592u32, 2099796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 196u32, 2099804u32)?;
    emu.lw_no_count(9usize, 2usize, 192u32, 2099808u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2099812u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102264u32));
    } else {
        emu.pc = 2099816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a68));
    }
}
#[inline(always)]
pub fn block_0x00200a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 5u32, 2099820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2099984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b10));
    } else {
        emu.pc = 2099824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a70));
    }
}
#[inline(always)]
pub fn block_0x00200a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099828u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966492u32, 2099832u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099836u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2099840u32);
    emu.apc_no_count(1usize, 2099840u32, 151552u32, 2099844u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2099852u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    } else {
        emu.pc = 2099856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200a90));
    }
}
#[inline(always)]
pub fn block_0x00200a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099860u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966497u32, 2099864u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099868u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2099872u32);
    emu.apc_no_count(1usize, 2099872u32, 151552u32, 2099876u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200aa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    } else {
        emu.pc = 2099884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200aac));
    }
}
#[inline(always)]
pub fn block_0x00200aac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099888u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966502u32, 2099892u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099896u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2099900u32);
    emu.apc_no_count(1usize, 2099900u32, 151552u32, 2099904u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    } else {
        emu.pc = 2099912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ac8));
    }
}
#[inline(always)]
pub fn block_0x00200ac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099916u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966507u32, 2099920u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099924u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2099928u32);
    emu.apc_no_count(1usize, 2099928u32, 151552u32, 2099932u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    } else {
        emu.pc = 2099940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ae4));
    }
}
#[inline(always)]
pub fn block_0x00200ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099944u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966512u32, 2099948u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099952u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2099956u32);
    emu.apc_no_count(1usize, 2099956u32, 151552u32, 2099960u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2099964u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00200afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2099988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b14));
    } else {
        emu.pc = 2099968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b00));
    }
}
#[inline(always)]
pub fn block_0x00200b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2099972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966517u32, 2099976u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2099980u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2099984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2102276u32));
}
#[inline(always)]
pub fn block_0x00200b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2099988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2099988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200b14));
}
#[inline(always)]
pub fn block_0x00200b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 140u32, 2099992u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2099996u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 0usize, 1u32, 2100000u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b24));
    }
}
#[inline(always)]
pub fn block_0x00200b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 164u32, 2100008u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b2c));
    }
}
#[inline(always)]
pub fn block_0x00200b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 148u32, 2100016u32)?;
    emu.adi_no_count(11usize, 0usize, 64u32, 2100020u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b38));
    }
}
#[inline(always)]
pub fn block_0x00200b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 172u32, 2100028u32)?;
    emu.adi_no_count(11usize, 0usize, 130u32, 2100032u32);
    emu.adi_no_count(18usize, 0usize, 0u32, 2100036u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b48));
    }
}
#[inline(always)]
pub fn block_0x00200b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 160u32, 2100044u32)?;
    emu.adi_no_count(10usize, 0usize, 64u32, 2100048u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b54));
    }
}
#[inline(always)]
pub fn block_0x00200b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 76u32, 2100056u32)?;
    emu.adi_no_count(18usize, 0usize, 0u32, 2100060u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b60));
    }
}
#[inline]
pub fn block_0x00200b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 168u32, 2100068u32)?;
    emu.lbu_no_count(11usize, 10usize, 1u32, 2100072u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2100076u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2100080u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2100084u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100088u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1072u32, 2100092u32);
    emu.xrr_no_count(11usize, 11usize, 12usize, 2100096u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2100100u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2100336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c70));
    } else {
        emu.pc = 2100104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200b88));
    }
}
#[inline]
pub fn block_0x00200b88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2100108u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 0usize, 130u32, 2100112u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2100116u32);
    emu.adi_no_count(14usize, 2usize, 460u32, 2100120u32);
    emu.adi_no_count(9usize, 11usize, 2u32, 2100124u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2100128u32)?;
    emu.sw_no_count(12usize, 2usize, 192u32, 2100132u32)?;
    emu.sw_no_count(13usize, 2usize, 196u32, 2100136u32)?;
    emu.sw_no_count(0usize, 2usize, 200u32, 2100140u32)?;
    emu.sw_no_count(9usize, 2usize, 460u32, 2100144u32)?;
    emu.sw_no_count(14usize, 2usize, 204u32, 2100148u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2100152u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 508u32, 2100156u32);
    emu.adi_no_count(10usize, 2usize, 380u32, 2100160u32);
    emu.adi_no_count(11usize, 2usize, 188u32, 2100164u32);
    emu.apc_no_count(1usize, 2100164u32, 36864u32, 2100168u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 460u32, 2100176u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200bd4));
    }
}
#[inline(always)]
pub fn block_0x00200bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 380u32, 2100184u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2100188u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200be0));
    }
}
#[inline(always)]
pub fn block_0x00200be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 384u32, 2100196u32)?;
    emu.lw_no_count(23usize, 2usize, 388u32, 2100200u32)?;
    emu.adi_no_count(10usize, 2usize, 188u32, 2100204u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2100208u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2100212u32);
    emu.apc_no_count(1usize, 2100212u32, 32768u32, 2100216u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100220u32;
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
#[inline(always)]
pub fn block_0x00200bfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2100224u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c04));
    }
}
#[inline(always)]
pub fn block_0x00200c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 192u32, 2100232u32)?;
    emu.lw_no_count(9usize, 2usize, 196u32, 2100236u32)?;
    emu.adi_no_count(11usize, 2usize, 140u32, 2100240u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2100244u32);
    emu.apc_no_count(1usize, 2100244u32, 36864u32, 2100248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2100256u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2100260u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c28));
    }
}
#[inline(always)]
pub fn block_0x00200c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 192u32, 2100268u32)?;
    emu.lw_no_count(20usize, 2usize, 196u32, 2100272u32)?;
    emu.adi_no_count(11usize, 2usize, 152u32, 2100276u32);
    emu.adi_no_count(10usize, 2usize, 188u32, 2100280u32);
    emu.apc_no_count(1usize, 2100280u32, 36864u32, 2100284u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2100292u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    } else {
        emu.pc = 2100296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c48));
    }
}
#[inline(always)]
pub fn block_0x00200c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 192u32, 2100300u32)?;
    emu.lw_no_count(18usize, 2usize, 196u32, 2100304u32)?;
    emu.adi_no_count(10usize, 2usize, 188u32, 2100308u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2100312u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2100316u32);
    emu.apc_no_count(1usize, 2100316u32, 28672u32, 2100320u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2100328u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102564u32));
    } else {
        emu.pc = 2100332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c6c));
    }
}
#[inline(always)]
pub fn block_0x00200c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2100336u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2100336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200c70));
}
#[inline(always)]
pub fn block_0x00200c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 24usize, 18usize, 2100340u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2100344u32)?;
    emu.lw_no_count(26usize, 2usize, 88u32, 2100348u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2102760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102760u32));
    } else {
        emu.pc = 2100352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c80));
    }
}
#[inline(always)]
pub fn block_0x00200c80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 176u32, 2100356u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2100360u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2100364u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2100388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ca4));
    } else {
        emu.pc = 2100368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200c90));
    }
}
#[inline(always)]
pub fn block_0x00200c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2100372u32);
    emu.lw_no_count(20usize, 2usize, 84u32, 2100376u32)?;
    emu.lw_no_count(10usize, 20usize, 40u32, 2100380u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cb4));
    } else {
        emu.pc = 2100384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ca0));
    }
}
#[inline(always)]
pub fn block_0x00200ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2100388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2102756u32));
}
#[inline(always)]
pub fn block_0x00200ca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 180u32, 2100392u32)?;
    emu.lw_no_count(20usize, 2usize, 84u32, 2100396u32)?;
    emu.lw_no_count(10usize, 20usize, 40u32, 2100400u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2100404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cb4));
    }
}
#[inline(always)]
pub fn block_0x00200cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 56u32, 2100408u32)?;
    emu.lw_no_count(18usize, 2usize, 184u32, 2100412u32)?;
    emu.lw_no_count(11usize, 2usize, 72u32, 2100416u32)?;
    emu.lw_no_count(27usize, 2usize, 96u32, 2100420u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2100424u32)?;
    emu.lw_no_count(10usize, 20usize, 36u32, 2100428u32)?;
    emu.apc_no_count(1usize, 2100428u32, 151552u32, 2100432u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100436u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2100456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ce8));
    } else {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    }
}
#[inline(always)]
pub fn block_0x00200cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2100444u32);
    emu.adi_no_count(27usize, 0usize, 0u32, 2100448u32);
    emu.lw_no_count(18usize, 2usize, 56u32, 2100452u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2100456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2102764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2102764u32));
}
#[inline]
pub fn block_0x00200ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 28u32, 2100460u32)?;
    emu.adi_no_count(8usize, 0usize, 0u32, 2100464u32);
    emu.xrr_no_count(10usize, 9usize, 19usize, 2100468u32);
    emu.adi_no_count(22usize, 0usize, 3u32, 2100472u32);
    emu.adi_no_count(19usize, 0usize, 90u32, 2100476u32);
    emu.adi_no_count(12usize, 0usize, 270u32, 2100480u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2100484u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2100488u32);
    emu.anr_no_count(10usize, 10usize, 18usize, 2100492u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2100496u32)?;
    emu.lw_no_count(18usize, 2usize, 56u32, 2100500u32)?;
    emu.adi_no_count(9usize, 21usize, 0u32, 2100504u32);
    emu.sw_no_count(26usize, 2usize, 44u32, 2100508u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2100512u32)?;
    emu.sw_no_count(27usize, 2usize, 48u32, 2100516u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2100520u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2100524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2100528u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200d30));
}
#[inline(always)]
pub fn block_0x00200d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2102360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102360u32));
    } else {
        emu.pc = 2100528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d30));
    }
}
#[inline]
pub fn block_0x00200d30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 8usize, 3u32, 2100532u32);
    emu.sli_no_count(11usize, 8usize, 6u32, 2100536u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2100540u32);
    emu.adr_no_count(21usize, 20usize, 11usize, 2100544u32);
    emu.lw_no_count(10usize, 21usize, 52u32, 2100548u32)?;
    emu.lw_no_count(25usize, 21usize, 36u32, 2100552u32)?;
    emu.lw_no_count(24usize, 21usize, 40u32, 2100556u32)?;
    emu.lw_no_count(23usize, 21usize, 48u32, 2100560u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2100564u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2100596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d74));
    } else {
        emu.pc = 2100568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d58));
    }
}
#[inline(always)]
pub fn block_0x00200d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 25usize, 0u32, 2100572u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2100576u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2100580u32);
    emu.apc_no_count(1usize, 2100580u32, 151552u32, 2100584u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 270u32, 2100592u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2100596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d74));
    }
}
#[inline(always)]
pub fn block_0x00200d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2100600u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2100604u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2100844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e6c));
    } else {
        emu.pc = 2100608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d80));
    }
}
#[inline(always)]
pub fn block_0x00200d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2100612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2101076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f54));
    } else {
        emu.pc = 2100616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d88));
    }
}
#[inline(always)]
pub fn block_0x00200d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2101580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101580u32));
    } else {
        emu.pc = 2100620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200d8c));
    }
}
#[inline(always)]
pub fn block_0x00200d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2100624u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2100628u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2100632u32);
    emu.lw_no_count(23usize, 10usize, 8u32, 2100636u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2101580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101580u32));
    } else {
        emu.pc = 2100640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200da0));
    }
}
#[inline(always)]
pub fn block_0x00200da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 21usize, 4u32, 2100644u32)?;
    emu.adi_no_count(26usize, 0usize, 0u32, 2100648u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2102556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102556u32));
    } else {
        emu.pc = 2100652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dac));
    }
}
#[inline(always)]
pub fn block_0x00200dac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 8u32, 2100656u32)?;
    emu.adi_no_count(27usize, 26usize, 0u32, 2100660u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2102764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102764u32));
    } else {
        emu.pc = 2100664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200db8));
    }
}
#[inline(always)]
pub fn block_0x00200db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 21usize, 12u32, 2100668u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2100672u32)?;
    emu.adi_no_count(26usize, 0usize, 0u32, 2100676u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102556u32));
    } else {
        emu.pc = 2100680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dc8));
    }
}
#[inline(always)]
pub fn block_0x00200dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 16u32, 2100684u32)?;
    emu.lw_no_count(11usize, 10usize, 16u32, 2100688u32)?;
    emu.adi_no_count(27usize, 26usize, 0u32, 2100692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2102764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102764u32));
    } else {
        emu.pc = 2100696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200dd8));
    }
}
#[inline(always)]
pub fn block_0x00200dd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 20usize, 2u32, 2100700u32);
    emu.mul_no_count(11usize, 14usize, 13usize, 2100704u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2100708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200de4));
    }
}
#[inline]
pub fn block_0x00200de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 16u32, 2100712u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2100716u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2100720u32)?;
    emu.lw_no_count(26usize, 10usize, 4u32, 2100724u32)?;
    emu.adi_no_count(10usize, 2usize, 380u32, 2100728u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2100732u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2100736u32);
    emu.apc_no_count(1usize, 2100736u32, 4096u32, 2100740u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 388u32, 2100748u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2100752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e10));
    }
}
#[inline(always)]
pub fn block_0x00200e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 384u32, 2100756u32)?;
    emu.adi_no_count(11usize, 25usize, 0u32, 2100760u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2100764u32);
    emu.apc_no_count(1usize, 2100764u32, 151552u32, 2100768u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2100776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e28));
    }
}
#[inline(always)]
pub fn block_0x00200e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 2usize, 24u32, 2100780u32)?;
    emu.mul_no_count(24usize, 9usize, 5usize, 2100784u32);
    emu.sli_no_count(24usize, 24usize, 2u32, 2100788u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102336u32));
    } else {
        emu.pc = 2100792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e38));
    }
}
#[inline(always)]
pub fn block_0x00200e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2101648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101648u32));
    } else {
        emu.pc = 2100796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e3c));
    }
}
#[inline(always)]
pub fn block_0x00200e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2100796u32, 36864u32, 2100800u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100804u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2100808u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2100812u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2100816u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2100820u32);
    emu.apc_no_count(1usize, 2100820u32, 40960u32, 2100824u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2103840u32));
    } else {
        emu.pc = 2100832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e60));
    }
}
#[inline(always)]
pub fn block_0x00200e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 10usize, 0u32, 2100836u32);
    emu.lw_no_count(5usize, 2usize, 24u32, 2100840u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2100844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2101652u32));
}
#[inline(always)]
pub fn block_0x00200e6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2101596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101596u32));
    } else {
        emu.pc = 2100848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e70));
    }
}
#[inline(always)]
pub fn block_0x00200e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2100852u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2100856u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2100860u32);
    emu.lw_no_count(23usize, 10usize, 8u32, 2100864u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2101596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101596u32));
    } else {
        emu.pc = 2100868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e84));
    }
}
#[inline(always)]
pub fn block_0x00200e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 21usize, 12u32, 2100872u32)?;
    emu.adi_no_count(26usize, 0usize, 0u32, 2100876u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2102556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102556u32));
    } else {
        emu.pc = 2100880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e90));
    }
}
#[inline(always)]
pub fn block_0x00200e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 21usize, 16u32, 2100884u32)?;
    emu.adi_no_count(27usize, 26usize, 0u32, 2100888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2102764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102764u32));
    } else {
        emu.pc = 2100892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200e9c));
    }
}
#[inline(always)]
pub fn block_0x00200e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 21usize, 4u32, 2100896u32)?;
    emu.lw_no_count(13usize, 10usize, 12u32, 2100900u32)?;
    emu.adr_no_count(11usize, 20usize, 12usize, 2100904u32);
    emu.adi_no_count(26usize, 0usize, 0u32, 2100908u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2102556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102556u32));
    } else {
        emu.pc = 2100912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200eb0));
    }
}
#[inline(always)]
pub fn block_0x00200eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 16u32, 2100916u32)?;
    emu.lw_no_count(12usize, 21usize, 8u32, 2100920u32)?;
    emu.lw_no_count(11usize, 10usize, 16u32, 2100924u32)?;
    emu.adr_no_count(21usize, 9usize, 12usize, 2100928u32);
    emu.adi_no_count(27usize, 26usize, 0u32, 2100932u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2102764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102764u32));
    } else {
        emu.pc = 2100936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ec8));
    }
}
#[inline(always)]
pub fn block_0x00200ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(18usize, 13usize, 2u32, 2100940u32);
    emu.mul_no_count(11usize, 18usize, 11usize, 2100944u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2100948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200ed4));
    }
}
#[inline(always)]
pub fn block_0x00200ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 20u32, 2100952u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2100956u32)?;
    emu.lw_no_count(26usize, 10usize, 4u32, 2100960u32)?;
    emu.adi_no_count(10usize, 2usize, 380u32, 2100964u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2100968u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2100972u32);
    emu.apc_no_count(1usize, 2100972u32, 4096u32, 2100976u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2100980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 388u32, 2100984u32)?;
    emu.lw_no_count(27usize, 2usize, 48u32, 2100988u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2100992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f00));
    }
}
#[inline(always)]
pub fn block_0x00200f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 384u32, 2100996u32)?;
    emu.adi_no_count(11usize, 25usize, 0u32, 2101000u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2101004u32);
    emu.apc_no_count(1usize, 2101004u32, 151552u32, 2101008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2101016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f18));
    }
}
#[inline(always)]
pub fn block_0x00200f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 20usize, 2u32, 2101020u32);
    emu.mul_no_count(25usize, 9usize, 24usize, 2101024u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2102312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102312u32));
    } else {
        emu.pc = 2101028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f24));
    }
}
#[inline(always)]
pub fn block_0x00200f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2101948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101948u32));
    } else {
        emu.pc = 2101032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f28));
    }
}
#[inline(always)]
pub fn block_0x00200f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101032u32, 36864u32, 2101036u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2101044u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101048u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2101052u32);
    emu.adi_no_count(12usize, 25usize, 0u32, 2101056u32);
    emu.apc_no_count(1usize, 2101056u32, 40960u32, 2101060u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2103812u32));
    } else {
        emu.pc = 2101068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f4c));
    }
}
#[inline(always)]
pub fn block_0x00200f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2101072u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2101076u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101952u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2101952u32));
}
#[inline(always)]
pub fn block_0x00200f54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2101620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101620u32));
    } else {
        emu.pc = 2101080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f58));
    }
}
#[inline(always)]
pub fn block_0x00200f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2101084u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2101088u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2101092u32);
    emu.lw_no_count(23usize, 10usize, 8u32, 2101096u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2101620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2101620u32));
    } else {
        emu.pc = 2101100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f6c));
    }
}
#[inline(always)]
pub fn block_0x00200f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 21usize, 4u32, 2101104u32)?;
    emu.adi_no_count(11usize, 0usize, 180u32, 2101108u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2101120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f80));
    } else {
        emu.pc = 2101112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f78));
    }
}
#[inline(always)]
pub fn block_0x00200f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2101120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f80));
    } else {
        emu.pc = 2101116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f7c));
    }
}
#[inline(always)]
pub fn block_0x00200f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2102756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102756u32));
    } else {
        emu.pc = 2101120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f80));
    }
}
#[inline(always)]
pub fn block_0x00200f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2101124u32)?;
    emu.lw_no_count(20usize, 10usize, 16u32, 2101128u32)?;
    emu.sli_no_count(27usize, 18usize, 2u32, 2101132u32);
    emu.mul_no_count(11usize, 27usize, 20usize, 2101136u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2101140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200f94));
    }
}
#[inline(always)]
pub fn block_0x00200f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 270u32, 2101144u32);
    emu.lw_no_count(26usize, 10usize, 4u32, 2101148u32)?;
    emu.adi_no_count(10usize, 2usize, 380u32, 2101152u32);
    emu.adi_no_count(11usize, 26usize, 0u32, 2101156u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2101160u32);
    emu.apc_no_count(1usize, 2101160u32, 4096u32, 2101164u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 388u32, 2101172u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2101176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fb8));
    }
}
#[inline(always)]
pub fn block_0x00200fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 384u32, 2101180u32)?;
    emu.adi_no_count(11usize, 25usize, 0u32, 2101184u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2101188u32);
    emu.apc_no_count(1usize, 2101188u32, 151552u32, 2101192u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2101200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fd0));
    }
}
#[inline(always)]
pub fn block_0x00200fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2101212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fdc));
    } else {
        emu.pc = 2101204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fd4));
    }
}
#[inline(always)]
pub fn block_0x00200fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 27usize, 0u32, 2101208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2101216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fe0));
    } else {
        emu.pc = 2101212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200fdc));
    }
}
#[inline(always)]
pub fn block_0x00200fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(25usize, 20usize, 2u32, 2101216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2101216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00200fe0));
}
#[inline(always)]
pub fn block_0x00200fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2101216u32, 36864u32, 2101220u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00200fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2101228u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2101232u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2101236u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2101240u32);
    emu.apc_no_count(1usize, 2101240u32, 40960u32, 2101244u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2103788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2103788u32));
    } else {
        emu.pc = 2101252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201004));
    }
}
#[inline(always)]
pub fn block_0x00201004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 10usize, 0u32, 2101256u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2101260u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2101264u32);
    emu.apc_no_count(1usize, 2101264u32, 36864u32, 2101268u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101272u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00201018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 270u32, 2101276u32);
    emu.adi_no_count(21usize, 0usize, 180u32, 2101280u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2101496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f8));
    } else {
        emu.pc = 2101284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201024));
    }
}
#[inline(always)]
pub fn block_0x00201024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2101496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f8));
    } else {
        emu.pc = 2101288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201028));
    }
}
#[inline(always)]
pub fn block_0x00201028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2101292u32);
    emu.adi_no_count(15usize, 0usize, 0u32, 2101296u32);
    emu.adi_no_count(13usize, 0usize, 4294967292u32, 2101300u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2101304u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2101304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201038));
}
#[inline(always)]
pub fn block_0x00201038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2101308u32);
    emu.sri_no_count(5usize, 13usize, 2u32, 2101312u32);
    emu.adi_no_count(16usize, 15usize, 1u32, 2101316u32);
    emu.xri_no_count(6usize, 15usize, 4294967295u32, 2101320u32);
    emu.adr_no_count(6usize, 20usize, 6usize, 2101324u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2101328u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2101328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00201050));
}
#[inline(always)]
pub fn block_0x00201050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2101380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201084));
    } else {
        emu.pc = 2101332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201054));
    }
}
#[inline(always)]
pub fn block_0x00201054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2101364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201074));
    } else {
        emu.pc = 2101336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201058));
    }
}
#[inline(always)]
pub fn block_0x00201058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2101340u32);
    emu.adi_no_count(7usize, 15usize, 0u32, 2101344u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2101388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020108c));
    } else {
        emu.pc = 2101348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201064));
    }
}
#[inline(always)]
pub fn block_0x00201064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(7usize, 17usize, 4294967295u32, 2101352u32);
    emu.adr_no_count(7usize, 18usize, 7usize, 2101356u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2101360u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020108c));
}
#[inline(always)]
pub fn block_0x00201074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(11usize, 17usize, 4294967295u32, 2101368u32);
    emu.adr_no_count(11usize, 18usize, 11usize, 2101372u32);
    emu.adi_no_count(7usize, 6usize, 0u32, 2101376u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2101380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2101388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020108c));
}
#[inline(always)]
pub fn block_0x00201084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 6usize, 0u32, 2101384u32);
    emu.adi_no_count(7usize, 17usize, 0u32, 2101388u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2101388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020108c));
}
#[inline(always)]
pub fn block_0x0020108c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(7usize, 7usize, 25usize, 2101392u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2101396u32);
    emu.adr_no_count(7usize, 7usize, 11usize, 2101400u32);
    emu.adi_no_count(11usize, 7usize, 4u32, 2101404u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2102468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102468u32));
    } else {
        emu.pc = 2101408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010a0));
    }
}
#[inline(always)]
pub fn block_0x002010a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2102488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102488u32));
    } else {
        emu.pc = 2101412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010a4));
    }
}
#[inline(always)]
pub fn block_0x002010a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2102512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102512u32));
    } else {
        emu.pc = 2101416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010a8));
    }
}
#[inline(always)]
pub fn block_0x002010a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 4u32, 2101420u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2102532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2102532u32));
    } else {
        emu.pc = 2101424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010b0));
    }
}
#[inline]
pub fn block_0x002010b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 17usize, 1u32, 2101428u32);
    emu.adr_no_count(7usize, 24usize, 7usize, 2101432u32);
    emu.adr_no_count(10usize, 26usize, 10usize, 2101436u32);
    emu.lbu_no_count(28usize, 10usize, 0u32, 2101440u32);
    emu.lbu_no_count(29usize, 10usize, 1u32, 2101444u32);
    emu.lbu_no_count(30usize, 10usize, 2u32, 2101448u32);
    emu.lbu_no_count(10usize, 10usize, 3u32, 2101452u32);
    emu.sb_no_count(28usize, 7usize, 0u32, 2101456u32);
    emu.sb_no_count(29usize, 7usize, 1u32, 2101460u32);
    emu.sb_no_count(30usize, 7usize, 2u32, 2101464u32);
    emu.sb_no_count(10usize, 7usize, 3u32, 2101468u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2101472u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2101328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201050));
    } else {
        emu.pc = 2101476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010e4));
    }
}
#[inline(always)]
pub fn block_0x002010e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 27usize, 2101480u32);
    emu.adr_no_count(14usize, 14usize, 27usize, 2101484u32);
    emu.adr_no_count(12usize, 12usize, 27usize, 2101488u32);
    emu.adi_no_count(15usize, 16usize, 0u32, 2101492u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2101304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201038));
    } else {
        emu.pc = 2101496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002010f8));
    }
}
#[inline(always)]
pub fn block_0x002010f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 188u32, 2101500u32);
    emu.adi_no_count(11usize, 24usize, 0u32, 2101504u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2101508u32);
    emu.apc_no_count(1usize, 2101508u32, 4096u32, 2101512u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020110c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 196u32, 2101520u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2101524u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2100440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00200cd8));
    } else {
        emu.pc = 2101528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00201118));
    }
}
#[inline(always)]
pub fn block_0x00201118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 192u32, 2101532u32)?;
    emu.lw_no_count(21usize, 2usize, 60u32, 2101536u32)?;
    emu.adi_no_count(11usize, 21usize, 0u32, 2101540u32);
    emu.apc_no_count(1usize, 2101540u32, 151552u32, 2101544u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2101548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
