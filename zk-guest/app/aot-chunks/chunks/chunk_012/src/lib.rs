pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2145672u32;
pub const PC_MAX: u32 = 2147944u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020bd88,
        block_0x0020bdbc,
        block_0x0020bdc4,
        block_0x0020bdd4,
        block_0x0020bdd8,
        block_0x0020bde4,
        block_0x0020bde8,
        block_0x0020bdf8,
        block_0x0020bdfc,
        block_0x0020be08,
        block_0x0020be10,
        block_0x0020be14,
        block_0x0020be3c,
        block_0x0020be44,
        block_0x0020be4c,
        block_0x0020be5c,
        block_0x0020be84,
        block_0x0020be8c,
        block_0x0020be9c,
        block_0x0020bec8,
        block_0x0020bed0,
        block_0x0020bed4,
        block_0x0020bedc,
        block_0x0020bee8,
        block_0x0020bef8,
        block_0x0020bf08,
        block_0x0020bf10,
        block_0x0020bf30,
        block_0x0020bf4c,
        block_0x0020bf50,
        block_0x0020bf6c,
        block_0x0020bf74,
        block_0x0020bfa0,
        block_0x0020bfd8,
        block_0x0020c004,
        block_0x0020c034,
        block_0x0020c048,
        block_0x0020c070,
        block_0x0020c090,
        block_0x0020c09c,
        block_0x0020c0d8,
        block_0x0020c0f0,
        block_0x0020c124,
        block_0x0020c140,
        block_0x0020c148,
        block_0x0020c178,
        block_0x0020c190,
        block_0x0020c1a4,
        block_0x0020c1b4,
        block_0x0020c1d0,
        block_0x0020c1d8,
        block_0x0020c1e0,
        block_0x0020c200,
        block_0x0020c20c,
        block_0x0020c220,
        block_0x0020c234,
        block_0x0020c244,
        block_0x0020c27c,
        block_0x0020c284,
        block_0x0020c2c0,
        block_0x0020c2d8,
        block_0x0020c308,
        block_0x0020c324,
        block_0x0020c32c,
        block_0x0020c358,
        block_0x0020c370,
        block_0x0020c384,
        block_0x0020c394,
        block_0x0020c3ac,
        block_0x0020c3b4,
        block_0x0020c3d4,
        block_0x0020c3e0,
        block_0x0020c3f4,
        block_0x0020c408,
        block_0x0020c418,
        block_0x0020c448,
        block_0x0020c454,
        block_0x0020c45c,
        block_0x0020c464,
        block_0x0020c470,
        block_0x0020c478,
        block_0x0020c498,
        block_0x0020c4c4,
        block_0x0020c4dc,
        block_0x0020c4e8,
        block_0x0020c504,
        block_0x0020c524,
        block_0x0020c534,
        block_0x0020c540,
        block_0x0020c544,
        block_0x0020c55c,
        block_0x0020c564,
        block_0x0020c56c,
        block_0x0020c580,
        block_0x0020c5a8,
        block_0x0020c5b4,
        block_0x0020c5bc,
        block_0x0020c5c4,
        block_0x0020c5d8,
        block_0x0020c60c,
        block_0x0020c618,
        block_0x0020c630,
        block_0x0020c644,
        block_0x0020c64c,
        block_0x0020c654,
        block_0x0020c65c,
        block_0x0020c668,
    ];
    const IDX: [u16; 569usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 6u16, 7u16, 0u16,
        0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 20u16, 0u16, 21u16, 22u16, 0u16, 23u16, 0u16, 0u16, 24u16, 0u16,
        0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 30u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16,
        0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 64u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 71u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 79u16, 0u16,
        0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 84u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 88u16, 0u16,
        0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16,
        93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16,
        0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16,
        107u16,
    ];
    if pc < 2145672u32 || pc > 2147944u32 {
        return None;
    }
    let word_offset = ((pc - 2145672u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020bd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4151074816u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2145676u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1618087936u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2145680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(228253696u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2145684u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4257644544u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2145688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965826u32, 2145692u32);
    emu.adi_no_count(12usize, 12usize, 1443u32, 2145696u32);
    emu.adi_no_count(13usize, 13usize, 203u32, 2145700u32);
    emu.adi_no_count(14usize, 14usize, 861u32, 2145704u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2145708u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2145712u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2145716u32)?;
    emu.sw_no_count(11usize, 10usize, 12u32, 2145720u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145724u32;
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
pub fn block_0x0020bdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2145728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2145748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdd4));
    } else {
        emu.pc = 2145732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdc4));
    }
}
#[inline(always)]
pub fn block_0x0020bdc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2145736u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2145740u32);
    emu.apc_no_count(6usize, 2145740u32, 4294926336u32, 2145744u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2145748u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bdd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145752u32;
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
pub fn block_0x0020bdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2145756u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2145760u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2145784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdf8));
    } else {
        emu.pc = 2145764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bde4));
    }
}
#[inline(always)]
pub fn block_0x0020bde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdf8));
    } else {
        emu.pc = 2145768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bde8));
    }
}
#[inline(always)]
pub fn block_0x0020bde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 4u32, 2145772u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2145776u32);
    emu.apc_no_count(6usize, 2145776u32, 4294926336u32, 2145780u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2145784u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145788u32;
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
pub fn block_0x0020bdfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2145792u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2145796u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2145812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be14));
    } else {
        emu.pc = 2145800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be08));
    }
}
#[inline(always)]
pub fn block_0x0020be08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 3u32, 2145804u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2145812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be14));
    } else {
        emu.pc = 2145808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be10));
    }
}
#[inline(always)]
pub fn block_0x0020be10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145812u32;
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
pub fn block_0x0020be14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2145816u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2145820u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2145824u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2145828u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2145832u32)?;
    emu.lw_no_count(18usize, 11usize, 4u32, 2145836u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2145840u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2145844u32);
    emu.lw_no_count(9usize, 11usize, 0u32, 2145848u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2145860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be44));
    } else {
        emu.pc = 2145852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be3c));
    }
}
#[inline(always)]
pub fn block_0x0020be3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2145856u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2145860u32;
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
pub fn block_0x0020be44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 4u32, 2145864u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2145884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be5c));
    } else {
        emu.pc = 2145868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be4c));
    }
}
#[inline(always)]
pub fn block_0x0020be4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 8u32, 2145872u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2145876u32);
    emu.apc_no_count(1usize, 2145876u32, 4294926336u32, 2145880u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145884u32;
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
#[inline]
pub fn block_0x0020be5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 12u32, 2145888u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2145892u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2145896u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2145900u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2145904u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2145908u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2145912u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2145916u32);
    emu.apc_no_count(6usize, 2145916u32, 4294926336u32, 2145920u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2145924u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020be84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2145928u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145932u32;
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
pub fn block_0x0020be8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2145936u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2145940u32)?;
    emu.lw_no_count(6usize, 12usize, 12u32, 2145944u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2145948u32;
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
pub fn block_0x0020be9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2145952u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2145956u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2145960u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2145964u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2145968u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2145972u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2145976u32)?;
    emu.lw_no_count(8usize, 10usize, 8u32, 2145980u32)?;
    emu.adi_no_count(12usize, 0usize, 128u32, 2145984u32);
    emu.sri_no_count(19usize, 11usize, 11u32, 2145988u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2146000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bed0));
    } else {
        emu.pc = 2145992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bec8));
    }
}
#[inline(always)]
pub fn block_0x0020bec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2145996u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bee8));
}
#[inline(always)]
pub fn block_0x0020bed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2146012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bedc));
    } else {
        emu.pc = 2146004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bed4));
    }
}
#[inline(always)]
pub fn block_0x0020bed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2146008u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bee8));
}
#[inline(always)]
pub fn block_0x0020bedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2146016u32);
    emu.sltru_no_count(9usize, 0usize, 12usize, 2146020u32);
    emu.adi_no_count(9usize, 9usize, 3u32, 2146024u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2146024u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bee8));
}
#[inline(always)]
pub fn block_0x0020bee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2146028u32)?;
    emu.sbr_no_count(13usize, 12usize, 8usize, 2146032u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2146036u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2146064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf10));
    } else {
        emu.pc = 2146040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bef8));
    }
}
#[inline(always)]
pub fn block_0x0020bef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2146044u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2146048u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2146052u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2146124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf4c));
    } else {
        emu.pc = 2146056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf08));
    }
}
#[inline(always)]
pub fn block_0x0020bf08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(11usize, 12usize, 0u32, 2146060u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146064u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd8));
}
#[inline(always)]
pub fn block_0x0020bf10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2146068u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2146072u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2146076u32);
    emu.adi_no_count(20usize, 11usize, 0u32, 2146080u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2146084u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2146088u32);
    emu.apc_no_count(1usize, 2146088u32, 4294963200u32, 2146092u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 20usize, 0u32, 2146100u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2146104u32);
    emu.lw_no_count(12usize, 18usize, 8u32, 2146108u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2146112u32)?;
    emu.adi_no_count(14usize, 0usize, 128u32, 2146116u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2146120u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2146056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf08));
    } else {
        emu.pc = 2146124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf4c));
    }
}
#[inline(always)]
pub fn block_0x0020bf4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2146156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf6c));
    } else {
        emu.pc = 2146128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf50));
    }
}
#[inline(always)]
pub fn block_0x0020bf50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 6u32, 2146132u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2146136u32);
    emu.ori_no_count(13usize, 13usize, 192u32, 2146140u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2146144u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2146148u32);
    emu.sb_no_count(11usize, 12usize, 1u32, 2146152u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2146156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd8));
}
#[inline(always)]
pub fn block_0x0020bf6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 16u32, 2146160u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2146208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfa0));
    } else {
        emu.pc = 2146164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf74));
    }
}
#[inline]
pub fn block_0x0020bf74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 12u32, 2146168u32);
    emu.sli_no_count(14usize, 11usize, 20u32, 2146172u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2146176u32);
    emu.ori_no_count(13usize, 13usize, 224u32, 2146180u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2146184u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2146188u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2146192u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2146196u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2146200u32);
    emu.sb_no_count(11usize, 12usize, 2u32, 2146204u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2146208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd8));
}
#[inline]
pub fn block_0x0020bfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 11usize, 18u32, 2146212u32);
    emu.sli_no_count(14usize, 11usize, 14u32, 2146216u32);
    emu.sli_no_count(15usize, 11usize, 20u32, 2146220u32);
    emu.ani_no_count(11usize, 11usize, 63u32, 2146224u32);
    emu.ori_no_count(13usize, 13usize, 240u32, 2146228u32);
    emu.sri_no_count(14usize, 14usize, 26u32, 2146232u32);
    emu.sri_no_count(15usize, 15usize, 26u32, 2146236u32);
    emu.adi_no_count(11usize, 11usize, 128u32, 2146240u32);
    emu.adi_no_count(14usize, 14usize, 128u32, 2146244u32);
    emu.adi_no_count(15usize, 15usize, 128u32, 2146248u32);
    emu.sb_no_count(13usize, 12usize, 0u32, 2146252u32);
    emu.sb_no_count(14usize, 12usize, 1u32, 2146256u32);
    emu.sb_no_count(15usize, 12usize, 2u32, 2146260u32);
    emu.sb_no_count(11usize, 12usize, 3u32, 2146264u32);
    emu.add_memory_rw_events(14usize);
    emu.pc = 2146264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd8));
}
#[inline]
pub fn block_0x0020bfd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 9usize, 8usize, 2146268u32);
    emu.sw_no_count(8usize, 10usize, 8u32, 2146272u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2146276u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2146280u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2146284u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2146288u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2146292u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2146296u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2146300u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2146304u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146308u32;
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
pub fn block_0x0020c004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2146312u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2146316u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2146320u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2146324u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2146328u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2146332u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2146336u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2146340u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2146344u32)?;
    emu.lw_no_count(18usize, 9usize, 8u32, 2146348u32)?;
    emu.sbr_no_count(10usize, 10usize, 18usize, 2146352u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2146416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c070));
    } else {
        emu.pc = 2146356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c034));
    }
}
#[inline(always)]
pub fn block_0x0020c034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2146360u32)?;
    emu.adr_no_count(10usize, 10usize, 18usize, 2146364u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2146368u32);
    emu.apc_no_count(1usize, 2146368u32, 4294938624u32, 2146372u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 8usize, 2146380u32);
    emu.sw_no_count(8usize, 9usize, 8u32, 2146384u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2146388u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2146392u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2146396u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2146400u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2146404u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2146408u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2146412u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146416u32;
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
pub fn block_0x0020c070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2146420u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2146424u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2146428u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2146432u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2146436u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2146440u32);
    emu.apc_no_count(1usize, 2146440u32, 4294963200u32, 2146444u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 0u32, 2146452u32);
    emu.lw_no_count(18usize, 9usize, 8u32, 2146456u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2146460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c034));
}
#[inline]
pub fn block_0x0020c09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2146464u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2146468u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2146472u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2146476u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2146480u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2146484u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2146488u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2146492u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2146496u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2146500u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2146504u32)?;
    emu.lw_no_count(20usize, 9usize, 8u32, 2146508u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2146512u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2146516u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2146596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c124));
    } else {
        emu.pc = 2146520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0d8));
    }
}
#[inline(always)]
pub fn block_0x0020c0d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2146524u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2146528u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2146532u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2146536u32);
    emu.apc_no_count(1usize, 2146536u32, 4294938624u32, 2146540u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 8usize, 2146548u32);
    emu.sw_no_count(20usize, 9usize, 8u32, 2146552u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2146556u32);
    emu.sb_no_count(10usize, 18usize, 0u32, 2146560u32);
    emu.sw_no_count(8usize, 18usize, 4u32, 2146564u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2146568u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2146572u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2146576u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2146580u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2146584u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2146588u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2146592u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146596u32;
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
pub fn block_0x0020c124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2146600u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2146604u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2146608u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2146612u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2146616u32);
    emu.apc_no_count(1usize, 2146616u32, 4294963200u32, 2146620u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 9usize, 8u32, 2146628u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0d8));
}
#[inline]
pub fn block_0x0020c148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2146636u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2146640u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2146644u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2146648u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2146652u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2146656u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2146660u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2146664u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2146668u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2146672u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2146676u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2146776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1d8));
    } else {
        emu.pc = 2146680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c178));
    }
}
#[inline(always)]
pub fn block_0x0020c178(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2146684u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2146688u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2146692u32);
    emu.sli_no_count(22usize, 13usize, 3u32, 2146696u32);
    emu.adr_no_count(22usize, 12usize, 22usize, 2146700u32);
    emu.adi_no_count(10usize, 12usize, 4u32, 2146704u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2146704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c190));
}
#[inline(always)]
pub fn block_0x0020c190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2146708u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2146712u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2146716u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2146720u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2146704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c190));
    } else {
        emu.pc = 2146724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1a4));
    }
}
#[inline(always)]
pub fn block_0x0020c1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2146728u32)?;
    emu.lw_no_count(20usize, 19usize, 8u32, 2146732u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2146736u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2146828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c20c));
    } else {
        emu.pc = 2146740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1b4));
    }
}
#[inline(always)]
pub fn block_0x0020c1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2146744u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2146748u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2146752u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2146756u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2146760u32);
    emu.apc_no_count(1usize, 2146760u32, 4294963200u32, 2146764u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1240u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 19usize, 8u32, 2146772u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146828u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c20c));
}
#[inline(always)]
pub fn block_0x0020c1d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2146780u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c244));
}
#[inline(always)]
pub fn block_0x0020c1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2146788u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2146792u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2146796u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2146800u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2146804u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2146808u32);
    emu.apc_no_count(1usize, 2146808u32, 4294963200u32, 2146812u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 23usize, 0u32, 2146820u32);
    emu.lw_no_count(20usize, 19usize, 8u32, 2146824u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2146828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c220));
}
#[inline(always)]
pub fn block_0x0020c20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 0u32, 2146832u32)?;
    emu.lw_no_count(21usize, 18usize, 4u32, 2146836u32)?;
    emu.lw_no_count(11usize, 18usize, 0u32, 2146840u32)?;
    emu.sbr_no_count(10usize, 10usize, 20usize, 2146844u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2146784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1e0));
    } else {
        emu.pc = 2146848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c220));
    }
}
#[inline(always)]
pub fn block_0x0020c220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 4u32, 2146852u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2146856u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2146860u32);
    emu.apc_no_count(1usize, 2146860u32, 4294938624u32, 2146864u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 20usize, 21usize, 2146872u32);
    emu.adi_no_count(18usize, 18usize, 8u32, 2146876u32);
    emu.sw_no_count(20usize, 19usize, 8u32, 2146880u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2146828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c20c));
    } else {
        emu.pc = 2146884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c244));
    }
}
#[inline]
pub fn block_0x0020c244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2146888u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2146892u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2146896u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2146900u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2146904u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2146908u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2146912u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2146916u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2146920u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2146924u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2146928u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2146932u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2146936u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146940u32;
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
pub fn block_0x0020c27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2146944u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146948u32;
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
pub fn block_0x0020c284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2146952u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2146956u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2146960u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2146964u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2146968u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2146972u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2146976u32)?;
    emu.adi_no_count(9usize, 13usize, 0u32, 2146980u32);
    emu.adi_no_count(19usize, 12usize, 0u32, 2146984u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2146988u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2146992u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2146996u32)?;
    emu.sbr_no_count(11usize, 11usize, 20usize, 2147000u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2147004u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2147080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c308));
    } else {
        emu.pc = 2147008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2c0));
    }
}
#[inline(always)]
pub fn block_0x0020c2c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2147012u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2147016u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2147020u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2147024u32);
    emu.apc_no_count(1usize, 2147024u32, 4294938624u32, 2147028u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 20usize, 9usize, 2147036u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2147040u32);
    emu.sw_no_count(9usize, 8usize, 8u32, 2147044u32)?;
    emu.sb_no_count(10usize, 18usize, 0u32, 2147048u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2147052u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2147056u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2147060u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2147064u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2147068u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2147072u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2147076u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147080u32;
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
pub fn block_0x0020c308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2147084u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2147088u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2147092u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2147096u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2147100u32);
    emu.apc_no_count(1usize, 2147100u32, 4294963200u32, 2147104u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147108u32;
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
pub fn block_0x0020c324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 8usize, 8u32, 2147112u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147008u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2c0));
}
#[inline]
pub fn block_0x0020c32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2147120u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2147124u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2147128u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2147132u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2147136u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2147140u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2147144u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2147148u32)?;
    emu.sw_no_count(22usize, 2usize, 0u32, 2147152u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2147156u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2147352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c418));
    } else {
        emu.pc = 2147160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c358));
    }
}
#[inline(always)]
pub fn block_0x0020c358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2147164u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2147168u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2147172u32);
    emu.sli_no_count(21usize, 13usize, 3u32, 2147176u32);
    emu.adr_no_count(21usize, 9usize, 21usize, 2147180u32);
    emu.adi_no_count(10usize, 9usize, 4u32, 2147184u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2147184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c370));
}
#[inline(always)]
pub fn block_0x0020c370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2147188u32)?;
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2147192u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2147196u32);
    emu.adi_no_count(10usize, 10usize, 8u32, 2147200u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2147184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c370));
    } else {
        emu.pc = 2147204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c384));
    }
}
#[inline(always)]
pub fn block_0x0020c384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2147208u32)?;
    emu.lw_no_count(19usize, 18usize, 8u32, 2147212u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2147216u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2147296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c3e0));
    } else {
        emu.pc = 2147220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c394));
    }
}
#[inline(always)]
pub fn block_0x0020c394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2147224u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2147228u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2147232u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2147236u32);
    emu.apc_no_count(1usize, 2147236u32, 4294963200u32, 2147240u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c3ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 8u32, 2147248u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c3e0));
}
#[inline(always)]
pub fn block_0x0020c3b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2147256u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2147260u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2147264u32);
    emu.adi_no_count(22usize, 11usize, 0u32, 2147268u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2147272u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2147276u32);
    emu.apc_no_count(1usize, 2147276u32, 4294963200u32, 2147280u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c3d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 22usize, 0u32, 2147288u32);
    emu.lw_no_count(19usize, 18usize, 8u32, 2147292u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2147296u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147316u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c3f4));
}
#[inline(always)]
pub fn block_0x0020c3e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2147300u32)?;
    emu.lw_no_count(20usize, 9usize, 4u32, 2147304u32)?;
    emu.lw_no_count(11usize, 9usize, 0u32, 2147308u32)?;
    emu.sbr_no_count(10usize, 10usize, 19usize, 2147312u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2147252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c3b4));
    } else {
        emu.pc = 2147316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c3f4));
    }
}
#[inline(always)]
pub fn block_0x0020c3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 4u32, 2147320u32)?;
    emu.adr_no_count(10usize, 10usize, 19usize, 2147324u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2147328u32);
    emu.apc_no_count(1usize, 2147328u32, 4294938624u32, 2147332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 20usize, 2147340u32);
    emu.adi_no_count(9usize, 9usize, 8u32, 2147344u32);
    emu.sw_no_count(19usize, 18usize, 8u32, 2147348u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2147296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c3e0));
    } else {
        emu.pc = 2147352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c418));
    }
}
#[inline]
pub fn block_0x0020c418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2147356u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2147360u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2147364u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2147368u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2147372u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2147376u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2147380u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2147384u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2147388u32)?;
    emu.lw_no_count(22usize, 2usize, 0u32, 2147392u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2147396u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147400u32;
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
pub fn block_0x0020c448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2147404u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2147408u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147412u32;
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
pub fn block_0x0020c454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2147416u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147420u32;
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
pub fn block_0x0020c45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2147420u32, 4096u32, 2147424u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2147428u32;
    emu.write_reg_no_count(5usize, return_addr);
    let target = base.wrapping_add(4294965720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2147432u32);
    emu.lbu_no_count(10usize, 10usize, 13u32, 2147436u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2147448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c478));
    } else {
        emu.pc = 2147440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c470));
    }
}
#[inline(always)]
pub fn block_0x0020c470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2147440u32, 4096u32, 2147444u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 8u32, 2147452u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2147456u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2147460u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2147464u32);
    emu.sb_no_count(13usize, 2usize, 7u32, 2147468u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2147472u32)?;
    emu.apc_no_count(1usize, 2147472u32, 0u32, 2147476u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147480u32;
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
#[inline]
pub fn block_0x0020c498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2147484u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2147488u32)?;
    emu.adi_no_count(11usize, 2usize, 8u32, 2147492u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2147496u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2147500u32;
    emu.update_insn_clock();
    emu.lbu_no_count(13usize, 10usize, 4294967280u32, 2147504u32);
    emu.adi_no_count(14usize, 2usize, 7u32, 2147508u32);
    emu.sw_no_count(11usize, 2usize, 20u32, 2147512u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2147516u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2147520u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2147652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c544));
    } else {
        emu.pc = 2147524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c4));
    }
}
#[inline(always)]
pub fn block_0x0020c4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2147528u32;
    emu.update_insn_clock();
    emu.lw_no_count(19usize, 18usize, 4294967284u32, 2147532u32)?;
    emu.adi_no_count(9usize, 0usize, 1u32, 2147536u32);
    emu.sb_no_count(9usize, 10usize, 4294967280u32, 2147540u32);
    emu.sw_no_count(0usize, 18usize, 4294967284u32, 2147544u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2147652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c544));
    } else {
        emu.pc = 2147548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4dc));
    }
}
#[inline(always)]
pub fn block_0x0020c4dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 8u32, 2147552u32);
    emu.apc_no_count(1usize, 2147552u32, 4096u32, 2147556u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 11usize, 0u32, 2147564u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2147568u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147572u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965688u32, 2147576u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2147580u32);
    emu.apc_no_count(1usize, 2147580u32, 0u32, 2147584u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2147592u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147596u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 18usize, 4294967284u32, 2147600u32)?;
    emu.sb_no_count(9usize, 11usize, 4294967280u32, 2147604u32);
    emu.sw_no_count(19usize, 18usize, 4294967284u32, 2147608u32)?;
    emu.sw_no_count(9usize, 2usize, 32u32, 2147612u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2147616u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c55c));
    } else {
        emu.pc = 2147620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c524));
    }
}
#[inline(always)]
pub fn block_0x0020c524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2147624u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2147628u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2147632u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2147676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c55c));
    } else {
        emu.pc = 2147636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c534));
    }
}
#[inline(always)]
pub fn block_0x0020c534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2147640u32);
    emu.apc_no_count(1usize, 2147640u32, 4096u32, 2147644u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2147652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c55c));
}
#[inline(always)]
pub fn block_0x0020c544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147656u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965728u32, 2147660u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2147664u32);
    emu.adi_no_count(11usize, 2usize, 43u32, 2147668u32);
    emu.apc_no_count(1usize, 2147668u32, 0u32, 2147672u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2147676u32, 4096u32, 2147680u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2147684u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(5usize, 2147684u32, 4096u32, 2147688u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2147692u32;
    emu.write_reg_no_count(5usize, return_addr);
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
pub fn block_0x0020c56c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 12usize, 0u32, 2147696u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2147700u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2147704u32);
    emu.apc_no_count(1usize, 2147704u32, 4096u32, 2147708u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2147716u32);
    emu.lw_no_count(10usize, 19usize, 0u32, 2147720u32)?;
    emu.lw_no_count(11usize, 19usize, 4u32, 2147724u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2147728u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2147732u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2147736u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2147740u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2147744u32);
    emu.apc_no_count(1usize, 2147744u32, 4096u32, 2147748u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 19usize, 8u32, 2147756u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2147760u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c630));
    } else {
        emu.pc = 2147764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5b4));
    }
}
#[inline(always)]
pub fn block_0x0020c5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2147768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c618));
    } else {
        emu.pc = 2147772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5bc));
    }
}
#[inline(always)]
pub fn block_0x0020c5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2147776u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2147932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c65c));
    } else {
        emu.pc = 2147780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c4));
    }
}
#[inline(always)]
pub fn block_0x0020c5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147784u32;
    emu.update_insn_clock();
    emu.lbu_no_count(12usize, 11usize, 8u32, 2147788u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2147792u32);
    emu.sb_no_count(10usize, 11usize, 8u32, 2147796u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2147932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c65c));
    } else {
        emu.pc = 2147800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5d8));
    }
}
#[inline]
pub fn block_0x0020c5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147804u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965848u32, 2147808u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2147812u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2147816u32);
    emu.lw_no_count(13usize, 18usize, 36u32, 2147820u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2147824u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2147828u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2147832u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2147836u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2147840u32);
    emu.adi_no_count(12usize, 2usize, 20u32, 2147844u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2147848u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2147852u32;
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
pub fn block_0x0020c60c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 12u32, 2147856u32);
    emu.lw_no_count(11usize, 2usize, 16u32, 2147860u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2147864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c654));
}
#[inline(always)]
pub fn block_0x0020c618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2147868u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2147872u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2147876u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2147880u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147884u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2147888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c644));
}
#[inline(always)]
pub fn block_0x0020c630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 20u32, 2147892u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2147896u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2147900u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147904u32);
    emu.adi_no_count(14usize, 0usize, 0u32, 2147908u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2147908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c644));
}
#[inline(always)]
pub fn block_0x0020c644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2147908u32, 4096u32, 2147912u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c64c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 20u32, 2147920u32);
    emu.lw_no_count(11usize, 2usize, 24u32, 2147924u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c654));
}
#[inline(always)]
pub fn block_0x0020c654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2147924u32, 4294963200u32, 2147928u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c65c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 0u32, 2147936u32);
    emu.apc_no_count(6usize, 2147936u32, 0u32, 2147940u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2147944u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2147948u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2147952u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2147956u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2147960u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2147964u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2147968u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2147972u32);
    emu.apc_no_count(1usize, 2147972u32, 0u32, 2147976u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
