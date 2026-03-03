pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2188236u32;
pub const PC_MAX: u32 = 2190208u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 111usize] = [
        block_0x002163cc,
        block_0x002163dc,
        block_0x002163e0,
        block_0x002163ec,
        block_0x002163f0,
        block_0x00216404,
        block_0x00216434,
        block_0x00216438,
        block_0x0021643c,
        block_0x00216450,
        block_0x00216458,
        block_0x0021645c,
        block_0x00216460,
        block_0x00216474,
        block_0x00216478,
        block_0x00216490,
        block_0x002164a4,
        block_0x002164a8,
        block_0x002164b8,
        block_0x002164bc,
        block_0x002164c0,
        block_0x002164d4,
        block_0x002164fc,
        block_0x00216504,
        block_0x00216508,
        block_0x00216510,
        block_0x00216518,
        block_0x0021651c,
        block_0x00216528,
        block_0x0021653c,
        block_0x00216564,
        block_0x00216568,
        block_0x0021656c,
        block_0x00216574,
        block_0x0021657c,
        block_0x00216590,
        block_0x002165b8,
        block_0x002165bc,
        block_0x002165c0,
        block_0x002165c8,
        block_0x002165d8,
        block_0x002165e0,
        block_0x002165e4,
        block_0x002165ec,
        block_0x002165fc,
        block_0x00216608,
        block_0x0021660c,
        block_0x00216614,
        block_0x00216630,
        block_0x00216634,
        block_0x0021664c,
        block_0x00216650,
        block_0x00216664,
        block_0x0021666c,
        block_0x0021667c,
        block_0x00216698,
        block_0x002166a4,
        block_0x002166bc,
        block_0x002166c0,
        block_0x002166c4,
        block_0x002166d4,
        block_0x002166d8,
        block_0x002166dc,
        block_0x002166ec,
        block_0x00216704,
        block_0x00216708,
        block_0x00216714,
        block_0x00216718,
        block_0x00216764,
        block_0x00216768,
        block_0x00216774,
        block_0x0021678c,
        block_0x002167a0,
        block_0x002167ac,
        block_0x002167c4,
        block_0x002167cc,
        block_0x002167d0,
        block_0x002167ec,
        block_0x00216804,
        block_0x0021681c,
        block_0x00216838,
        block_0x00216854,
        block_0x00216870,
        block_0x0021688c,
        block_0x002168a8,
        block_0x002168c4,
        block_0x002168d8,
        block_0x002168f0,
        block_0x00216908,
        block_0x00216958,
        block_0x00216970,
        block_0x00216980,
        block_0x00216994,
        block_0x00216998,
        block_0x0021699c,
        block_0x002169a0,
        block_0x002169a8,
        block_0x002169ac,
        block_0x002169b0,
        block_0x002169e0,
        block_0x00216a48,
        block_0x00216aa4,
        block_0x00216b0c,
        block_0x00216b20,
        block_0x00216b30,
        block_0x00216b40,
        block_0x00216b44,
        block_0x00216b54,
        block_0x00216b6c,
        block_0x00216b70,
        block_0x00216b80,
    ];
    const IDX: [u16; 494usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        7u16, 8u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 12u16, 13u16, 0u16,
        0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16,
        0u16, 0u16, 17u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16,
        0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 27u16, 28u16, 0u16, 0u16, 29u16, 0u16,
        0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        31u16, 32u16, 33u16, 0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 39u16, 0u16,
        40u16, 0u16, 0u16, 0u16, 41u16, 0u16, 42u16, 43u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 45u16, 0u16, 0u16, 46u16, 47u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 59u16,
        60u16, 0u16, 0u16, 0u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16,
        0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16,
        94u16, 95u16, 96u16, 0u16, 97u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16,
    ];
    if pc < 2188236u32 || pc > 2190208u32 {
        return None;
    }
    let word_offset = ((pc - 2188236u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002163cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2188240u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2188244u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2188248u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2188256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002163e0));
    } else {
        emu.pc = 2188252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002163dc));
    }
}
#[inline(always)]
pub fn block_0x002163dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2188256u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2188256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002163e0));
}
#[inline(always)]
pub fn block_0x002163e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 40u32, 2188260u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2188264u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2189196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021678c));
    } else {
        emu.pc = 2188268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002163ec));
    }
}
#[inline(always)]
pub fn block_0x002163ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2188368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216450));
    } else {
        emu.pc = 2188272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002163f0));
    }
}
#[inline(always)]
pub fn block_0x002163f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2188276u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2188280u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2188284u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2188288u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2188292u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216404));
}
#[inline]
pub fn block_0x00216404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2188296u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2188300u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2188304u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2188308u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2188312u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2188316u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2188320u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2188324u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2188328u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2188332u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2188336u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2188292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216404));
    } else {
        emu.pc = 2188340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216434));
    }
}
#[inline(always)]
pub fn block_0x00216434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2188368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216450));
    } else {
        emu.pc = 2188344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216438));
    }
}
#[inline(always)]
pub fn block_0x00216438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2189528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168d8));
    } else {
        emu.pc = 2188348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021643c));
    }
}
#[inline(always)]
pub fn block_0x0021643c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2188352u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2188356u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2188360u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2188364u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2188368u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216450));
}
#[inline(always)]
pub fn block_0x00216450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 1336u32, 2188372u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2188380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021645c));
    } else {
        emu.pc = 2188376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216458));
    }
}
#[inline(always)]
pub fn block_0x00216458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2188380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2188380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021645c));
}
#[inline(always)]
pub fn block_0x0021645c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2189196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021678c));
    } else {
        emu.pc = 2188384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216460));
    }
}
#[inline(always)]
pub fn block_0x00216460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 10usize, 2u32, 2188388u32);
    emu.sbr_no_count(10usize, 0usize, 13usize, 2188392u32);
    emu.adi_no_count(12usize, 2usize, 1172u32, 2188396u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2188400u32);
    emu.adr_no_count(13usize, 7usize, 13usize, 2188404u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216474));
}
#[inline(always)]
pub fn block_0x00216474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2188456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164a8));
    } else {
        emu.pc = 2188408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216478));
    }
}
#[inline(always)]
pub fn block_0x00216478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2188412u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2188416u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2188420u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2188424u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2188428u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2188404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216474));
    } else {
        emu.pc = 2188432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216490));
    }
}
#[inline(always)]
pub fn block_0x00216490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2188436u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2188440u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2188444u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2188448u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2188472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164b8));
    } else {
        emu.pc = 2188452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164a4));
    }
}
#[inline(always)]
pub fn block_0x002164a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2188456u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2188768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002165e0));
}
#[inline(always)]
pub fn block_0x002164a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2188460u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2188464u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2188468u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2188768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165e0));
    } else {
        emu.pc = 2188472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164b8));
    }
}
#[inline(always)]
pub fn block_0x002164b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2188768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165e0));
    } else {
        emu.pc = 2188476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164bc));
    }
}
#[inline(always)]
pub fn block_0x002164bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2188572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021651c));
    } else {
        emu.pc = 2188480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164c0));
    }
}
#[inline(always)]
pub fn block_0x002164c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2188484u32);
    emu.sli_no_count(13usize, 21usize, 2u32, 2188488u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2188492u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2188496u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2188500u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002164d4));
}
#[inline]
pub fn block_0x002164d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2188504u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2188508u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2188512u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2188516u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2188520u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2188524u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2188528u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2188532u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2188536u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2188500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164d4));
    } else {
        emu.pc = 2188540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002164fc));
    }
}
#[inline(always)]
pub fn block_0x002164fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2188544u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2188560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216510));
    } else {
        emu.pc = 2188548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216504));
    }
}
#[inline(always)]
pub fn block_0x00216504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2189528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168d8));
    } else {
        emu.pc = 2188552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216508));
    }
}
#[inline(always)]
pub fn block_0x00216508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2188556u32)?;
    emu.adi_no_count(21usize, 21usize, 1u32, 2188560u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2188560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216510));
}
#[inline(always)]
pub fn block_0x00216510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 188u32, 2188564u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2188584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216528));
    } else {
        emu.pc = 2188568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216518));
    }
}
#[inline(always)]
pub fn block_0x00216518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2188572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2188660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216574));
}
#[inline(always)]
pub fn block_0x0021651c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2188576u32)?;
    emu.sw_no_count(21usize, 2usize, 188u32, 2188580u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2188660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216574));
    } else {
        emu.pc = 2188584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216528));
    }
}
#[inline(always)]
pub fn block_0x00216528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2188588u32);
    emu.sli_no_count(13usize, 25usize, 2u32, 2188592u32);
    emu.adi_no_count(10usize, 2usize, 192u32, 2188596u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2188600u32);
    emu.adi_no_count(14usize, 2usize, 192u32, 2188604u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021653c));
}
#[inline]
pub fn block_0x0021653c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2188608u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2188612u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2188616u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2188620u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2188624u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2188628u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2188632u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2188636u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2188640u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2188604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021653c));
    } else {
        emu.pc = 2188644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216564));
    }
}
#[inline(always)]
pub fn block_0x00216564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2188660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216574));
    } else {
        emu.pc = 2188648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216568));
    }
}
#[inline(always)]
pub fn block_0x00216568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2189528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168d8));
    } else {
        emu.pc = 2188652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021656c));
    }
}
#[inline(always)]
pub fn block_0x0021656c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2188656u32)?;
    emu.adi_no_count(25usize, 25usize, 1u32, 2188660u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2188660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216574));
}
#[inline(always)]
pub fn block_0x00216574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 352u32, 2188664u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2188744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c8));
    } else {
        emu.pc = 2188668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021657c));
    }
}
#[inline(always)]
pub fn block_0x0021657c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2188672u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2188676u32);
    emu.adi_no_count(10usize, 2usize, 356u32, 2188680u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2188684u32);
    emu.adi_no_count(14usize, 2usize, 356u32, 2188688u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216590));
}
#[inline]
pub fn block_0x00216590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2188692u32)?;
    emu.mulhu_no_count(16usize, 15usize, 9usize, 2188696u32);
    emu.mul_no_count(15usize, 15usize, 9usize, 2188700u32);
    emu.adr_no_count(12usize, 15usize, 12usize, 2188704u32);
    emu.sw_no_count(12usize, 14usize, 0u32, 2188708u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2188712u32);
    emu.sltru_no_count(12usize, 12usize, 15usize, 2188716u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2188720u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2188724u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2188688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216590));
    } else {
        emu.pc = 2188728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165b8));
    }
}
#[inline(always)]
pub fn block_0x002165b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2188744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c8));
    } else {
        emu.pc = 2188732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165bc));
    }
}
#[inline(always)]
pub fn block_0x002165bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2189528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168d8));
    } else {
        emu.pc = 2188736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165c0));
    }
}
#[inline(always)]
pub fn block_0x002165c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2188740u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2188744u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2188744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002165c8));
}
#[inline(always)]
pub fn block_0x002165c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2188748u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2188752u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2188756u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2187428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2187428u32));
    } else {
        emu.pc = 2188760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165d8));
    }
}
#[inline(always)]
pub fn block_0x002165d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2188764u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2188768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2187428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2187428u32));
}
#[inline(always)]
pub fn block_0x002165e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2188996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166c4));
    } else {
        emu.pc = 2188772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165e4));
    }
}
#[inline(always)]
pub fn block_0x002165e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 8u32, 2188776u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(24usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2188880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216650));
    } else {
        emu.pc = 2188780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002165ec));
    }
}
#[inline(always)]
pub fn block_0x002165ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2188784u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2188788u32);
    emu.apc_no_count(1usize, 2188788u32, 4294942720u32, 2188792u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002165fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 188u32, 2188800u32)?;
    emu.lw_no_count(10usize, 2usize, 680u32, 2188804u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2188812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021660c));
    } else {
        emu.pc = 2188808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216608));
    }
}
#[inline(always)]
pub fn block_0x00216608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2188812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2188812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021660c));
}
#[inline(always)]
pub fn block_0x0021660c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 41u32, 2188816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2189196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021678c));
    } else {
        emu.pc = 2188820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216614));
    }
}
#[inline(always)]
pub fn block_0x00216614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 2u32, 2188824u32);
    emu.adi_no_count(12usize, 2usize, 520u32, 2188828u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2188832u32);
    emu.sbr_no_count(10usize, 0usize, 11usize, 2188836u32);
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2188840u32);
    emu.adr_no_count(11usize, 12usize, 14usize, 2188844u32);
    emu.adr_no_count(12usize, 13usize, 14usize, 2188848u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2188848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216630));
}
#[inline(always)]
pub fn block_0x00216630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2189156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216764));
    } else {
        emu.pc = 2188852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216634));
    }
}
#[inline(always)]
pub fn block_0x00216634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2188856u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2188860u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2188864u32);
    emu.adi_no_count(11usize, 11usize, 4294967292u32, 2188868u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2188872u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2188848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216630));
    } else {
        emu.pc = 2188876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021664c));
    }
}
#[inline(always)]
pub fn block_0x0021664c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2189160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216768));
    } else {
        emu.pc = 2188880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216650));
    }
}
#[inline(always)]
pub fn block_0x00216650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2188884u32);
    emu.lw_no_count(21usize, 2usize, 20u32, 2188888u32)?;
    emu.adr_no_count(8usize, 21usize, 23usize, 2188892u32);
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2188896u32);
    emu.adi_no_count(12usize, 0usize, 57u32, 2188900u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2188900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216664));
}
#[inline(always)]
pub fn block_0x00216664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 20usize, 11usize, 2188904u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2189016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166d8));
    } else {
        emu.pc = 2188908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021666c));
    }
}
#[inline(always)]
pub fn block_0x0021666c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 19usize, 11usize, 2188912u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2188916u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2188920u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2188900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216664));
    } else {
        emu.pc = 2188924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021667c));
    }
}
#[inline(always)]
pub fn block_0x0021667c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 20usize, 11usize, 2188928u32);
    emu.adr_no_count(12usize, 10usize, 21usize, 2188932u32);
    emu.lbu_no_count(13usize, 12usize, 1u32, 2188936u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2188940u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2188944u32);
    emu.sb_no_count(13usize, 12usize, 1u32, 2188948u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2189508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002168c4));
    } else {
        emu.pc = 2188952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216698));
    }
}
#[inline(always)]
pub fn block_0x00216698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2188956u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2188960u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2189076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216714));
    } else {
        emu.pc = 2188964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166a4));
    }
}
#[inline(always)]
pub fn block_0x002166a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(12usize, 11usize, 4294967295u32, 2188968u32);
    emu.adr_no_count(10usize, 19usize, 11usize, 2188972u32);
    emu.adi_no_count(10usize, 10usize, 2u32, 2188976u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2188980u32);
    emu.apc_no_count(1usize, 2188980u32, 4294897664u32, 2188984u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2188988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002166bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2189080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216718));
    } else {
        emu.pc = 2188992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166c0));
    }
}
#[inline(always)]
pub fn block_0x002166c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2188996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216774));
}
#[inline(always)]
pub fn block_0x002166c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2189000u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2189004u32)?;
    emu.lw_no_count(22usize, 2usize, 8u32, 2189008u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2189080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216718));
    } else {
        emu.pc = 2189012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166d4));
    }
}
#[inline(always)]
pub fn block_0x002166d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2189016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216774));
}
#[inline(always)]
pub fn block_0x002166d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2189216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002167a0));
    } else {
        emu.pc = 2189020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166dc));
    }
}
#[inline(always)]
pub fn block_0x002166dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2189024u32);
    emu.sb_no_count(10usize, 21usize, 0u32, 2189028u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2189032u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2189252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002167c4));
    } else {
        emu.pc = 2189036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002166ec));
    }
}
#[inline(always)]
pub fn block_0x002166ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 1u32, 2189040u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2189044u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2189048u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2189052u32);
    emu.apc_no_count(1usize, 2189052u32, 4294897664u32, 2189056u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2189228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002167ac));
    } else {
        emu.pc = 2189064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216708));
    }
}
#[inline(always)]
pub fn block_0x00216708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2189068u32);
    emu.adi_no_count(23usize, 20usize, 2u32, 2189072u32);
    emu.adi_no_count(22usize, 22usize, 1u32, 2189076u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2189076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216714));
}
#[inline(always)]
pub fn block_0x00216714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2189172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216774));
    } else {
        emu.pc = 2189080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216718));
    }
}
#[inline]
pub fn block_0x00216718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2189084u32)?;
    emu.sw_no_count(21usize, 10usize, 0u32, 2189088u32)?;
    emu.sw_no_count(23usize, 10usize, 4u32, 2189092u32)?;
    emu.sh_no_count(22usize, 10usize, 8u32, 2189096u32)?;
    emu.lw_no_count(1usize, 2usize, 1388u32, 2189100u32)?;
    emu.lw_no_count(8usize, 2usize, 1384u32, 2189104u32)?;
    emu.lw_no_count(9usize, 2usize, 1380u32, 2189108u32)?;
    emu.lw_no_count(18usize, 2usize, 1376u32, 2189112u32)?;
    emu.lw_no_count(19usize, 2usize, 1372u32, 2189116u32)?;
    emu.lw_no_count(20usize, 2usize, 1368u32, 2189120u32)?;
    emu.lw_no_count(21usize, 2usize, 1364u32, 2189124u32)?;
    emu.lw_no_count(22usize, 2usize, 1360u32, 2189128u32)?;
    emu.lw_no_count(23usize, 2usize, 1356u32, 2189132u32)?;
    emu.lw_no_count(24usize, 2usize, 1352u32, 2189136u32)?;
    emu.lw_no_count(25usize, 2usize, 1348u32, 2189140u32)?;
    emu.lw_no_count(26usize, 2usize, 1344u32, 2189144u32)?;
    emu.lw_no_count(27usize, 2usize, 1340u32, 2189148u32)?;
    emu.adi_no_count(2usize, 2usize, 1392u32, 2189152u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189156u32;
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
pub fn block_0x00216764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2188880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216650));
    } else {
        emu.pc = 2189160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216768));
    }
}
#[inline(always)]
pub fn block_0x00216768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2189164u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2189168u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2189080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216718));
    } else {
        emu.pc = 2189172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216774));
    }
}
#[inline(always)]
pub fn block_0x00216774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189176u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966552u32, 2189180u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2189184u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2189188u32);
    emu.apc_no_count(1usize, 2189188u32, 4096u32, 2189192u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021678c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189200u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2189204u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2189208u32);
    emu.apc_no_count(1usize, 2189208u32, 4096u32, 2189212u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189216u32;
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
pub fn block_0x002167a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2189220u32);
    emu.lw_no_count(18usize, 2usize, 16u32, 2189224u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2189064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216708));
    } else {
        emu.pc = 2189228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002167ac));
    }
}
#[inline(always)]
pub fn block_0x002167ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966536u32, 2189236u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2189240u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2189244u32);
    emu.apc_no_count(1usize, 2189244u32, 4294938624u32, 2189248u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189252u32;
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
pub fn block_0x002167c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2189256u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2189064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216708));
    } else {
        emu.pc = 2189260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002167cc));
    }
}
#[inline(always)]
pub fn block_0x002167cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2189264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002167ac));
}
#[inline(always)]
pub fn block_0x002167d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189268u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1516u32, 2189272u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189276u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2189280u32);
    emu.adi_no_count(11usize, 0usize, 26u32, 2189284u32);
    emu.apc_no_count(1usize, 2189284u32, 4294938624u32, 2189288u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189292u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002167ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189296u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2189300u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2189304u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2189308u32);
    emu.apc_no_count(1usize, 2189308u32, 4096u32, 2189312u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189320u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2189324u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2189328u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2189332u32);
    emu.apc_no_count(1usize, 2189332u32, 4096u32, 2189336u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966780u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021681c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189344u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966320u32, 2189348u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189352u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966348u32, 2189356u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2189360u32);
    emu.apc_no_count(1usize, 2189360u32, 4294938624u32, 2189364u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189372u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2189376u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189380u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966396u32, 2189384u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2189388u32);
    emu.apc_no_count(1usize, 2189388u32, 4294938624u32, 2189392u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189396u32;
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
#[inline(always)]
pub fn block_0x00216854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189400u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966412u32, 2189404u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189408u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966440u32, 2189412u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2189416u32);
    emu.apc_no_count(1usize, 2189416u32, 4294938624u32, 2189420u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189428u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966640u32, 2189432u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189436u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966696u32, 2189440u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2189444u32);
    emu.apc_no_count(1usize, 2189444u32, 4294938624u32, 2189448u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021688c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189456u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966568u32, 2189460u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189464u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966624u32, 2189468u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2189472u32);
    emu.apc_no_count(1usize, 2189472u32, 4294938624u32, 2189476u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002168a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2189484u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966456u32, 2189488u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189492u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966504u32, 2189496u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2189500u32);
    emu.apc_no_count(1usize, 2189500u32, 4294938624u32, 2189504u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002168c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189512u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1648u32, 2189516u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2189520u32);
    emu.apc_no_count(1usize, 2189520u32, 4096u32, 2189524u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002168d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189532u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1500u32, 2189536u32);
    emu.adi_no_count(10usize, 0usize, 40u32, 2189540u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2189544u32);
    emu.apc_no_count(1usize, 2189544u32, 4294938624u32, 2189548u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189552u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002168f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2189556u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966520u32, 2189560u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2189564u32);
    emu.adi_no_count(11usize, 24usize, 0u32, 2189568u32);
    emu.apc_no_count(1usize, 2189568u32, 4294938624u32, 2189572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2189576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(656u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00216908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966416u32, 2189580u32);
    emu.sw_no_count(1usize, 2usize, 876u32, 2189584u32)?;
    emu.sw_no_count(8usize, 2usize, 872u32, 2189588u32)?;
    emu.sw_no_count(9usize, 2usize, 868u32, 2189592u32)?;
    emu.sw_no_count(18usize, 2usize, 864u32, 2189596u32)?;
    emu.sw_no_count(19usize, 2usize, 860u32, 2189600u32)?;
    emu.sw_no_count(20usize, 2usize, 856u32, 2189604u32)?;
    emu.sw_no_count(21usize, 2usize, 852u32, 2189608u32)?;
    emu.sw_no_count(22usize, 2usize, 848u32, 2189612u32)?;
    emu.sw_no_count(23usize, 2usize, 844u32, 2189616u32)?;
    emu.sw_no_count(24usize, 2usize, 840u32, 2189620u32)?;
    emu.sw_no_count(25usize, 2usize, 836u32, 2189624u32)?;
    emu.sw_no_count(26usize, 2usize, 832u32, 2189628u32)?;
    emu.sw_no_count(27usize, 2usize, 828u32, 2189632u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2189636u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2189640u32);
    emu.lw_no_count(5usize, 11usize, 0u32, 2189644u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2189648u32)?;
    emu.orr_no_count(14usize, 5usize, 13usize, 2189652u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2192548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2192548u32));
    } else {
        emu.pc = 2189656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216958));
    }
}
#[inline(always)]
pub fn block_0x00216958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2189660u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2189664u32)?;
    emu.lw_no_count(10usize, 11usize, 8u32, 2189668u32)?;
    emu.lw_no_count(14usize, 11usize, 12u32, 2189672u32)?;
    emu.orr_no_count(15usize, 10usize, 14usize, 2189676u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2192576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2192576u32));
    } else {
        emu.pc = 2189680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216970));
    }
}
#[inline(always)]
pub fn block_0x00216970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 16u32, 2189684u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2189688u32)?;
    emu.orr_no_count(17usize, 15usize, 16usize, 2189692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2192604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2192604u32));
    } else {
        emu.pc = 2189696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216980));
    }
}
#[inline(always)]
pub fn block_0x00216980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 5usize, 15usize, 2189700u32);
    emu.sltru_no_count(15usize, 15usize, 5usize, 2189704u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2189708u32);
    emu.adr_no_count(16usize, 16usize, 15usize, 2189712u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2189720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216998));
    } else {
        emu.pc = 2189716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216994));
    }
}
#[inline(always)]
pub fn block_0x00216994(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2189720u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2189720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216998));
}
#[inline(always)]
pub fn block_0x00216998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2192632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2192632u32));
    } else {
        emu.pc = 2189724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021699c));
    }
}
#[inline(always)]
pub fn block_0x0021699c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2189736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002169a8));
    } else {
        emu.pc = 2189728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002169a0));
    }
}
#[inline(always)]
pub fn block_0x002169a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 13usize, 14usize, 2189732u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2189736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002169ac));
}
#[inline(always)]
pub fn block_0x002169a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 10usize, 2189740u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2189740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002169ac));
}
#[inline(always)]
pub fn block_0x002169ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2192660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2192660u32));
    } else {
        emu.pc = 2189744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002169b0));
    }
}
#[inline]
pub fn block_0x002169b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(20usize, 11usize, 24u32, 2189748u32)?;
    emu.sltiu_no_count(10usize, 5usize, 1u32, 2189752u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2189756u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2189760u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2189764u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 13usize, 10usize, 2189768u32);
    emu.adi_no_count(15usize, 11usize, 1365u32, 2189772u32);
    emu.adi_no_count(14usize, 14usize, 819u32, 2189776u32);
    emu.adi_no_count(10usize, 17usize, 4294967055u32, 2189780u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2189784u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 257u32, 2189788u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2189896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216a48));
    } else {
        emu.pc = 2189792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002169e0));
    }
}
#[inline(never)]
pub fn block_0x002169e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 5usize, 4294967295u32, 2189796u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2189800u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189804u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2189808u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189812u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2189816u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189820u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2189824u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189828u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2189832u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189836u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2189840u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2189844u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2189848u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2189852u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2189856u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2189860u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2189864u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2189868u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2189872u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2189876u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2189880u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2189884u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2189888u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2189892u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2189896u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2189988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216aa4));
}
#[inline]
pub fn block_0x00216a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 16usize, 1u32, 2189900u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189904u32);
    emu.sri_no_count(17usize, 16usize, 2u32, 2189908u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189912u32);
    emu.sri_no_count(17usize, 16usize, 4u32, 2189916u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189920u32);
    emu.sri_no_count(17usize, 16usize, 8u32, 2189924u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189928u32);
    emu.sri_no_count(17usize, 16usize, 16u32, 2189932u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2189936u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2189940u32);
    emu.sri_no_count(17usize, 16usize, 1u32, 2189944u32);
    emu.anr_no_count(15usize, 17usize, 15usize, 2189948u32);
    emu.sbr_no_count(15usize, 16usize, 15usize, 2189952u32);
    emu.anr_no_count(16usize, 15usize, 14usize, 2189956u32);
    emu.sri_no_count(15usize, 15usize, 2u32, 2189960u32);
    emu.anr_no_count(14usize, 15usize, 14usize, 2189964u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2189968u32);
    emu.sri_no_count(15usize, 14usize, 4u32, 2189972u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2189976u32);
    emu.anr_no_count(10usize, 14usize, 10usize, 2189980u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2189984u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2189988u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2189988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216aa4));
}
#[inline(never)]
pub fn block_0x00216aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2189992u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2189996u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2190000u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2190004u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2190008u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2190012u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2190016u32);
    emu.sbr_no_count(11usize, 11usize, 14usize, 2190020u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2190024u32);
    emu.anr_no_count(13usize, 14usize, 13usize, 2190028u32);
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2190032u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 128u32, 2190036u32);
    emu.sw_no_count(11usize, 2usize, 168u32, 2190040u32)?;
    emu.adr_no_count(14usize, 10usize, 14usize, 2190044u32);
    emu.sw_no_count(5usize, 2usize, 8u32, 2190048u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2190052u32)?;
    emu.sltru_no_count(10usize, 14usize, 10usize, 2190056u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2190060u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2190064u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2190068u32);
    emu.sai_no_count(22usize, 10usize, 1040u32, 2190072u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2190076u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2190080u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2190084u32);
    emu.apc_no_count(1usize, 2190084u32, 4294897664u32, 2190088u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965920u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 176u32, 2190096u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2190100u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2190104u32);
    emu.apc_no_count(1usize, 2190104u32, 4294897664u32, 2190108u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2190116u32);
    emu.sw_no_count(10usize, 2usize, 332u32, 2190120u32)?;
    emu.sw_no_count(10usize, 2usize, 172u32, 2190124u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b54));
    } else {
        emu.pc = 2190128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b30));
    }
}
#[inline(always)]
pub fn block_0x00216b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2190132u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2190136u32);
    emu.apc_no_count(1usize, 2190136u32, 4294938624u32, 2190140u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2190192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b70));
    } else {
        emu.pc = 2190148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b44));
    }
}
#[inline(always)]
pub fn block_0x00216b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2190152u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2190156u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2190160u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2190164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2190208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b80));
}
#[inline(always)]
pub fn block_0x00216b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 20usize, 2190168u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2190172u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2190176u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2190180u32);
    emu.apc_no_count(1usize, 2190180u32, 4294938624u32, 2190184u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00216b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2190148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b44));
    } else {
        emu.pc = 2190192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00216b70));
    }
}
#[inline(always)]
pub fn block_0x00216b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 22usize, 2190196u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2190200u32);
    emu.sri_no_count(11usize, 10usize, 16u32, 2190204u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2190208u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2190208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00216b80));
}
#[inline(always)]
pub fn block_0x00216b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2190208u32, 4294963200u32, 2190212u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2190216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
