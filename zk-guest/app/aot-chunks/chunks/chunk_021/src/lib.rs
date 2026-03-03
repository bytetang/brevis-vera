pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2168880u32;
pub const PC_MAX: u32 = 2171488u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00211830,
        block_0x0021183c,
        block_0x00211848,
        block_0x00211854,
        block_0x00211858,
        block_0x0021185c,
        block_0x00211868,
        block_0x00211870,
        block_0x0021187c,
        block_0x00211884,
        block_0x002118a0,
        block_0x002118d4,
        block_0x002118dc,
        block_0x002118e0,
        block_0x002118ec,
        block_0x002118f4,
        block_0x00211900,
        block_0x00211908,
        block_0x00211914,
        block_0x00211920,
        block_0x00211938,
        block_0x00211a2c,
        block_0x00211a38,
        block_0x00211a44,
        block_0x00211a48,
        block_0x00211a4c,
        block_0x00211a64,
        block_0x00211a78,
        block_0x00211a88,
        block_0x00211a90,
        block_0x00211a98,
        block_0x00211ae4,
        block_0x00211af8,
        block_0x00211b08,
        block_0x00211b1c,
        block_0x00211b20,
        block_0x00211b24,
        block_0x00211b28,
        block_0x00211b30,
        block_0x00211b34,
        block_0x00211b38,
        block_0x00211b40,
        block_0x00211b50,
        block_0x00211b78,
        block_0x00211be8,
        block_0x00211bf8,
        block_0x00211c68,
        block_0x00211c6c,
        block_0x00211c90,
        block_0x00211ca0,
        block_0x00211cb4,
        block_0x00211cd0,
        block_0x00211cd4,
        block_0x00211cf4,
        block_0x00211d08,
        block_0x00211d18,
        block_0x00211d2c,
        block_0x00211d48,
        block_0x00211d4c,
        block_0x00211d68,
        block_0x00211da8,
        block_0x00211ee4,
        block_0x00211eec,
        block_0x00211efc,
        block_0x00211f50,
        block_0x00211f58,
        block_0x00211f64,
        block_0x00211f6c,
        block_0x00211f78,
        block_0x00211f88,
        block_0x00211f90,
        block_0x00211f98,
        block_0x00211fa4,
        block_0x00211fa8,
        block_0x00211fb0,
        block_0x00211fbc,
        block_0x00211fcc,
        block_0x00211fd4,
        block_0x00211fdc,
        block_0x00211fe0,
        block_0x00211fe8,
        block_0x00211ff8,
        block_0x00211ffc,
        block_0x00212000,
        block_0x00212070,
        block_0x00212078,
        block_0x00212098,
        block_0x002120b0,
        block_0x002120b8,
        block_0x002120bc,
        block_0x002120dc,
        block_0x002120e4,
        block_0x002120ec,
        block_0x00212100,
        block_0x00212110,
        block_0x00212118,
        block_0x00212120,
        block_0x00212138,
        block_0x00212154,
        block_0x00212158,
        block_0x00212174,
        block_0x0021217c,
        block_0x0021218c,
        block_0x002121b4,
        block_0x002121d0,
        block_0x0021221c,
        block_0x00212224,
        block_0x0021222c,
        block_0x00212260,
    ];
    const IDX: [u16; 653usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 5u16, 6u16, 0u16,
        0u16, 7u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 0u16, 13u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16,
        0u16, 17u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16,
        0u16, 23u16, 0u16, 0u16, 24u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16, 37u16, 38u16, 0u16,
        39u16, 40u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 0u16, 0u16, 67u16, 0u16, 68u16,
        0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16,
        73u16, 74u16, 0u16, 75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        78u16, 0u16, 79u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 82u16, 83u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 107u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
    ];
    if pc < 2168880u32 || pc > 2171488u32 {
        return None;
    }
    let word_offset = ((pc - 2168880u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00211830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 3u32, 2168884u32);
    emu.ani_no_count(13usize, 13usize, 4294967292u32, 2168888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2168904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211848));
    } else {
        emu.pc = 2168892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021183c));
    }
}
#[inline(always)]
pub fn block_0x0021183c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2168896u32);
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2168900u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2168904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211884));
}
#[inline(always)]
pub fn block_0x00211848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 13usize, 11usize, 2168908u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2168912u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2168920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211858));
    } else {
        emu.pc = 2168916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211854));
    }
}
#[inline(always)]
pub fn block_0x00211854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 14usize, 0u32, 2168920u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168920u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211858));
}
#[inline(always)]
pub fn block_0x00211858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021187c));
    } else {
        emu.pc = 2168924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021185c));
    }
}
#[inline(always)]
pub fn block_0x0021185c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2168928u32);
    emu.sbr_no_count(15usize, 0usize, 13usize, 2168932u32);
    emu.adi_no_count(16usize, 11usize, 0u32, 2168936u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2168936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211868));
}
#[inline(always)]
pub fn block_0x00211868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(17usize, 16usize, 0u32, 2168940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2169108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211914));
    } else {
        emu.pc = 2168944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211870));
    }
}
#[inline(always)]
pub fn block_0x00211870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2168948u32);
    emu.adi_no_count(16usize, 16usize, 1u32, 2168952u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2168936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211868));
    } else {
        emu.pc = 2168956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021187c));
    }
}
#[inline(always)]
pub fn block_0x0021187c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 12usize, 4294967288u32, 2168960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2169052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118dc));
    } else {
        emu.pc = 2168964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211884));
    }
}
#[inline(always)]
pub fn block_0x00211884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2168968u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 11usize, 4u32, 2168972u32);
    let a = 0u32.wrapping_add(2155905024u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2168976u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 256u32, 2168980u32);
    emu.adi_no_count(17usize, 16usize, 1u32, 2168984u32);
    emu.mul_no_count(17usize, 10usize, 17usize, 2168988u32);
    emu.adi_no_count(5usize, 5usize, 128u32, 2168992u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2168992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002118a0));
}
#[inline]
pub fn block_0x002118a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 11usize, 13usize, 2168996u32);
    emu.adr_no_count(7usize, 15usize, 13usize, 2169000u32);
    emu.lw_no_count(6usize, 6usize, 0u32, 2169004u32)?;
    emu.lw_no_count(7usize, 7usize, 0u32, 2169008u32)?;
    emu.xrr_no_count(6usize, 6usize, 17usize, 2169012u32);
    emu.xrr_no_count(7usize, 7usize, 17usize, 2169016u32);
    emu.sbr_no_count(28usize, 16usize, 6usize, 2169020u32);
    emu.orr_no_count(6usize, 28usize, 6usize, 2169024u32);
    emu.sbr_no_count(28usize, 16usize, 7usize, 2169028u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2169032u32);
    emu.anr_no_count(6usize, 6usize, 7usize, 2169036u32);
    emu.anr_no_count(6usize, 6usize, 5usize, 2169040u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a != b {
        emu.pc = 2169052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118dc));
    } else {
        emu.pc = 2169044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118d4));
    }
}
#[inline(always)]
pub fn block_0x002118d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 8u32, 2169048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2168992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118a0));
    } else {
        emu.pc = 2169052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118dc));
    }
}
#[inline(always)]
pub fn block_0x002118dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2169088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211900));
    } else {
        emu.pc = 2169056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118e0));
    }
}
#[inline(always)]
pub fn block_0x002118e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 11usize, 13usize, 2169060u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2169064u32);
    emu.sbr_no_count(12usize, 0usize, 12usize, 2169068u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2169068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002118ec));
}
#[inline(always)]
pub fn block_0x002118ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 14usize, 0u32, 2169072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2169096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211908));
    } else {
        emu.pc = 2169076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118f4));
    }
}
#[inline(always)]
pub fn block_0x002118f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2169080u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2169084u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2169068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002118ec));
    } else {
        emu.pc = 2169088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211900));
    }
}
#[inline(always)]
pub fn block_0x00211900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2169092u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169096u32;
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
pub fn block_0x00211908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 11usize, 2169100u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2169104u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169108u32;
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
pub fn block_0x00211914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 14usize, 2169112u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2169116u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169120u32;
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
pub fn block_0x00211920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2169124u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2169128u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965935u32, 2169132u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2169136u32);
    emu.apc_no_count(1usize, 2169136u32, 4294959104u32, 2169140u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00211938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169148u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169152u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965980u32, 2169156u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2169160u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2169164u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2169168u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2169172u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169176u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2169180u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2169184u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2169188u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2169192u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2169196u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2169200u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2169204u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2169208u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169212u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2169216u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2169220u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2169224u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2169228u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2169232u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2169236u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2169240u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169244u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2169248u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2169252u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2169256u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2169260u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2169264u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2169268u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2169272u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169276u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2169280u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2169284u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2169288u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2169292u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2169296u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2169300u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169304u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2169308u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2169312u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2169316u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2169320u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2169324u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2169328u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2169332u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2169336u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2169340u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2169344u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2169348u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2169352u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2169356u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2169360u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2169364u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2169368u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2169372u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2169376u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2169380u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2169384u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2169416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a48));
    } else {
        emu.pc = 2169388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a2c));
    }
}
#[inline(always)]
pub fn block_0x00211a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2169392u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2169396u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2169420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a4c));
    } else {
        emu.pc = 2169400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a38));
    }
}
#[inline(always)]
pub fn block_0x00211a38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2169404u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2169408u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2169444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a64));
    } else {
        emu.pc = 2169412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a44));
    }
}
#[inline(always)]
pub fn block_0x00211a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2169416u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a90));
}
#[inline(always)]
pub fn block_0x00211a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2169420u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a4c));
}
#[inline(always)]
pub fn block_0x00211a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2169424u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2169428u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2169432u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2169436u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2169440u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2169488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a90));
    } else {
        emu.pc = 2169444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a64));
    }
}
#[inline(always)]
pub fn block_0x00211a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2169448u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2169452u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2169456u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2169460u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1984u32, 2169464u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2169464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211a78));
}
#[inline(always)]
pub fn block_0x00211a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2169468u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2169472u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2169476u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2169488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a90));
    } else {
        emu.pc = 2169480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a88));
    }
}
#[inline(always)]
pub fn block_0x00211a88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2169484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2169464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a78));
    } else {
        emu.pc = 2169488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211a90));
    }
}
#[inline(always)]
pub fn block_0x00211a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2169492u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169496u32;
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
pub fn block_0x00211a98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2169500u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2169504u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2169508u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2169512u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2169516u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2169520u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2169524u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2169528u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2169532u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2169536u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2169540u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2169544u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2169548u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2169552u32)?;
    emu.adi_no_count(26usize, 12usize, 0u32, 2169556u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2169560u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2169564u32)?;
    emu.orr_no_count(12usize, 14usize, 16usize, 2169568u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2172680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172680u32));
    } else {
        emu.pc = 2169572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ae4));
    }
}
#[inline(always)]
pub fn block_0x00211ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2169576u32);
    emu.lw_no_count(17usize, 11usize, 8u32, 2169580u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2169584u32)?;
    emu.orr_no_count(10usize, 17usize, 6usize, 2169588u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2172708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172708u32));
    } else {
        emu.pc = 2169592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211af8));
    }
}
#[inline(always)]
pub fn block_0x00211af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 16u32, 2169596u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2169600u32)?;
    emu.orr_no_count(12usize, 10usize, 5usize, 2169604u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2172736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172736u32));
    } else {
        emu.pc = 2169608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b08));
    }
}
#[inline(always)]
pub fn block_0x00211b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 10usize, 2169612u32);
    emu.sltru_no_count(7usize, 15usize, 14usize, 2169616u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2169620u32);
    emu.adr_no_count(12usize, 5usize, 7usize, 2169624u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2169632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b20));
    } else {
        emu.pc = 2169628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b1c));
    }
}
#[inline(always)]
pub fn block_0x00211b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 16usize, 2169632u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169632u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211b20));
}
#[inline(always)]
pub fn block_0x00211b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2172764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172764u32));
    } else {
        emu.pc = 2169636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b24));
    }
}
#[inline(always)]
pub fn block_0x00211b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2169648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b30));
    } else {
        emu.pc = 2169640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b28));
    }
}
#[inline(always)]
pub fn block_0x00211b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 16usize, 6usize, 2169644u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2169648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211b34));
}
#[inline(always)]
pub fn block_0x00211b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 17usize, 2169652u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211b34));
}
#[inline(always)]
pub fn block_0x00211b34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2172792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172792u32));
    } else {
        emu.pc = 2169656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b38));
    }
}
#[inline(always)]
pub fn block_0x00211b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 16u32, 2169660u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2172820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172820u32));
    } else {
        emu.pc = 2169664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b40));
    }
}
#[inline(always)]
pub fn block_0x00211b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 10usize, 2169668u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2169672u32);
    emu.sri_no_count(12usize, 10usize, 29u32, 2169676u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2172848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172848u32));
    } else {
        emu.pc = 2169680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b50));
    }
}
#[inline]
pub fn block_0x00211b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 15usize, 1u32, 2169684u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169688u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2169692u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2169696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2169700u32;
    emu.update_insn_clock();
    emu.adi_no_count(30usize, 12usize, 1365u32, 2169704u32);
    emu.adi_no_count(29usize, 7usize, 819u32, 2169708u32);
    emu.adi_no_count(28usize, 28usize, 4294967055u32, 2169712u32);
    emu.adi_no_count(7usize, 31usize, 257u32, 2169716u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2169848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211bf8));
    } else {
        emu.pc = 2169720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b78));
    }
}
#[inline(never)]
pub fn block_0x00211b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(12usize, 15usize, 5usize, 2169724u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2169728u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169732u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2169736u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169740u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2169744u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169748u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2169752u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169756u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2169760u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2169764u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2169768u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2169772u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2169776u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2169780u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2169784u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2169788u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2169792u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2169796u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2169800u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2169804u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2169808u32);
    emu.adi_no_count(7usize, 12usize, 32u32, 2169812u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2169816u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2169820u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2169824u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2169828u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2169960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c68));
    } else {
        emu.pc = 2169832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211be8));
    }
}
#[inline(always)]
pub fn block_0x00211be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 10usize, 7usize, 2169836u32);
    emu.srr_no_count(11usize, 5usize, 8usize, 2169840u32);
    emu.orr_no_count(11usize, 10usize, 11usize, 2169844u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2169848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211c6c));
}
#[inline(never)]
pub fn block_0x00211bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 10usize, 1u32, 2169852u32);
    emu.orr_no_count(12usize, 10usize, 12usize, 2169856u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2169860u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169864u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2169868u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169872u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2169876u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169880u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2169884u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2169888u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2169892u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2169896u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2169900u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2169904u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2169908u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2169912u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2169916u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2169920u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2169924u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2169928u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2169932u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2169936u32);
    emu.sri_no_count(7usize, 12usize, 24u32, 2169940u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2169944u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2169948u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2169952u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2169956u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2169832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211be8));
    } else {
        emu.pc = 2169960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c68));
    }
}
#[inline(always)]
pub fn block_0x00211c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(11usize, 15usize, 31usize, 2169964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211c6c));
}
#[inline]
pub fn block_0x00211c6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 9usize, 1055u32, 2169968u32);
    emu.sltru_no_count(10usize, 14usize, 17usize, 2169972u32);
    emu.sbr_no_count(12usize, 16usize, 6usize, 2169976u32);
    emu.sbr_no_count(28usize, 14usize, 17usize, 2169980u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2169984u32);
    emu.sw_no_count(28usize, 2usize, 24u32, 2169988u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2169992u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2169996u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2170036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cb4));
    } else {
        emu.pc = 2170000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211c90));
    }
}
#[inline(always)]
pub fn block_0x00211c90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(17usize, 28usize, 31usize, 2170004u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2170008u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2170012u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2170064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cd0));
    } else {
        emu.pc = 2170016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ca0));
    }
}
#[inline(always)]
pub fn block_0x00211ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 6usize, 7usize, 2170020u32);
    emu.sli_no_count(29usize, 17usize, 1u32, 2170024u32);
    emu.slr_no_count(29usize, 29usize, 8usize, 2170028u32);
    emu.orr_no_count(29usize, 12usize, 29usize, 2170032u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2170036u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170068u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211cd4));
}
#[inline(always)]
pub fn block_0x00211cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 10usize, 7usize, 2170040u32);
    emu.sri_no_count(17usize, 28usize, 1u32, 2170044u32);
    emu.srr_no_count(17usize, 17usize, 8usize, 2170048u32);
    emu.orr_no_count(17usize, 12usize, 17usize, 2170052u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2170056u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2170060u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2170016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ca0));
    } else {
        emu.pc = 2170064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cd0));
    }
}
#[inline(always)]
pub fn block_0x00211cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(29usize, 17usize, 31usize, 2170068u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170068u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211cd4));
}
#[inline(always)]
pub fn block_0x00211cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 17usize, 7usize, 2170072u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2170076u32);
    emu.anr_no_count(12usize, 5usize, 12usize, 2170080u32);
    emu.xrr_no_count(10usize, 12usize, 10usize, 2170084u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2170088u32);
    emu.sw_no_count(29usize, 2usize, 40u32, 2170092u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2170096u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2172644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172644u32));
    } else {
        emu.pc = 2170100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211cf4));
    }
}
#[inline(always)]
pub fn block_0x00211cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 24u32, 2170104u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2170108u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2170112u32)?;
    emu.slr_no_count(10usize, 14usize, 31usize, 2170116u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2170156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d2c));
    } else {
        emu.pc = 2170120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d08));
    }
}
#[inline(always)]
pub fn block_0x00211d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 10usize, 0u32, 2170124u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2170128u32);
    emu.srr_no_count(10usize, 10usize, 31usize, 2170132u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2170184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d48));
    } else {
        emu.pc = 2170136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d18));
    }
}
#[inline(always)]
pub fn block_0x00211d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 28usize, 1u32, 2170140u32);
    emu.slr_no_count(12usize, 12usize, 8usize, 2170144u32);
    emu.srr_no_count(31usize, 29usize, 31usize, 2170148u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2170152u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2170156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d4c));
}
#[inline(always)]
pub fn block_0x00211d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 14usize, 1u32, 2170160u32);
    emu.srr_no_count(12usize, 12usize, 8usize, 2170164u32);
    emu.slr_no_count(28usize, 16usize, 31usize, 2170168u32);
    emu.orr_no_count(28usize, 28usize, 12usize, 2170172u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2170176u32);
    emu.srr_no_count(10usize, 28usize, 31usize, 2170180u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2170136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d18));
    } else {
        emu.pc = 2170184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d48));
    }
}
#[inline(always)]
pub fn block_0x00211d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 10usize, 0u32, 2170188u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211d4c));
}
#[inline(always)]
pub fn block_0x00211d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 5usize, 10usize, 2170192u32);
    emu.xrr_no_count(12usize, 31usize, 14usize, 2170196u32);
    emu.xrr_no_count(14usize, 10usize, 16usize, 2170200u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2170204u32);
    emu.sw_no_count(31usize, 2usize, 40u32, 2170208u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2170212u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2172644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172644u32));
    } else {
        emu.pc = 2170216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211d68));
    }
}
#[inline]
pub fn block_0x00211d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 16u32, 2170220u32)?;
    emu.sbr_no_count(10usize, 30usize, 7usize, 2170224u32);
    emu.adi_no_count(12usize, 0usize, 4294967200u32, 2170228u32);
    emu.adi_no_count(16usize, 0usize, 80u32, 2170232u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2170236u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 10usize, 2170240u32);
    emu.adi_no_count(14usize, 14usize, 4294965651u32, 2170244u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2170248u32);
    emu.sai_no_count(12usize, 12usize, 1040u32, 2170252u32);
    emu.adi_no_count(12usize, 12usize, 1087u32, 2170256u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2170260u32);
    emu.mulh_no_count(12usize, 12usize, 14usize, 2170264u32);
    emu.sri_no_count(14usize, 12usize, 31u32, 2170268u32);
    emu.sai_no_count(12usize, 12usize, 1034u32, 2170272u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2170276u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2172924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172924u32));
    } else {
        emu.pc = 2170280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211da8));
    }
}
#[inline(never)]
pub fn block_0x00211da8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 15usize, 7usize, 2170284u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2170288u32);
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2170292u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966120u32, 2170296u32);
    emu.sbr_no_count(15usize, 0usize, 10usize, 2170300u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2170304u32);
    emu.lw_no_count(14usize, 16usize, 0u32, 2170308u32)?;
    emu.lw_no_count(7usize, 16usize, 4u32, 2170312u32)?;
    emu.anr_no_count(10usize, 5usize, 12usize, 2170316u32);
    emu.lh_no_count(12usize, 16usize, 8u32, 2170320u32)?;
    emu.mulhu_no_count(30usize, 14usize, 10usize, 2170324u32);
    emu.mul_no_count(31usize, 7usize, 10usize, 2170328u32);
    emu.mulhu_no_count(8usize, 7usize, 10usize, 2170332u32);
    emu.mul_no_count(9usize, 14usize, 11usize, 2170336u32);
    emu.mulhu_no_count(18usize, 14usize, 11usize, 2170340u32);
    emu.mul_no_count(10usize, 7usize, 11usize, 2170344u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2170348u32);
    emu.sbr_no_count(15usize, 15usize, 12usize, 2170352u32);
    emu.mulhu_no_count(12usize, 14usize, 6usize, 2170356u32);
    emu.mul_no_count(19usize, 7usize, 6usize, 2170360u32);
    emu.mulhu_no_count(6usize, 7usize, 6usize, 2170364u32);
    emu.mul_no_count(20usize, 14usize, 17usize, 2170368u32);
    emu.mulhu_no_count(21usize, 14usize, 17usize, 2170372u32);
    emu.mul_no_count(5usize, 7usize, 17usize, 2170376u32);
    emu.mulhu_no_count(17usize, 7usize, 17usize, 2170380u32);
    emu.mulhu_no_count(22usize, 14usize, 29usize, 2170384u32);
    emu.mul_no_count(23usize, 7usize, 29usize, 2170388u32);
    emu.mulhu_no_count(29usize, 7usize, 29usize, 2170392u32);
    emu.mul_no_count(24usize, 14usize, 28usize, 2170396u32);
    emu.mulhu_no_count(14usize, 14usize, 28usize, 2170400u32);
    emu.mul_no_count(25usize, 7usize, 28usize, 2170404u32);
    emu.mulhu_no_count(28usize, 7usize, 28usize, 2170408u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2170412u32);
    emu.sltru_no_count(7usize, 30usize, 31usize, 2170416u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2170420u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2170424u32);
    emu.sltru_no_count(31usize, 12usize, 19usize, 2170428u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2170432u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2170436u32);
    emu.sltru_no_count(6usize, 22usize, 23usize, 2170440u32);
    emu.adr_no_count(19usize, 29usize, 6usize, 2170444u32);
    emu.adr_no_count(30usize, 9usize, 30usize, 2170448u32);
    emu.sltru_no_count(6usize, 30usize, 9usize, 2170452u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2170456u32);
    emu.adr_no_count(6usize, 20usize, 12usize, 2170460u32);
    emu.sltru_no_count(12usize, 6usize, 20usize, 2170464u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2170468u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2170472u32);
    emu.sltru_no_count(29usize, 22usize, 24usize, 2170476u32);
    emu.adr_no_count(14usize, 14usize, 29usize, 2170480u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2170484u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2170488u32);
    emu.adr_no_count(9usize, 11usize, 7usize, 2170492u32);
    emu.ani_no_count(7usize, 15usize, 63u32, 2170496u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2170500u32);
    emu.sltru_no_count(8usize, 12usize, 31usize, 2170504u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2170508u32);
    emu.adi_no_count(29usize, 7usize, 4294967264u32, 2170512u32);
    emu.sri_no_count(17usize, 30usize, 31u32, 2170516u32);
    emu.sri_no_count(30usize, 22usize, 31u32, 2170520u32);
    emu.adr_no_count(11usize, 19usize, 14usize, 2170524u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2170528u32);
    emu.adr_no_count(14usize, 5usize, 12usize, 2170532u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2170536u32);
    emu.adr_no_count(31usize, 25usize, 11usize, 2170540u32);
    emu.sltru_no_count(19usize, 18usize, 10usize, 2170544u32);
    emu.adr_no_count(11usize, 17usize, 18usize, 2170548u32);
    emu.sltru_no_count(10usize, 14usize, 5usize, 2170552u32);
    emu.sltru_no_count(24usize, 31usize, 25usize, 2170556u32);
    emu.adr_no_count(5usize, 28usize, 12usize, 2170560u32);
    emu.adr_no_count(18usize, 30usize, 31usize, 2170564u32);
    emu.adr_no_count(9usize, 9usize, 19usize, 2170568u32);
    emu.sltru_no_count(17usize, 11usize, 17usize, 2170572u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2170576u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2170580u32);
    emu.sltiu_no_count(28usize, 12usize, 1u32, 2170584u32);
    emu.adr_no_count(21usize, 17usize, 28usize, 2170588u32);
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2170592u32);
    emu.add_memory_rw_events(78usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2170604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211eec));
    } else {
        emu.pc = 2170596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ee4));
    }
}
#[inline(always)]
pub fn block_0x00211ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(22usize, 21usize, 7usize, 2170600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211efc));
}
#[inline(always)]
pub fn block_0x00211eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 21usize, 1u32, 2170608u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2170612u32);
    emu.srr_no_count(9usize, 12usize, 15usize, 2170616u32);
    emu.orr_no_count(22usize, 9usize, 28usize, 2170620u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2170620u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211efc));
}
#[inline]
pub fn block_0x00211efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 5usize, 24usize, 2170624u32);
    emu.sw_no_count(18usize, 2usize, 12u32, 2170628u32)?;
    emu.sltru_no_count(18usize, 18usize, 30usize, 2170632u32);
    emu.lhu_no_count(9usize, 16usize, 10u32, 2170636u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2170640u32);
    emu.sai_no_count(5usize, 6usize, 1055u32, 2170644u32);
    emu.slti_no_count(10usize, 29usize, 0u32, 2170648u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2170652u32);
    emu.sri_no_count(6usize, 22usize, 4u32, 2170656u32);
    emu.adi_no_count(28usize, 0usize, 625u32, 2170660u32);
    emu.slr_no_count(30usize, 16usize, 7usize, 2170664u32);
    emu.slr_no_count(16usize, 16usize, 15usize, 2170668u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2170672u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2170676u32);
    emu.anr_no_count(15usize, 15usize, 30usize, 2170680u32);
    emu.anr_no_count(16usize, 10usize, 16usize, 2170684u32);
    emu.sltiu_no_count(10usize, 16usize, 1u32, 2170688u32);
    emu.sbr_no_count(19usize, 15usize, 10usize, 2170692u32);
    emu.adi_no_count(20usize, 16usize, 4294967295u32, 2170696u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2170700u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2170732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f6c));
    } else {
        emu.pc = 2170704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f50));
    }
}
#[inline(always)]
pub fn block_0x00211f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 100u32, 2170708u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2170776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f98));
    } else {
        emu.pc = 2170712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f58));
    }
}
#[inline(always)]
pub fn block_0x00211f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 9u32, 2170716u32);
    emu.sltiu_no_count(10usize, 22usize, 10u32, 2170720u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2170844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fdc));
    } else {
        emu.pc = 2170724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f64));
    }
}
#[inline(always)]
pub fn block_0x00211f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1u32, 2170728u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fe0));
}
#[inline(always)]
pub fn block_0x00211f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2170736u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2170740u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2170800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fb0));
    } else {
        emu.pc = 2170744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f78));
    }
}
#[inline(always)]
pub fn block_0x00211f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2170748u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1696u32, 2170752u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2170756u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2170768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f90));
    } else {
        emu.pc = 2170760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f88));
    }
}
#[inline(always)]
pub fn block_0x00211f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2170764u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 1808u32, 2170768u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2170768u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f90));
}
#[inline(always)]
pub fn block_0x00211f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 5u32, 2170772u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212000));
}
#[inline(always)]
pub fn block_0x00211f98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 1000u32, 2170780u32);
    emu.sltiu_no_count(10usize, 22usize, 1000u32, 2170784u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2170792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fa8));
    } else {
        emu.pc = 2170788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fa4));
    }
}
#[inline(always)]
pub fn block_0x00211fa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1000u32, 2170792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fa8));
}
#[inline(always)]
pub fn block_0x00211fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 3u32, 2170796u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212000));
}
#[inline(always)]
pub fn block_0x00211fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2170804u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 256u32, 2170808u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2170856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fe8));
    } else {
        emu.pc = 2170812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fbc));
    }
}
#[inline(always)]
pub fn block_0x00211fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2170816u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1664u32, 2170820u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2170824u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2170836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fd4));
    } else {
        emu.pc = 2170828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fcc));
    }
}
#[inline(always)]
pub fn block_0x00211fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2170832u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 576u32, 2170836u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2170836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fd4));
}
#[inline(always)]
pub fn block_0x00211fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 7u32, 2170840u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212000));
}
#[inline(always)]
pub fn block_0x00211fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 10u32, 2170848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211fe0));
}
#[inline(always)]
pub fn block_0x00211fe0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 1u32, 2170852u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170880u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212000));
}
#[inline(always)]
pub fn block_0x00211fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2170860u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 10usize, 4294965760u32, 2170864u32);
    emu.sltru_no_count(10usize, 22usize, 6usize, 2170868u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2170876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ffc));
    } else {
        emu.pc = 2170872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ff8));
    }
}
#[inline(always)]
pub fn block_0x00211ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 6usize, 0u32, 2170876u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ffc));
}
#[inline(always)]
pub fn block_0x00211ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 9u32, 2170880u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212000));
}
#[inline(never)]
pub fn block_0x00212000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 24usize, 18usize, 2170884u32);
    emu.sw_no_count(18usize, 2usize, 4u32, 2170888u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2170892u32)?;
    emu.anr_no_count(18usize, 19usize, 21usize, 2170896u32);
    emu.anr_no_count(21usize, 20usize, 12usize, 2170900u32);
    emu.sbr_no_count(6usize, 1usize, 9usize, 2170904u32);
    emu.sltru_no_count(28usize, 5usize, 14usize, 2170908u32);
    emu.sbr_no_count(30usize, 5usize, 8usize, 2170912u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2170916u32);
    emu.adi_no_count(24usize, 0usize, 4294967295u32, 2170920u32);
    emu.sai_no_count(14usize, 29usize, 1055u32, 2170924u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2170928u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 0usize, 10u32, 2170932u32);
    emu.adi_no_count(6usize, 6usize, 1u32, 2170936u32);
    emu.sw_no_count(6usize, 2usize, 0u32, 2170940u32)?;
    emu.sbr_no_count(28usize, 30usize, 28usize, 2170944u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2170948u32);
    emu.adi_no_count(6usize, 8usize, 4294966477u32, 2170952u32);
    emu.sltru_no_count(5usize, 11usize, 5usize, 2170956u32);
    emu.adi_no_count(8usize, 11usize, 2u32, 2170960u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2170964u32);
    emu.sltru_no_count(9usize, 8usize, 11usize, 2170968u32);
    emu.anr_no_count(17usize, 8usize, 20usize, 2170972u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2170976u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2170980u32);
    emu.anr_no_count(5usize, 9usize, 19usize, 2170984u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2170988u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2170992u32)?;
    emu.add_memory_rw_events(28usize);
    emu.pc = 2170992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212070));
}
#[inline(always)]
pub fn block_0x00212070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 13usize, 11usize, 2170996u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2172876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172876u32));
    } else {
        emu.pc = 2171000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212078));
    }
}
#[inline(always)]
pub fn block_0x00212078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 26usize, 0u32, 2171004u32);
    emu.divu_no_count(28usize, 22usize, 26usize, 2171008u32);
    emu.mul_no_count(26usize, 28usize, 26usize, 2171012u32);
    emu.adi_no_count(30usize, 28usize, 48u32, 2171016u32);
    emu.sbr_no_count(22usize, 22usize, 26usize, 2171020u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2171024u32);
    emu.slr_no_count(26usize, 22usize, 7usize, 2171028u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120bc));
    } else {
        emu.pc = 2171032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212098));
    }
}
#[inline(always)]
pub fn block_0x00212098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 14usize, 26usize, 2171036u32);
    emu.adr_no_count(27usize, 26usize, 18usize, 2171040u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2171044u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2171048u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2171052u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2171100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120dc));
    } else {
        emu.pc = 2171056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120b0));
    }
}
#[inline(always)]
pub fn block_0x002120b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 26usize, 8usize, 2171060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2171108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120e4));
    } else {
        emu.pc = 2171064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120b8));
    }
}
#[inline(always)]
pub fn block_0x002120b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2171068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212110));
}
#[inline(always)]
pub fn block_0x002120bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(28usize, 22usize, 1u32, 2171072u32);
    emu.srr_no_count(27usize, 28usize, 31usize, 2171076u32);
    emu.anr_no_count(28usize, 14usize, 26usize, 2171080u32);
    emu.adr_no_count(27usize, 27usize, 18usize, 2171084u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2171088u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2171092u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2171096u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2171056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120b0));
    } else {
        emu.pc = 2171100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120dc));
    }
}
#[inline(always)]
pub fn block_0x002120dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 27usize, 9usize, 2171104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2171152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212110));
    } else {
        emu.pc = 2171108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120e4));
    }
}
#[inline(always)]
pub fn block_0x002120e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 1usize, 11usize, 2171112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2171168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212120));
    } else {
        emu.pc = 2171116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120ec));
    }
}
#[inline(always)]
pub fn block_0x002120ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(28usize, 25usize, 6usize, 2171120u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2171124u32);
    emu.sri_no_count(26usize, 28usize, 3u32, 2171128u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2171132u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2170992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212070));
    } else {
        emu.pc = 2171136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212100));
    }
}
#[inline(always)]
pub fn block_0x00212100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2224128u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2171140u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 400u32, 2171144u32);
    emu.apc_no_count(1usize, 2171144u32, 4096u32, 2171148u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171152u32;
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
pub fn block_0x00212110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 25usize, 7usize, 2171156u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212224));
    } else {
        emu.pc = 2171160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212118));
    }
}
#[inline(always)]
pub fn block_0x00212118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2171164u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171168u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171436u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021222c));
}
#[inline(always)]
pub fn block_0x00212120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2171172u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2171176u32);
    emu.adi_no_count(6usize, 0usize, 1u32, 2171180u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2171184u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2171188u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2171192u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212154));
}
#[inline(always)]
pub fn block_0x00212138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 18usize, 5usize, 2171196u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2171200u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2171204u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2171208u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2171212u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2171216u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a != b {
        emu.pc = 2171344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121d0));
    } else {
        emu.pc = 2171220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212154));
    }
}
#[inline(always)]
pub fn block_0x00212154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2172900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172900u32));
    } else {
        emu.pc = 2171224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212158));
    }
}
#[inline(always)]
pub fn block_0x00212158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 14usize, 0u32, 2171228u32);
    emu.adi_no_count(8usize, 6usize, 0u32, 2171232u32);
    emu.mulhu_no_count(14usize, 21usize, 10usize, 2171236u32);
    emu.mul_no_count(6usize, 18usize, 10usize, 2171240u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2171244u32);
    emu.mul_no_count(6usize, 21usize, 10usize, 2171248u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021217c));
    } else {
        emu.pc = 2171252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212174));
    }
}
#[inline(always)]
pub fn block_0x00212174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(30usize, 14usize, 7usize, 2171256u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021218c));
}
#[inline(always)]
pub fn block_0x0021217c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 14usize, 1u32, 2171264u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2171268u32);
    emu.srr_no_count(30usize, 6usize, 7usize, 2171272u32);
    emu.orr_no_count(30usize, 30usize, 28usize, 2171276u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2171276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021218c));
}
#[inline]
pub fn block_0x0021218c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(18usize, 14usize, 19usize, 2171280u32);
    emu.anr_no_count(21usize, 6usize, 20usize, 2171284u32);
    emu.mulhu_no_count(14usize, 17usize, 10usize, 2171288u32);
    emu.mul_no_count(5usize, 5usize, 10usize, 2171292u32);
    emu.mul_no_count(17usize, 17usize, 10usize, 2171296u32);
    emu.adi_no_count(22usize, 30usize, 48u32, 2171300u32);
    emu.adr_no_count(5usize, 14usize, 5usize, 2171304u32);
    emu.adr_no_count(14usize, 23usize, 11usize, 2171308u32);
    emu.sb_no_count(22usize, 14usize, 0u32, 2171312u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2171192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212138));
    } else {
        emu.pc = 2171316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121b4));
    }
}
#[inline(always)]
pub fn block_0x002121b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 21usize, 17usize, 2171320u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2171324u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2171328u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2171332u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2171336u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2171340u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2171220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212154));
    } else {
        emu.pc = 2171344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121d0));
    }
}
#[inline]
pub fn block_0x002121d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 12u32, 2171348u32)?;
    emu.sltru_no_count(10usize, 12usize, 7usize, 2171352u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2171356u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2171360u32)?;
    emu.sbr_no_count(13usize, 13usize, 28usize, 2171364u32);
    emu.sbr_no_count(12usize, 12usize, 7usize, 2171368u32);
    emu.sbr_no_count(13usize, 13usize, 10usize, 2171372u32);
    emu.mulhu_no_count(10usize, 6usize, 12usize, 2171376u32);
    emu.mul_no_count(31usize, 14usize, 12usize, 2171380u32);
    emu.mul_no_count(7usize, 6usize, 12usize, 2171384u32);
    emu.mul_no_count(12usize, 6usize, 13usize, 2171388u32);
    emu.adr_no_count(13usize, 7usize, 6usize, 2171392u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2171396u32);
    emu.sltru_no_count(12usize, 7usize, 6usize, 2171400u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2171404u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2171408u32);
    emu.sbr_no_count(29usize, 31usize, 12usize, 2171412u32);
    emu.sbr_no_count(30usize, 7usize, 6usize, 2171416u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2171596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2171596u32));
    } else {
        emu.pc = 2171420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021221c));
    }
}
#[inline(always)]
pub fn block_0x0021221c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 29usize, 2171424u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2171600u32));
}
#[inline(always)]
pub fn block_0x00212224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 1u32, 2171432u32);
    emu.srr_no_count(15usize, 13usize, 31usize, 2171436u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2171436u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021222c));
}
#[inline]
pub fn block_0x0021222c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2171440u32)?;
    emu.lw_no_count(16usize, 2usize, 8u32, 2171444u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2171448u32)?;
    emu.sltru_no_count(13usize, 12usize, 17usize, 2171452u32);
    emu.lw_no_count(5usize, 2usize, 4u32, 2171456u32)?;
    emu.sbr_no_count(16usize, 16usize, 5usize, 2171460u32);
    emu.sbr_no_count(28usize, 12usize, 17usize, 2171464u32);
    emu.sbr_no_count(17usize, 16usize, 13usize, 2171468u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2171472u32);
    emu.sltiu_no_count(12usize, 28usize, 1u32, 2171476u32);
    emu.sbr_no_count(7usize, 17usize, 12usize, 2171480u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2171484u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2171496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2171496u32));
    } else {
        emu.pc = 2171488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212260));
    }
}
#[inline(always)]
pub fn block_0x00212260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 27usize, 7usize, 2171492u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171496u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171500u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2171500u32));
}
