pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2228484u32;
pub const PC_MAX: u32 = 2229936u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00220104,
        block_0x00220108,
        block_0x00220124,
        block_0x00220130,
        block_0x00220134,
        block_0x00220140,
        block_0x00220144,
        block_0x0022014c,
        block_0x00220158,
        block_0x00220164,
        block_0x00220168,
        block_0x0022016c,
        block_0x00220188,
        block_0x00220194,
        block_0x00220198,
        block_0x002201a4,
        block_0x002201a8,
        block_0x002201b0,
        block_0x002201bc,
        block_0x002201c8,
        block_0x002201cc,
        block_0x002201d4,
        block_0x002201d8,
        block_0x002201e8,
        block_0x002201fc,
        block_0x00220204,
        block_0x00220208,
        block_0x00220210,
        block_0x0022022c,
        block_0x00220234,
        block_0x0022023c,
        block_0x00220248,
        block_0x00220260,
        block_0x00220268,
        block_0x0022026c,
        block_0x00220284,
        block_0x0022028c,
        block_0x00220290,
        block_0x002202a0,
        block_0x002202a4,
        block_0x002202b0,
        block_0x002202b4,
        block_0x002202c8,
        block_0x002202d0,
        block_0x002202d4,
        block_0x002202dc,
        block_0x002202e4,
        block_0x002202f4,
        block_0x00220308,
        block_0x00220310,
        block_0x00220314,
        block_0x0022031c,
        block_0x00220338,
        block_0x00220340,
        block_0x00220348,
        block_0x00220354,
        block_0x0022036c,
        block_0x00220374,
        block_0x00220378,
        block_0x00220390,
        block_0x00220398,
        block_0x0022039c,
        block_0x002203ac,
        block_0x002203b0,
        block_0x002203bc,
        block_0x002203c0,
        block_0x002203d4,
        block_0x002203dc,
        block_0x002203e0,
        block_0x002203e8,
        block_0x00220404,
        block_0x0022040c,
        block_0x00220414,
        block_0x00220424,
        block_0x00220438,
        block_0x00220440,
        block_0x00220458,
        block_0x00220460,
        block_0x0022047c,
        block_0x00220484,
        block_0x0022048c,
        block_0x002204a0,
        block_0x002204a8,
        block_0x002204ac,
        block_0x002204b8,
        block_0x002204e4,
        block_0x002204ec,
        block_0x00220500,
        block_0x00220508,
        block_0x00220510,
        block_0x00220520,
        block_0x00220524,
        block_0x00220534,
        block_0x00220538,
        block_0x00220540,
        block_0x00220544,
        block_0x00220580,
        block_0x002205a4,
        block_0x002205c0,
        block_0x002205dc,
        block_0x002205f8,
        block_0x00220614,
        block_0x00220630,
        block_0x0022064c,
        block_0x00220668,
        block_0x00220680,
        block_0x00220698,
        block_0x002206b0,
    ];
    const IDX: [u16; 364usize] = [
        1u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 5u16,
        0u16, 0u16, 6u16, 7u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 11u16,
        12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16, 15u16, 0u16,
        0u16, 16u16, 17u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 20u16, 21u16,
        0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16,
        26u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16,
        30u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16,
        34u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 38u16, 0u16,
        0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 41u16, 42u16, 0u16, 0u16, 0u16, 0u16,
        43u16, 0u16, 44u16, 45u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 50u16, 51u16, 0u16, 52u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 55u16, 0u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 65u16, 66u16,
        0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 68u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 71u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 74u16, 0u16,
        0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 80u16, 0u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 84u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 87u16, 0u16, 0u16, 0u16,
        0u16, 88u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16,
        0u16, 0u16, 93u16, 94u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
    ];
    if pc < 2228484u32 || pc > 2229936u32 {
        return None;
    }
    let word_offset = ((pc - 2228484u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00220104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 26usize, 28usize, 2228488u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2228488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220108));
}
#[inline(always)]
pub fn block_0x00220108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(6usize, 14usize, 10usize, 2228492u32);
    emu.sltiu_no_count(29usize, 13usize, 1u32, 2228496u32);
    emu.sltru_no_count(10usize, 8usize, 26usize, 2228500u32);
    emu.sbr_no_count(12usize, 9usize, 27usize, 2228504u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2228508u32);
    emu.sbr_no_count(5usize, 8usize, 26usize, 2228512u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220134));
    } else {
        emu.pc = 2228516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220124));
    }
}
#[inline(always)]
pub fn block_0x00220124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 10usize, 15usize, 2228520u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2228524u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2228544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220140));
    } else {
        emu.pc = 2228528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220130));
    }
}
#[inline(always)]
pub fn block_0x00220130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228532u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022014c));
}
#[inline(always)]
pub fn block_0x00220134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 5usize, 6usize, 2228536u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2228540u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2228556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022014c));
    } else {
        emu.pc = 2228544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220140));
    }
}
#[inline(always)]
pub fn block_0x00220140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201cc));
    } else {
        emu.pc = 2228548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220144));
    }
}
#[inline(always)]
pub fn block_0x00220144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 15usize, 2228552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201e8));
    } else {
        emu.pc = 2228556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022014c));
    }
}
#[inline(always)]
pub fn block_0x0022014c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 26usize, 0u32, 2228560u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2228564u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2228900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a4));
    } else {
        emu.pc = 2228568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220158));
    }
}
#[inline(always)]
pub fn block_0x00220158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 13usize, 2228572u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2228576u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202b0));
    } else {
        emu.pc = 2228580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220164));
    }
}
#[inline(always)]
pub fn block_0x00220164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228584u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220414));
}
#[inline(always)]
pub fn block_0x00220168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 30usize, 2228588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2228588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022016c));
}
#[inline(always)]
pub fn block_0x0022016c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 7usize, 2228592u32);
    emu.adr_no_count(7usize, 31usize, 14usize, 2228596u32);
    emu.sltru_no_count(12usize, 17usize, 21usize, 2228600u32);
    emu.sbr_no_count(14usize, 5usize, 18usize, 2228604u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2228608u32);
    emu.sbr_no_count(6usize, 17usize, 21usize, 2228612u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2228632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220198));
    } else {
        emu.pc = 2228616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220188));
    }
}
#[inline(always)]
pub fn block_0x00220188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 14usize, 15usize, 2228620u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2228624u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201a4));
    } else {
        emu.pc = 2228628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220194));
    }
}
#[inline(always)]
pub fn block_0x00220194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002201b0));
}
#[inline(always)]
pub fn block_0x00220198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 6usize, 16usize, 2228636u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2228640u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201b0));
    } else {
        emu.pc = 2228644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201a4));
    }
}
#[inline(always)]
pub fn block_0x002201a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2228948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202d4));
    } else {
        emu.pc = 2228648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201a8));
    }
}
#[inline(always)]
pub fn block_0x002201a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2228652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202dc));
    } else {
        emu.pc = 2228656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201b0));
    }
}
#[inline(always)]
pub fn block_0x002201b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 21usize, 0u32, 2228660u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2228664u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2229168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203b0));
    } else {
        emu.pc = 2228668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201bc));
    }
}
#[inline(always)]
pub fn block_0x002201bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 13usize, 2228672u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2228676u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203bc));
    } else {
        emu.pc = 2228680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201c8));
    }
}
#[inline(always)]
pub fn block_0x002201c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022048c));
}
#[inline(always)]
pub fn block_0x002201cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 6usize, 2228688u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201e8));
    } else {
        emu.pc = 2228692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201d4));
    }
}
#[inline(always)]
pub fn block_0x002201d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022014c));
}
#[inline(always)]
pub fn block_0x002201d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2228700u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2228704u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2228708u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a0));
    } else {
        emu.pc = 2228712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201e8));
    }
}
#[inline(always)]
pub fn block_0x002201e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 26usize, 6usize, 2228716u32);
    emu.sltru_no_count(5usize, 16usize, 26usize, 2228720u32);
    emu.adr_no_count(10usize, 27usize, 15usize, 2228724u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2228728u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2228744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220208));
    } else {
        emu.pc = 2228732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201fc));
    }
}
#[inline(always)]
pub fn block_0x002201fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2228736u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220210));
    } else {
        emu.pc = 2228740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220204));
    }
}
#[inline(always)]
pub fn block_0x00220204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228796u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022023c));
}
#[inline(always)]
pub fn block_0x00220208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2228748u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022023c));
    } else {
        emu.pc = 2228752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220210));
    }
}
#[inline(always)]
pub fn block_0x00220210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 28usize, 26usize, 2228756u32);
    emu.sbr_no_count(12usize, 7usize, 27usize, 2228760u32);
    emu.sltru_no_count(14usize, 16usize, 28usize, 2228764u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2228768u32);
    emu.sbr_no_count(12usize, 5usize, 7usize, 2228772u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2228776u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220234));
    } else {
        emu.pc = 2228780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022022c));
    }
}
#[inline(always)]
pub fn block_0x0022022c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 28usize, 26usize, 2228784u32);
    emu.sbr_no_count(12usize, 16usize, 28usize, 2228788u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2228788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220234));
}
#[inline(always)]
pub fn block_0x00220234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2228792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220510));
    } else {
        emu.pc = 2228796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022023c));
    }
}
#[inline(always)]
pub fn block_0x0022023c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2228800u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2228804u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2228844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022026c));
    } else {
        emu.pc = 2228808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220248));
    }
}
#[inline(always)]
pub fn block_0x00220248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2228812u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2228816u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2228820u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2228824u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2228828u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2228868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220284));
    } else {
        emu.pc = 2228832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220260));
    }
}
#[inline(always)]
pub fn block_0x00220260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 6usize, 2228836u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022028c));
    } else {
        emu.pc = 2228840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220268));
    }
}
#[inline(always)]
pub fn block_0x00220268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228844u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002202a0));
}
#[inline(always)]
pub fn block_0x0022026c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2228848u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2228852u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2228856u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2228860u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2228864u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2228832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220260));
    } else {
        emu.pc = 2228868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220284));
    }
}
#[inline(always)]
pub fn block_0x00220284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 29usize, 15usize, 2228872u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a0));
    } else {
        emu.pc = 2228876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022028c));
    }
}
#[inline(always)]
pub fn block_0x0022028c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2228696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201d8));
    } else {
        emu.pc = 2228880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220290));
    }
}
#[inline(always)]
pub fn block_0x00220290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 31usize, 6usize, 2228884u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2228888u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2228892u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201e8));
    } else {
        emu.pc = 2228896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a0));
    }
}
#[inline(always)]
pub fn block_0x002202a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2228568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220158));
    } else {
        emu.pc = 2228900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a4));
    }
}
#[inline(always)]
pub fn block_0x002202a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 17usize, 2228904u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2228908u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220414));
    } else {
        emu.pc = 2228912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202b0));
    }
}
#[inline(always)]
pub fn block_0x002202b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2229268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220414));
    } else {
        emu.pc = 2228916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202b4));
    }
}
#[inline(always)]
pub fn block_0x002202b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 16usize, 6usize, 2228920u32);
    emu.sltru_no_count(10usize, 6usize, 16usize, 2228924u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2228928u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2228932u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203e0));
    } else {
        emu.pc = 2228936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202c8));
    }
}
#[inline(always)]
pub fn block_0x002202c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 17usize, 2228940u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2229224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203e8));
    } else {
        emu.pc = 2228944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202d0));
    }
}
#[inline(always)]
pub fn block_0x002202d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2228948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220508));
}
#[inline(always)]
pub fn block_0x002202d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 16usize, 2228952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2228656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201b0));
    } else {
        emu.pc = 2228956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202dc));
    }
}
#[inline(always)]
pub fn block_0x002202dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 23usize, 11usize, 2228960u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2228964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002202f4));
}
#[inline(always)]
pub fn block_0x002202e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 15usize, 2228968u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2228972u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2228976u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203ac));
    } else {
        emu.pc = 2228980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202f4));
    }
}
#[inline(always)]
pub fn block_0x002202f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 21usize, 16usize, 2228984u32);
    emu.sltru_no_count(7usize, 6usize, 21usize, 2228988u32);
    emu.adr_no_count(10usize, 18usize, 15usize, 2228992u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2228996u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2229012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220314));
    } else {
        emu.pc = 2229000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220308));
    }
}
#[inline(always)]
pub fn block_0x00220308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2229004u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022031c));
    } else {
        emu.pc = 2229008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220310));
    }
}
#[inline(always)]
pub fn block_0x00220310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220348));
}
#[inline(always)]
pub fn block_0x00220314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2229016u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220348));
    } else {
        emu.pc = 2229020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022031c));
    }
}
#[inline(always)]
pub fn block_0x0022031c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 30usize, 21usize, 2229024u32);
    emu.sbr_no_count(12usize, 29usize, 18usize, 2229028u32);
    emu.sltru_no_count(31usize, 6usize, 30usize, 2229032u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2229036u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2229040u32);
    emu.sbr_no_count(12usize, 12usize, 31usize, 2229044u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220340));
    } else {
        emu.pc = 2229048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220338));
    }
}
#[inline(always)]
pub fn block_0x00220338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 30usize, 21usize, 2229052u32);
    emu.sbr_no_count(12usize, 6usize, 30usize, 2229056u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2229056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220340));
}
#[inline(always)]
pub fn block_0x00220340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2229060u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220524));
    } else {
        emu.pc = 2229064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220348));
    }
}
#[inline(always)]
pub fn block_0x00220348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2229068u32);
    emu.sb_no_count(22usize, 14usize, 4294967295u32, 2229072u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2229112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220378));
    } else {
        emu.pc = 2229076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220354));
    }
}
#[inline(always)]
pub fn block_0x00220354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2229080u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2229084u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2229088u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2229092u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2229096u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2229136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220390));
    } else {
        emu.pc = 2229100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022036c));
    }
}
#[inline(always)]
pub fn block_0x0022036c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 16usize, 2229104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220398));
    } else {
        emu.pc = 2229108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220374));
    }
}
#[inline(always)]
pub fn block_0x00220374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002203ac));
}
#[inline(always)]
pub fn block_0x00220378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2229116u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2229120u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2229124u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2229128u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2229132u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2229100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022036c));
    } else {
        emu.pc = 2229136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220390));
    }
}
#[inline(always)]
pub fn block_0x00220390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 15usize, 2229140u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203ac));
    } else {
        emu.pc = 2229144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220398));
    }
}
#[inline(always)]
pub fn block_0x00220398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2228964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202e4));
    } else {
        emu.pc = 2229148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022039c));
    }
}
#[inline(always)]
pub fn block_0x0022039c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 19usize, 16usize, 2229152u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2229156u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2229160u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2228980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202f4));
    } else {
        emu.pc = 2229164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203ac));
    }
}
#[inline(always)]
pub fn block_0x002203ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2228668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002201bc));
    } else {
        emu.pc = 2229168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203b0));
    }
}
#[inline(always)]
pub fn block_0x002203b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 28usize, 2229172u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2229176u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022048c));
    } else {
        emu.pc = 2229180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203bc));
    }
}
#[inline(always)]
pub fn block_0x002203bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a != b {
        emu.pc = 2229388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022048c));
    } else {
        emu.pc = 2229184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203c0));
    }
}
#[inline(always)]
pub fn block_0x002203c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 6usize, 16usize, 2229188u32);
    emu.sltru_no_count(10usize, 16usize, 6usize, 2229192u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2229196u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2229200u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220458));
    } else {
        emu.pc = 2229204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203d4));
    }
}
#[inline(always)]
pub fn block_0x002203d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 28usize, 2229208u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2229344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220460));
    } else {
        emu.pc = 2229212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203dc));
    }
}
#[inline(always)]
pub fn block_0x002203dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220540));
}
#[inline(always)]
pub fn block_0x002203e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 6usize, 13usize, 2229220u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2229512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220508));
    } else {
        emu.pc = 2229224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203e8));
    }
}
#[inline(always)]
pub fn block_0x002203e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 16usize, 2229228u32);
    emu.sbr_no_count(14usize, 17usize, 5usize, 2229232u32);
    emu.sbr_no_count(15usize, 10usize, 17usize, 2229236u32);
    emu.sltru_no_count(17usize, 6usize, 13usize, 2229240u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2229244u32);
    emu.sbr_no_count(12usize, 15usize, 17usize, 2229248u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022040c));
    } else {
        emu.pc = 2229252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220404));
    }
}
#[inline(always)]
pub fn block_0x00220404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 16usize, 2229256u32);
    emu.sbr_no_count(12usize, 6usize, 13usize, 2229260u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2229260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0022040c));
}
#[inline(always)]
pub fn block_0x0022040c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2229264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220508));
    } else {
        emu.pc = 2229268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220414));
    }
}
#[inline(always)]
pub fn block_0x00220414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 16usize, 2u32, 2229272u32);
    emu.sltiu_no_count(12usize, 5usize, 1u32, 2229276u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2229280u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220508));
    } else {
        emu.pc = 2229284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220424));
    }
}
#[inline(always)]
pub fn block_0x00220424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4294967292u32, 2229288u32);
    emu.sltru_no_count(12usize, 10usize, 8usize, 2229292u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2229296u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2229300u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2229504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220500));
    } else {
        emu.pc = 2229304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220438));
    }
}
#[inline(always)]
pub fn block_0x00220438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 5usize, 2229308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220508));
    } else {
        emu.pc = 2229312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220440));
    }
}
#[inline(always)]
pub fn block_0x00220440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 11usize, 2229316u32);
    emu.sw_no_count(7usize, 18usize, 0u32, 2229320u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2229324u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2229328u32)?;
    emu.sh_no_count(10usize, 18usize, 8u32, 2229332u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2229336u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220544));
}
#[inline(always)]
pub fn block_0x00220458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 13usize, 2229340u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2229568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220540));
    } else {
        emu.pc = 2229344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220460));
    }
}
#[inline(always)]
pub fn block_0x00220460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 6usize, 2229348u32);
    emu.sbr_no_count(14usize, 28usize, 7usize, 2229352u32);
    emu.sbr_no_count(15usize, 10usize, 28usize, 2229356u32);
    emu.sltru_no_count(28usize, 16usize, 13usize, 2229360u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2229364u32);
    emu.sbr_no_count(12usize, 15usize, 28usize, 2229368u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220484));
    } else {
        emu.pc = 2229372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022047c));
    }
}
#[inline(always)]
pub fn block_0x0022047c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 6usize, 2229376u32);
    emu.sbr_no_count(12usize, 16usize, 13usize, 2229380u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2229380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220484));
}
#[inline(always)]
pub fn block_0x00220484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2229384u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220540));
    } else {
        emu.pc = 2229388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0022048c));
    }
}
#[inline(always)]
pub fn block_0x0022048c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2229392u32);
    emu.mulhu_no_count(12usize, 8usize, 10usize, 2229396u32);
    emu.mul_no_count(13usize, 9usize, 10usize, 2229400u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2229404u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2229420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204ac));
    } else {
        emu.pc = 2229408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204a0));
    }
}
#[inline(always)]
pub fn block_0x002204a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 12usize, 2229412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204b8));
    } else {
        emu.pc = 2229416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204a8));
    }
}
#[inline(always)]
pub fn block_0x002204a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220540));
}
#[inline(always)]
pub fn block_0x002204ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(10usize, 8usize, 10usize, 2229424u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2229428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220540));
    } else {
        emu.pc = 2229432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204b8));
    }
}
#[inline]
pub fn block_0x002204b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967256u32, 2229436u32);
    emu.mul_no_count(12usize, 9usize, 10usize, 2229440u32);
    emu.mulhu_no_count(13usize, 8usize, 10usize, 2229444u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2229448u32);
    emu.sbr_no_count(13usize, 13usize, 8usize, 2229452u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2229456u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2229460u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2229464u32);
    emu.sltru_no_count(13usize, 10usize, 17usize, 2229468u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2229472u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2229560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220538));
    } else {
        emu.pc = 2229476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204e4));
    }
}
#[inline(always)]
pub fn block_0x002204e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 7usize, 2229480u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2229568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220540));
    } else {
        emu.pc = 2229484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204ec));
    }
}
#[inline(always)]
pub fn block_0x002204ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 29usize, 0u32, 2229488u32)?;
    emu.sw_no_count(11usize, 29usize, 4u32, 2229492u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2229496u32)?;
    emu.sh_no_count(10usize, 29usize, 8u32, 2229500u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2229504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220544));
}
#[inline(always)]
pub fn block_0x00220500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 16usize, 2229508u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220440));
    } else {
        emu.pc = 2229512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220508));
    }
}
#[inline(always)]
pub fn block_0x00220508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 18usize, 0u32, 2229516u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2229520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2229572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220544));
}
#[inline(always)]
pub fn block_0x00220510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2229524u32);
    emu.adi_no_count(16usize, 26usize, 0u32, 2229528u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2229532u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2228900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002202a4));
    } else {
        emu.pc = 2229536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220520));
    }
}
#[inline(always)]
pub fn block_0x00220520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220158));
}
#[inline(always)]
pub fn block_0x00220524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 0u32, 2229544u32);
    emu.adi_no_count(6usize, 21usize, 0u32, 2229548u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2229552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2229168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002203b0));
    } else {
        emu.pc = 2229556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220534));
    }
}
#[inline(always)]
pub fn block_0x00220534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2229560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2228668u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002201bc));
}
#[inline(always)]
pub fn block_0x00220538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 6usize, 2229564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2229484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002204ec));
    } else {
        emu.pc = 2229568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00220540));
    }
}
#[inline(always)]
pub fn block_0x00220540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 29usize, 0u32, 2229572u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2229572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00220544));
}
#[inline]
pub fn block_0x00220544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 124u32, 2229576u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2229580u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2229584u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2229588u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2229592u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2229596u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2229600u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2229604u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2229608u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2229612u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2229616u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2229620u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2229624u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2229628u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229632u32;
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
pub fn block_0x00220580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 52u32, 2229636u32)?;
    let a = 0u32.wrapping_add(2289664u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2229640u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967244u32, 2229644u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2229648u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2229652u32);
    emu.adi_no_count(13usize, 2usize, 52u32, 2229656u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2229660u32);
    emu.apc_no_count(1usize, 2229660u32, 4294955008u32, 2229664u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229668u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002205a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229672u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 264u32, 2229676u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229680u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 292u32, 2229684u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2229688u32);
    emu.apc_no_count(1usize, 2229688u32, 4294955008u32, 2229692u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002205c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 308u32, 2229704u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229708u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 340u32, 2229712u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2229716u32);
    emu.apc_no_count(1usize, 2229716u32, 4294955008u32, 2229720u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002205dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229728u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 356u32, 2229732u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229736u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 384u32, 2229740u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2229744u32);
    emu.apc_no_count(1usize, 2229744u32, 4294955008u32, 2229748u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002205f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229756u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 648u32, 2229760u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229764u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 704u32, 2229768u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2229772u32);
    emu.apc_no_count(1usize, 2229772u32, 4294955008u32, 2229776u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00220614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229784u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2229788u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229792u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 632u32, 2229796u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2229800u32);
    emu.apc_no_count(1usize, 2229800u32, 4294955008u32, 2229804u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00220630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229812u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 400u32, 2229816u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229820u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 448u32, 2229824u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2229828u32);
    emu.apc_no_count(1usize, 2229828u32, 4294955008u32, 2229832u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0022064c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2229840u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 464u32, 2229844u32);
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229848u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 512u32, 2229852u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2229856u32);
    emu.apc_no_count(1usize, 2229856u32, 4294955008u32, 2229860u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00220668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229868u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 544u32, 2229872u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2229876u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2229880u32);
    emu.apc_no_count(1usize, 2229880u32, 4294955008u32, 2229884u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00220680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229892u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 560u32, 2229896u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2229900u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2229904u32);
    emu.apc_no_count(1usize, 2229904u32, 4294955008u32, 2229908u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229912u32;
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
pub fn block_0x00220698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2285568u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2229916u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 248u32, 2229920u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2229924u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2229928u32);
    emu.apc_no_count(1usize, 2229928u32, 4294955008u32, 2229932u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2229936u32;
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
#[inline]
pub fn block_0x002206b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2229940u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2229944u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2229948u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2229952u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2229956u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2229960u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2229964u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2229968u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2229972u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2229976u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2229980u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2229984u32)?;
    emu.sw_no_count(26usize, 2usize, 0u32, 2229988u32)?;
    emu.adi_no_count(15usize, 14usize, 0u32, 2229992u32);
    emu.lw_no_count(17usize, 11usize, 0u32, 2229996u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2230000u32)?;
    emu.orr_no_count(14usize, 17usize, 16usize, 2230004u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2231600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2231600u32));
    } else {
        emu.pc = 2230008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2230008u32));
    }
}
