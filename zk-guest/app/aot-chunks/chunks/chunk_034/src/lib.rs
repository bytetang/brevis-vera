pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2235168u32;
pub const PC_MAX: u32 = 2237392u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 133usize] = [
        block_0x00221b20,
        block_0x00221b4c,
        block_0x00221b5c,
        block_0x00221b68,
        block_0x00221b70,
        block_0x00221b74,
        block_0x00221b9c,
        block_0x00221ba0,
        block_0x00221bbc,
        block_0x00221c04,
        block_0x00221c0c,
        block_0x00221c14,
        block_0x00221c1c,
        block_0x00221c24,
        block_0x00221c3c,
        block_0x00221c44,
        block_0x00221c60,
        block_0x00221c64,
        block_0x00221c6c,
        block_0x00221c74,
        block_0x00221c90,
        block_0x00221c94,
        block_0x00221ca8,
        block_0x00221cac,
        block_0x00221cb4,
        block_0x00221cbc,
        block_0x00221cc0,
        block_0x00221cc8,
        block_0x00221cd0,
        block_0x00221cd8,
        block_0x00221ce4,
        block_0x00221d2c,
        block_0x00221d40,
        block_0x00221d50,
        block_0x00221d54,
        block_0x00221d5c,
        block_0x00221d64,
        block_0x00221d80,
        block_0x00221d88,
        block_0x00221d9c,
        block_0x00221da0,
        block_0x00221da4,
        block_0x00221db8,
        block_0x00221dbc,
        block_0x00221dc0,
        block_0x00221df8,
        block_0x00221e08,
        block_0x00221e0c,
        block_0x00221e20,
        block_0x00221e2c,
        block_0x00221e40,
        block_0x00221e44,
        block_0x00221e48,
        block_0x00221e8c,
        block_0x00221ebc,
        block_0x00221ecc,
        block_0x00221f00,
        block_0x00221f04,
        block_0x00221f14,
        block_0x00221f1c,
        block_0x00221f20,
        block_0x00221f28,
        block_0x00221f50,
        block_0x00221f58,
        block_0x00221f60,
        block_0x00221fa4,
        block_0x00221fc0,
        block_0x00221fc4,
        block_0x00222008,
        block_0x0022200c,
        block_0x0022201c,
        block_0x00222034,
        block_0x00222038,
        block_0x00222040,
        block_0x00222054,
        block_0x00222058,
        block_0x00222064,
        block_0x00222070,
        block_0x0022207c,
        block_0x00222080,
        block_0x0022209c,
        block_0x002220a4,
        block_0x002220b8,
        block_0x002220bc,
        block_0x002220c0,
        block_0x002220d4,
        block_0x002220d8,
        block_0x002220dc,
        block_0x00222118,
        block_0x0022212c,
        block_0x00222138,
        block_0x0022214c,
        block_0x00222150,
        block_0x00222154,
        block_0x002221a0,
        block_0x002221b0,
        block_0x002221b4,
        block_0x002221bc,
        block_0x002221c4,
        block_0x00222230,
        block_0x00222244,
        block_0x00222248,
        block_0x0022224c,
        block_0x0022225c,
        block_0x00222260,
        block_0x00222274,
        block_0x00222278,
        block_0x00222280,
        block_0x002222b8,
        block_0x002222c0,
        block_0x002222c4,
        block_0x002222d4,
        block_0x002222d8,
        block_0x002222e0,
        block_0x002222e4,
        block_0x002222e8,
        block_0x002222ec,
        block_0x002222fc,
        block_0x00222304,
        block_0x00222308,
        block_0x00222310,
        block_0x00222314,
        block_0x0022231c,
        block_0x00222320,
        block_0x0022232c,
        block_0x00222358,
        block_0x00222368,
        block_0x0022236c,
        block_0x00222384,
        block_0x00222388,
        block_0x00222390,
        block_0x00222394,
        block_0x002223d0,
    ];
    const IDX: [u16; 557usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 0u16, 5u16, 6u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 12u16, 0u16, 13u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 19u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16, 25u16, 0u16,
        26u16, 27u16, 0u16, 28u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16,
        35u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 40u16, 41u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        43u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 53u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16,
        61u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 67u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16,
        76u16, 0u16, 0u16, 77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 80u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        84u16, 85u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 88u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16,
        0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 93u16,
        94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 96u16, 97u16, 0u16,
        98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16, 102u16, 103u16, 0u16,
        0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        109u16, 0u16, 110u16, 111u16, 0u16, 0u16, 0u16, 112u16, 113u16, 0u16, 114u16,
        115u16, 116u16, 117u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 120u16, 0u16,
        121u16, 122u16, 0u16, 123u16, 124u16, 0u16, 0u16, 125u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 127u16, 128u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 129u16, 130u16, 0u16, 131u16, 132u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 133u16,
    ];
    if pc < 2235168u32 || pc > 2237392u32 {
        return None;
    }
    let word_offset = ((pc - 2235168u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00221b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2235172u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2235176u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2235180u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2235184u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2235188u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2235192u32)?;
    emu.adi_no_count(8usize, 14usize, 0u32, 2235196u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2235200u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2235204u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2235208u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2235248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b70));
    } else {
        emu.pc = 2235212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b4c));
    }
}
#[inline(always)]
pub fn block_0x00221b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 16u32, 2235216u32)?;
    emu.adi_no_count(19usize, 10usize, 0u32, 2235220u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2235224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2235228u32;
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
pub fn block_0x00221b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2235232u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2235236u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2235248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b70));
    } else {
        emu.pc = 2235240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b68));
    }
}
#[inline(always)]
pub fn block_0x00221b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2235244u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2235248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221ba0));
}
#[inline(always)]
pub fn block_0x00221b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b9c));
    } else {
        emu.pc = 2235252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221b74));
    }
}
#[inline]
pub fn block_0x00221b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2235256u32)?;
    emu.adi_no_count(11usize, 9usize, 0u32, 2235260u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2235264u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2235268u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2235272u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2235276u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2235280u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2235284u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2235288u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2235292u32;
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
pub fn block_0x00221b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2235296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2235296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221ba0));
}
#[inline(always)]
pub fn block_0x00221ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2235300u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2235304u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2235308u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2235312u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2235316u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2235320u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2235324u32;
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
pub fn block_0x00221bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2235328u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2235332u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2235336u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2235340u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2235344u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2235348u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2235352u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2235356u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2235360u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2235364u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2235368u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2235372u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2235376u32);
    emu.lw_no_count(18usize, 10usize, 8u32, 2235380u32)?;
    let a = 0u32.wrapping_add(402653184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2235384u32;
    emu.update_insn_clock();
    emu.anr_no_count(12usize, 18usize, 12usize, 2235388u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2235392u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2235620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ce4));
    } else {
        emu.pc = 2235396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c04));
    }
}
#[inline(always)]
pub fn block_0x00221c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 18usize, 3u32, 2235400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2235500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c6c));
    } else {
        emu.pc = 2235404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c0c));
    }
}
#[inline(always)]
pub fn block_0x00221c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2235408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2235692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d2c));
    } else {
        emu.pc = 2235412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c14));
    }
}
#[inline(always)]
pub fn block_0x00221c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2235416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2235452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c3c));
    } else {
        emu.pc = 2235420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c1c));
    }
}
#[inline(always)]
pub fn block_0x00221c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 8usize, 9usize, 2235424u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2235428u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2235428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c24));
}
#[inline(always)]
pub fn block_0x00221c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2235432u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2235436u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2235440u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2235444u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2235448u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2235428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c24));
    } else {
        emu.pc = 2235452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c3c));
    }
}
#[inline(always)]
pub fn block_0x00221c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 10usize, 12u32, 2235456u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2235620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ce4));
    } else {
        emu.pc = 2235460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c44));
    }
}
#[inline(always)]
pub fn block_0x00221c44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 0u32, 2235464u32);
    emu.sbr_no_count(20usize, 12usize, 11usize, 2235468u32);
    emu.sli_no_count(11usize, 18usize, 1u32, 2235472u32);
    emu.sri_no_count(11usize, 11usize, 30u32, 2235476u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2235480u32);
    emu.sli_no_count(18usize, 18usize, 11u32, 2235484u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2235732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d54));
    } else {
        emu.pc = 2235488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c60));
    }
}
#[inline(always)]
pub fn block_0x00221c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d64));
    } else {
        emu.pc = 2235492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c64));
    }
}
#[inline(always)]
pub fn block_0x00221c64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 20usize, 0u32, 2235496u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2235500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221d64));
}
#[inline(always)]
pub fn block_0x00221c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 10usize, 14u32, 2235504u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2235896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221df8));
    } else {
        emu.pc = 2235508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c74));
    }
}
#[inline(always)]
pub fn block_0x00221c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 8usize, 9usize, 2235512u32);
    emu.adi_no_count(14usize, 0usize, 224u32, 2235516u32);
    emu.adi_no_count(15usize, 0usize, 240u32, 2235520u32);
    emu.adi_no_count(16usize, 8usize, 0u32, 2235524u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2235528u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2235532u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2235536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221ca8));
}
#[inline(always)]
pub fn block_0x00221c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 1u32, 2235540u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2235540u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c94));
}
#[inline(always)]
pub fn block_0x00221c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 16usize, 9usize, 2235544u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2235548u32);
    emu.sbr_no_count(9usize, 17usize, 16usize, 2235552u32);
    emu.adi_no_count(16usize, 17usize, 0u32, 2235556u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2235608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cd8));
    } else {
        emu.pc = 2235560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ca8));
    }
}
#[inline(always)]
pub fn block_0x00221ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2235608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cd8));
    } else {
        emu.pc = 2235564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cac));
    }
}
#[inline(always)]
pub fn block_0x00221cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 16usize, 0u32, 2235568u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2235536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c90));
    } else {
        emu.pc = 2235572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cb4));
    }
}
#[inline(always)]
pub fn block_0x00221cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 17usize, 255u32, 2235576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2235592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cc8));
    } else {
        emu.pc = 2235580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cbc));
    }
}
#[inline(always)]
pub fn block_0x00221cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2235600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cd0));
    } else {
        emu.pc = 2235584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221cc0));
    }
}
#[inline(always)]
pub fn block_0x00221cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 4u32, 2235588u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2235592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c94));
}
#[inline(always)]
pub fn block_0x00221cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 2u32, 2235596u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2235600u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c94));
}
#[inline(always)]
pub fn block_0x00221cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 3u32, 2235604u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2235608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c94));
}
#[inline(always)]
pub fn block_0x00221cd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 12usize, 2235612u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2235616u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2235460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c44));
    } else {
        emu.pc = 2235620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ce4));
    }
}
#[inline]
pub fn block_0x00221ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2235624u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2235628u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2235632u32)?;
    emu.adi_no_count(11usize, 8usize, 0u32, 2235636u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2235640u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2235644u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2235648u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2235652u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2235656u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2235660u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2235664u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2235668u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2235672u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2235676u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2235680u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2235684u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2235688u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2235692u32;
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
pub fn block_0x00221d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2235696u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2235700u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2235704u32);
    emu.apc_no_count(1usize, 2235704u32, 16384u32, 2235708u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2235712u32;
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
pub fn block_0x00221d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2235716u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2235720u32);
    emu.lhu_no_count(12usize, 19usize, 12u32, 2235724u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2235620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ce4));
    } else {
        emu.pc = 2235728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d50));
    }
}
#[inline(always)]
pub fn block_0x00221d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2235732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221c44));
}
#[inline(always)]
pub fn block_0x00221d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 2u32, 2235736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2235748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d64));
    } else {
        emu.pc = 2235740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d5c));
    }
}
#[inline(always)]
pub fn block_0x00221d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 20usize, 16u32, 2235744u32);
    emu.sri_no_count(21usize, 11usize, 17u32, 2235748u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2235748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221d64));
}
#[inline(always)]
pub fn block_0x00221d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2235752u32);
    emu.sri_no_count(18usize, 18usize, 11u32, 2235756u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2235760u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2235764u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2235768u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2235772u32);
    emu.anr_no_count(25usize, 21usize, 24usize, 2235776u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2235776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221d80));
}
#[inline(always)]
pub fn block_0x00221d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 23usize, 24usize, 2235780u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2235812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221da4));
    } else {
        emu.pc = 2235784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d88));
    }
}
#[inline(always)]
pub fn block_0x00221d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2235788u32)?;
    emu.adi_no_count(23usize, 23usize, 1u32, 2235792u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2235796u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2235800u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2235804u32;
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
pub fn block_0x00221d9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221d80));
    } else {
        emu.pc = 2235808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221da0));
    }
}
#[inline(always)]
pub fn block_0x00221da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2235812u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221dbc));
}
#[inline(always)]
pub fn block_0x00221da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 22usize, 12u32, 2235816u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2235820u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2235824u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2235828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2235832u32;
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
pub fn block_0x00221db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e0c));
    } else {
        emu.pc = 2235836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221dbc));
    }
}
#[inline(always)]
pub fn block_0x00221dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2235840u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2235840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221dc0));
}
#[inline]
pub fn block_0x00221dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2235844u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2235848u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2235852u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2235856u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2235860u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2235864u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2235868u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2235872u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2235876u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2235880u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2235884u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2235888u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2235892u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2235896u32;
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
pub fn block_0x00221df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2235900u32);
    emu.sbr_no_count(11usize, 11usize, 0usize, 2235904u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2235908u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2235460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221c44));
    } else {
        emu.pc = 2235912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e08));
    }
}
#[inline(always)]
pub fn block_0x00221e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2235916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221ce4));
}
#[inline(always)]
pub fn block_0x00221e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2235920u32);
    emu.sbr_no_count(10usize, 20usize, 21usize, 2235924u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2235928u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967295u32, 2235932u32);
    emu.anr_no_count(21usize, 10usize, 20usize, 2235936u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2235936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221e20));
}
#[inline(always)]
pub fn block_0x00221e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 9usize, 20usize, 2235940u32);
    emu.sltru_no_count(8usize, 10usize, 21usize, 2235944u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2235840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221dc0));
    } else {
        emu.pc = 2235948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e2c));
    }
}
#[inline(always)]
pub fn block_0x00221e2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2235952u32)?;
    emu.adi_no_count(9usize, 9usize, 1u32, 2235956u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2235960u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2235964u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2235968u32;
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
pub fn block_0x00221e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2235936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e20));
    } else {
        emu.pc = 2235972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e44));
    }
}
#[inline(always)]
pub fn block_0x00221e44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2235976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2235840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221dc0));
}
#[inline]
pub fn block_0x00221e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2235980u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2235984u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2235988u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2235992u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2235996u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2236000u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2236004u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2236008u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2236012u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2236016u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2236020u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2236024u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2236028u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2236032u32);
    emu.lhu_no_count(18usize, 10usize, 12u32, 2236036u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2236040u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2236256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f60));
    } else {
        emu.pc = 2236044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221e8c));
    }
}
#[inline]
pub fn block_0x00221e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 8usize, 8u32, 2236048u32)?;
    emu.lw_no_count(10usize, 12usize, 0u32, 2236052u32)?;
    emu.lw_no_count(11usize, 12usize, 4u32, 2236056u32)?;
    emu.lw_no_count(13usize, 12usize, 8u32, 2236060u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2236064u32)?;
    emu.lw_no_count(22usize, 8usize, 12u32, 2236068u32)?;
    emu.sli_no_count(14usize, 21usize, 7u32, 2236072u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2236076u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2236080u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2236084u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2236088u32)?;
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2236324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221fa4));
    } else {
        emu.pc = 2236092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ebc));
    }
}
#[inline(always)]
pub fn block_0x00221ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2236096u32)?;
    emu.adi_no_count(11usize, 21usize, 0u32, 2236100u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2236104u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2236424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222008));
    } else {
        emu.pc = 2236108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ecc));
    }
}
#[inline]
pub fn block_0x00221ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2236112u32);
    emu.lw_no_count(5usize, 2usize, 8u32, 2236116u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2236120u32);
    let a = 0u32.wrapping_add(393216u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2236124u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(524288u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2236128u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2236132u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(516096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2236136u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294967286u32, 2236140u32);
    emu.adi_no_count(16usize, 16usize, 4294967196u32, 2236144u32);
    emu.adi_no_count(17usize, 17usize, 4294966296u32, 2236148u32);
    emu.adi_no_count(5usize, 5usize, 4u32, 2236152u32);
    emu.adi_no_count(6usize, 6usize, 4294965488u32, 2236156u32);
    emu.add_memory_rw_events(13usize);
    let return_addr = 2236160u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221f14));
}
#[inline(always)]
pub fn block_0x00221f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 5usize, 0u32, 2236164u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2236164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221f04));
}
#[inline(always)]
pub fn block_0x00221f04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 7usize, 12usize, 2236168u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2236172u32);
    emu.adi_no_count(5usize, 5usize, 12u32, 2236176u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2236428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022200c));
    } else {
        emu.pc = 2236180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f14));
    }
}
#[inline(always)]
pub fn block_0x00221f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(7usize, 5usize, 4294967292u32, 2236184u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2236160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f00));
    } else {
        emu.pc = 2236188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f1c));
    }
}
#[inline(always)]
pub fn block_0x00221f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2236240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f50));
    } else {
        emu.pc = 2236192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f20));
    }
}
#[inline(always)]
pub fn block_0x00221f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(7usize, 5usize, 4294967294u32, 2236196u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2236248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f58));
    } else {
        emu.pc = 2236200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221f28));
    }
}
#[inline]
pub fn block_0x00221f28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 7usize, 15usize, 2236204u32);
    emu.adr_no_count(29usize, 7usize, 16usize, 2236208u32);
    emu.anr_no_count(28usize, 28usize, 29usize, 2236212u32);
    emu.adr_no_count(29usize, 7usize, 17usize, 2236216u32);
    emu.adr_no_count(7usize, 7usize, 6usize, 2236220u32);
    emu.anr_no_count(7usize, 29usize, 7usize, 2236224u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2236228u32);
    emu.sri_no_count(7usize, 7usize, 17u32, 2236232u32);
    emu.adi_no_count(7usize, 7usize, 1u32, 2236236u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2236240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221f04));
}
#[inline(always)]
pub fn block_0x00221f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 5usize, 4u32, 2236244u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2236248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221f04));
}
#[inline(always)]
pub fn block_0x00221f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 0usize, 1u32, 2236252u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2236256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00221f04));
}
#[inline]
pub fn block_0x00221f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2236260u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2236264u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2236268u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2236272u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2236276u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2236280u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2236284u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2236288u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2236292u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2236296u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2236300u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2236304u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2236308u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2236312u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2236316u32);
    emu.apc_no_count(6usize, 2236316u32, 0u32, 2236320u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2236324u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00221fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 4u32, 2236328u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2236332u32)?;
    emu.lw_no_count(11usize, 2usize, 0u32, 2236336u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2236340u32)?;
    emu.lw_no_count(13usize, 12usize, 12u32, 2236344u32)?;
    emu.adi_no_count(12usize, 9usize, 0u32, 2236348u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2236352u32;
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
pub fn block_0x00221fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002220d8));
    } else {
        emu.pc = 2236356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221fc4));
    }
}
#[inline]
pub fn block_0x00221fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2236360u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2236364u32;
    emu.update_insn_clock();
    emu.anr_no_count(11usize, 21usize, 11usize, 2236368u32);
    emu.adi_no_count(12usize, 12usize, 48u32, 2236372u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2236376u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2236380u32);
    emu.sri_no_count(9usize, 9usize, 16u32, 2236384u32);
    emu.sbr_no_count(12usize, 18usize, 9usize, 2236388u32);
    emu.sltru_no_count(13usize, 18usize, 12usize, 2236392u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2236396u32);
    emu.anr_no_count(18usize, 13usize, 12usize, 2236400u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2236404u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2236408u32)?;
    emu.sw_no_count(0usize, 2usize, 4u32, 2236412u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2236416u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2236420u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2236108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00221ecc));
    } else {
        emu.pc = 2236424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222008));
    }
}
#[inline(always)]
pub fn block_0x00222008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2236428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2236428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022200c));
}
#[inline(always)]
pub fn block_0x0022200c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 10usize, 2236432u32);
    emu.sli_no_count(12usize, 18usize, 16u32, 2236436u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2236440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2236480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222040));
    } else {
        emu.pc = 2236444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022201c));
    }
}
#[inline(always)]
pub fn block_0x0022201c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(20usize, 18usize, 10usize, 2236448u32);
    emu.sli_no_count(10usize, 11usize, 1u32, 2236452u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2236456u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2236460u32);
    emu.sli_no_count(11usize, 11usize, 11u32, 2236464u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2236516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222064));
    } else {
        emu.pc = 2236468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222034));
    }
}
#[inline(always)]
pub fn block_0x00222034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022207c));
    } else {
        emu.pc = 2236472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222038));
    }
}
#[inline(always)]
pub fn block_0x00222038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2236476u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2236480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222080));
}
#[inline(always)]
pub fn block_0x00222040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2236484u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2236488u32)?;
    emu.adi_no_count(12usize, 2usize, 0u32, 2236492u32);
    emu.apc_no_count(1usize, 2236492u32, 0u32, 2236496u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2236500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00222054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2236504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2236504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222058));
}
#[inline(always)]
pub fn block_0x00222058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 8usize, 8u32, 2236508u32)?;
    emu.sw_no_count(22usize, 8usize, 12u32, 2236512u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2236516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002220dc));
}
#[inline(always)]
pub fn block_0x00222064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 2u32, 2236520u32);
    emu.adi_no_count(23usize, 20usize, 0u32, 2236524u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2236544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222080));
    } else {
        emu.pc = 2236528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222070));
    }
}
#[inline(always)]
pub fn block_0x00222070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 20usize, 16u32, 2236532u32);
    emu.sri_no_count(23usize, 10usize, 17u32, 2236536u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2236540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222080));
}
#[inline(always)]
pub fn block_0x0022207c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 20usize, 0u32, 2236544u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2236544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222080));
}
#[inline(always)]
pub fn block_0x00222080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2236548u32);
    emu.sri_no_count(9usize, 11usize, 11u32, 2236552u32);
    emu.lw_no_count(18usize, 8usize, 0u32, 2236556u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2236560u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2236564u32;
    emu.update_insn_clock();
    emu.adi_no_count(25usize, 25usize, 4294967295u32, 2236568u32);
    emu.anr_no_count(26usize, 23usize, 25usize, 2236572u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2236572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022209c));
}
#[inline(always)]
pub fn block_0x0022209c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 24usize, 25usize, 2236576u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2236608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002220c0));
    } else {
        emu.pc = 2236580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002220a4));
    }
}
#[inline(always)]
pub fn block_0x002220a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 16u32, 2236584u32)?;
    emu.adi_no_count(24usize, 24usize, 1u32, 2236588u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2236592u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2236596u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2236600u32;
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
pub fn block_0x002220b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022209c));
    } else {
        emu.pc = 2236604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002220bc));
    }
}
#[inline(always)]
pub fn block_0x002220bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2236608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236632u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002220d8));
}
#[inline(always)]
pub fn block_0x002220c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 0u32, 2236612u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2236616u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2236620u32);
    emu.apc_no_count(1usize, 2236620u32, 0u32, 2236624u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2236628u32;
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
pub fn block_0x002220d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222118));
    } else {
        emu.pc = 2236632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002220d8));
    }
}
#[inline(always)]
pub fn block_0x002220d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2236636u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2236636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002220dc));
}
#[inline]
pub fn block_0x002220dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2236640u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2236644u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2236648u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2236652u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2236656u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2236660u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2236664u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2236668u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2236672u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2236676u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2236680u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2236684u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2236688u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2236692u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2236696u32;
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
pub fn block_0x00222118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2236700u32);
    emu.sbr_no_count(10usize, 20usize, 23usize, 2236704u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2236708u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2236712u32);
    emu.anr_no_count(25usize, 10usize, 23usize, 2236716u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2236716u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022212c));
}
#[inline(always)]
pub fn block_0x0022212c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 24usize, 23usize, 2236720u32);
    emu.sltru_no_count(20usize, 10usize, 25usize, 2236724u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2236504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222058));
    } else {
        emu.pc = 2236728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222138));
    }
}
#[inline(always)]
pub fn block_0x00222138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 16u32, 2236732u32)?;
    emu.adi_no_count(24usize, 24usize, 1u32, 2236736u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2236740u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2236744u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2236748u32;
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
pub fn block_0x0022214c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022212c));
    } else {
        emu.pc = 2236752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222150));
    }
}
#[inline(always)]
pub fn block_0x00222150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2236756u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222058));
}
#[inline]
pub fn block_0x00222154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2236760u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2236764u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2236768u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2236772u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2236776u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2236780u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2236784u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2236788u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2236792u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2236796u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2236800u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2236804u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2236808u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2236812u32)?;
    emu.adi_no_count(18usize, 12usize, 0u32, 2236816u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2236820u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2236824u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2236828u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2236860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221bc));
    } else {
        emu.pc = 2236832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221a0));
    }
}
#[inline(always)]
pub fn block_0x002221a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2236836u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2236840u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2236844u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2236848u32;
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
pub fn block_0x002221b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221bc));
    } else {
        emu.pc = 2236852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221b4));
    }
}
#[inline(always)]
pub fn block_0x002221b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2236856u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2236860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237332u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222394));
}
#[inline(always)]
pub fn block_0x002221bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 12u32, 2236864u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2237332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222394));
    } else {
        emu.pc = 2236868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221c4));
    }
}
#[inline(never)]
pub fn block_0x002221c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 18usize, 8u32, 2236872u32)?;
    emu.sli_no_count(11usize, 10usize, 2u32, 2236876u32);
    emu.sli_no_count(10usize, 10usize, 4u32, 2236880u32);
    emu.adi_no_count(22usize, 0usize, 65u32, 2236884u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2236888u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 1192u32, 2236892u32);
    emu.adi_no_count(23usize, 0usize, 64u32, 2236896u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2236900u32);
    emu.adi_no_count(25usize, 0usize, 2u32, 2236904u32);
    let a = 0u32.wrapping_add(3435986944u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2236908u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 0usize, 10u32, 2236912u32);
    let a = 0u32.wrapping_add(393216u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2236916u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(524288u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2236920u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2236924u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(516096u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2236928u32;
    emu.update_insn_clock();
    emu.sbr_no_count(10usize, 10usize, 11usize, 2236932u32);
    emu.adi_no_count(12usize, 12usize, 4294967286u32, 2236936u32);
    emu.sw_no_count(12usize, 2usize, 16u32, 2236940u32)?;
    emu.adi_no_count(11usize, 13usize, 4294967196u32, 2236944u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2236948u32)?;
    emu.adi_no_count(11usize, 14usize, 4294966296u32, 2236952u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2236956u32)?;
    emu.adr_no_count(26usize, 20usize, 10usize, 2236960u32);
    emu.adi_no_count(10usize, 20usize, 12u32, 2236964u32);
    emu.adi_no_count(11usize, 15usize, 4294965488u32, 2236968u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2236972u32)?;
    emu.add_memory_rw_events(27usize);
    let return_addr = 2236976u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022224c));
}
#[inline(always)]
pub fn block_0x00222230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 4u32, 2236980u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2236984u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2236988u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2236992u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2236996u32;
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
pub fn block_0x00222244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2237292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022236c));
    } else {
        emu.pc = 2237000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222248));
    }
}
#[inline(always)]
pub fn block_0x00222248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2237004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002221b4));
}
#[inline(always)]
pub fn block_0x0022224c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 20usize, 0u32, 2237008u32);
    emu.lhu_no_count(11usize, 20usize, 0u32, 2237012u32)?;
    emu.adi_no_count(20usize, 10usize, 0u32, 2237016u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2237112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222b8));
    } else {
        emu.pc = 2237020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022225c));
    }
}
#[inline(always)]
pub fn block_0x0022225c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2236976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222230));
    } else {
        emu.pc = 2237024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222260));
    }
}
#[inline(always)]
pub fn block_0x00222260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 12usize, 2u32, 2237028u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2237032u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2237036u32);
    emu.lhu_no_count(11usize, 12usize, 0u32, 2237040u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2237192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222308));
    } else {
        emu.pc = 2237044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222274));
    }
}
#[inline(always)]
pub fn block_0x00222274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2237200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222310));
    } else {
        emu.pc = 2237048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222278));
    }
}
#[inline(always)]
pub fn block_0x00222278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 12usize, 2u32, 2237052u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2237320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222388));
    } else {
        emu.pc = 2237056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222280));
    }
}
#[inline]
pub fn block_0x00222280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 16u32, 2237060u32)?;
    emu.adr_no_count(12usize, 11usize, 12usize, 2237064u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2237068u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2237072u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2237076u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2237080u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2237084u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2237088u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2237092u32);
    emu.anr_no_count(11usize, 13usize, 11usize, 2237096u32);
    emu.xrr_no_count(11usize, 12usize, 11usize, 2237100u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2237104u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2237108u32);
    emu.add_memory_rw_events(14usize);
    let return_addr = 2237112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222314));
}
#[inline(always)]
pub fn block_0x002222b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 12usize, 4u32, 2237116u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2237156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222e4));
    } else {
        emu.pc = 2237120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222c0));
    }
}
#[inline(always)]
pub fn block_0x002222c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2237124u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2237124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002222c4));
}
#[inline(always)]
pub fn block_0x002222c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 64u32, 2237128u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2237132u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2237136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2237140u32;
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
pub fn block_0x002222d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221b4));
    } else {
        emu.pc = 2237144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222d8));
    }
}
#[inline(always)]
pub fn block_0x002222d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 4294967232u32, 2237148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2237124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222c4));
    } else {
        emu.pc = 2237152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222e0));
    }
}
#[inline(always)]
pub fn block_0x002222e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2237156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002222ec));
}
#[inline(always)]
pub fn block_0x002222e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2237292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022236c));
    } else {
        emu.pc = 2237160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002222e8));
    }
}
#[inline(always)]
pub fn block_0x002222e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2237164u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2237164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002222ec));
}
#[inline(always)]
pub fn block_0x002222ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2237168u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2237172u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2237176u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2237180u32;
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
pub fn block_0x002222fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 65u32, 2237184u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2237292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022236c));
    } else {
        emu.pc = 2237188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222304));
    }
}
#[inline(always)]
pub fn block_0x00222304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2237192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2236852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002221b4));
}
#[inline(always)]
pub fn block_0x00222308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 8u32, 2237196u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2237200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222314));
}
#[inline(always)]
pub fn block_0x00222310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 4u32, 2237204u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2237204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222314));
}
#[inline(always)]
pub fn block_0x00222314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 6u32, 2237208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2237392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002223d0));
    } else {
        emu.pc = 2237212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022231c));
    }
}
#[inline(always)]
pub fn block_0x0022231c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2237272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222358));
    } else {
        emu.pc = 2237216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222320));
    }
}
#[inline(always)]
pub fn block_0x00222320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2237220u32);
    emu.adi_no_count(13usize, 2usize, 19u32, 2237224u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2237228u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2237228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022232c));
}
#[inline]
pub fn block_0x0022232c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 10usize, 16u32, 2237232u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2237236u32);
    emu.mulhu_no_count(14usize, 14usize, 27usize, 2237240u32);
    emu.sri_no_count(14usize, 14usize, 19u32, 2237244u32);
    emu.mul_no_count(15usize, 14usize, 21usize, 2237248u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2237252u32);
    emu.ori_no_count(10usize, 10usize, 48u32, 2237256u32);
    emu.sb_no_count(10usize, 13usize, 0u32, 2237260u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2237264u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2237268u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2237228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022232c));
    } else {
        emu.pc = 2237272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222358));
    }
}
#[inline(always)]
pub fn block_0x00222358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 8usize, 12u32, 2237276u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2237280u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2237284u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2237288u32;
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
pub fn block_0x00222368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2236852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002221b4));
    } else {
        emu.pc = 2237292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022236c));
    }
}
#[inline(always)]
pub fn block_0x0022236c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 20usize, 26usize, 2237296u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2237300u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2237304u32);
    emu.ani_no_count(10usize, 10usize, 12u32, 2237308u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2237312u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2237004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022224c));
    } else {
        emu.pc = 2237316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00222384));
    }
}
#[inline(always)]
pub fn block_0x00222384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2237320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222390));
}
#[inline(always)]
pub fn block_0x00222388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2237324u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2237328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2237216u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222320));
}
#[inline(always)]
pub fn block_0x00222390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2237332u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2237332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00222394));
}
#[inline]
pub fn block_0x00222394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2237336u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2237340u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2237344u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2237348u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2237352u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2237356u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2237360u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2237364u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2237368u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2237372u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2237376u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2237380u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2237384u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2237388u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237392u32;
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
pub fn block_0x002223d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2237396u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1256u32, 2237400u32);
    emu.adi_no_count(11usize, 0usize, 5u32, 2237404u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2237408u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2237412u32);
    emu.apc_no_count(1usize, 2237412u32, 12288u32, 2237416u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2237420u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
