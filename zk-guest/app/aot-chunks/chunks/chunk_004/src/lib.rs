pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2108984u32;
pub const PC_MAX: u32 = 2111840u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 110usize] = [
        block_0x00202e38,
        block_0x00202e54,
        block_0x00202e68,
        block_0x00202e88,
        block_0x00202e98,
        block_0x00202e9c,
        block_0x00202f08,
        block_0x00202f14,
        block_0x00202f24,
        block_0x00202f34,
        block_0x00202f58,
        block_0x00202f60,
        block_0x00202f6c,
        block_0x00202f80,
        block_0x00202fb0,
        block_0x00202fb8,
        block_0x00202fc8,
        block_0x00202fd4,
        block_0x00202ff8,
        block_0x00203014,
        block_0x0020301c,
        block_0x00203098,
        block_0x002030b0,
        block_0x002030c4,
        block_0x002030cc,
        block_0x002030d0,
        block_0x00203110,
        block_0x00203124,
        block_0x00203138,
        block_0x00203148,
        block_0x002031a8,
        block_0x002031cc,
        block_0x002031dc,
        block_0x002031ec,
        block_0x002031f4,
        block_0x0020320c,
        block_0x0020322c,
        block_0x0020323c,
        block_0x00203240,
        block_0x002032ac,
        block_0x002032b8,
        block_0x002032cc,
        block_0x002032dc,
        block_0x002032e8,
        block_0x00203324,
        block_0x00203340,
        block_0x00203348,
        block_0x00203350,
        block_0x00203368,
        block_0x00203388,
        block_0x00203398,
        block_0x0020339c,
        block_0x002033b8,
        block_0x002033c0,
        block_0x0020342c,
        block_0x00203438,
        block_0x00203448,
        block_0x00203458,
        block_0x00203468,
        block_0x00203484,
        block_0x0020348c,
        block_0x00203494,
        block_0x002034b4,
        block_0x002034c8,
        block_0x002034d8,
        block_0x0020359c,
        block_0x002035b8,
        block_0x002035c0,
        block_0x002035dc,
        block_0x002035e0,
        block_0x0020362c,
        block_0x00203630,
        block_0x00203648,
        block_0x0020365c,
        block_0x00203668,
        block_0x00203688,
        block_0x0020368c,
        block_0x002036dc,
        block_0x002036e4,
        block_0x002036ec,
        block_0x002036fc,
        block_0x00203704,
        block_0x00203710,
        block_0x00203714,
        block_0x0020372c,
        block_0x00203750,
        block_0x00203754,
        block_0x00203774,
        block_0x00203780,
        block_0x00203794,
        block_0x002037ac,
        block_0x002037d4,
        block_0x002037e8,
        block_0x0020382c,
        block_0x00203830,
        block_0x00203834,
        block_0x0020383c,
        block_0x00203854,
        block_0x00203858,
        block_0x00203884,
        block_0x002038a0,
        block_0x002038a8,
        block_0x002038c4,
        block_0x002038d4,
        block_0x002038d8,
        block_0x002038e8,
        block_0x00203904,
        block_0x00203924,
        block_0x00203930,
        block_0x00203960,
    ];
    const IDX: [u16; 715usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 6u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 13u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 20u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 26u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 0u16, 0u16, 34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16,
        0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16,
        0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16,
        0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 71u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 79u16, 0u16, 80u16, 0u16,
        0u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 87u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 95u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16, 99u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 110u16,
    ];
    if pc < 2108984u32 || pc > 2111840u32 {
        return None;
    }
    let word_offset = ((pc - 2108984u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00202e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2108988u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2108992u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2108996u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2109000u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2109004u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2109008u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2109432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ff8));
    } else {
        emu.pc = 2109012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e54));
    }
}
#[inline(always)]
pub fn block_0x00202e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 15usize, 0u32, 2109016u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2109020u32);
    emu.lw_no_count(10usize, 11usize, 4u32, 2109024u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2109028u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2109084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e9c));
    } else {
        emu.pc = 2109032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e68));
    }
}
#[inline(always)]
pub fn block_0x00202e68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2109036u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109040u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2109044u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2109048u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2109052u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2109056u32);
    emu.apc_no_count(1usize, 2109056u32, 12288u32, 2109060u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2109068u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2109072u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2109076u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2109204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f14));
    } else {
        emu.pc = 2109080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202e98));
    }
}
#[inline(always)]
pub fn block_0x00202e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2109084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f34));
}
#[inline(never)]
pub fn block_0x00202e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 0u32, 2109088u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2109092u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2109096u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2109100u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2109104u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2109108u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2109112u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2109116u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2109120u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2109124u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2109128u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2109132u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2109136u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2109140u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2109144u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2109148u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2109152u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2109156u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2109160u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2109164u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2109168u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2109172u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2109176u32);
    emu.sw_no_count(17usize, 9usize, 0u32, 2109180u32)?;
    emu.sw_no_count(16usize, 9usize, 4u32, 2109184u32)?;
    emu.apc_no_count(1usize, 2109184u32, 12288u32, 2109188u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2109196u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2109200u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2109236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f34));
    } else {
        emu.pc = 2109204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f14));
    }
}
#[inline(always)]
pub fn block_0x00202f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2109208u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2109212u32);
    emu.apc_no_count(1usize, 2109212u32, 4096u32, 2109216u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109220u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2109224u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2109228u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109232u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2109272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f58));
    } else {
        emu.pc = 2109236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f34));
    }
}
#[inline]
pub fn block_0x00202f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109240u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109244u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2109248u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2109252u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2109256u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2109260u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2109264u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2109268u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109272u32;
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
pub fn block_0x00202f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2109276u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2109592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203098));
    } else {
        emu.pc = 2109280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f60));
    }
}
#[inline(always)]
pub fn block_0x00202f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 9usize, 4u32, 2109284u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2109288u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2109312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f80));
    } else {
        emu.pc = 2109292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202f6c));
    }
}
#[inline(always)]
pub fn block_0x00202f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109296u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2109300u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2109304u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2109308u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2109312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202fc8));
}
#[inline]
pub fn block_0x00202f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 9usize, 0u32, 2109316u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2109320u32)?;
    emu.adi_no_count(6usize, 5usize, 4294967292u32, 2109324u32);
    emu.adi_no_count(7usize, 16usize, 4u32, 2109328u32);
    emu.lbu_no_count(13usize, 16usize, 0u32, 2109332u32);
    emu.lbu_no_count(17usize, 16usize, 1u32, 2109336u32);
    emu.lbu_no_count(14usize, 16usize, 2u32, 2109340u32);
    emu.lbu_no_count(15usize, 16usize, 3u32, 2109344u32);
    emu.adi_no_count(28usize, 0usize, 2u32, 2109348u32);
    emu.sw_no_count(7usize, 9usize, 0u32, 2109352u32)?;
    emu.sw_no_count(6usize, 9usize, 4u32, 2109356u32)?;
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2109616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002030b0));
    } else {
        emu.pc = 2109360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fb0));
    }
}
#[inline(always)]
pub fn block_0x00202fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 0usize, 4u32, 2109364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2109468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020301c));
    } else {
        emu.pc = 2109368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202fb8));
    }
}
#[inline(always)]
pub fn block_0x00202fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109372u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1281u32, 2109376u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2109380u32)?;
    emu.sw_no_count(7usize, 2usize, 12u32, 2109384u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2109384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202fc8));
}
#[inline(always)]
pub fn block_0x00202fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2109388u32);
    emu.apc_no_count(1usize, 2109388u32, 12288u32, 2109392u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109396u32;
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
#[inline]
pub fn block_0x00202fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109400u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2109404u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2109408u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2109412u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2109416u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2109420u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2109424u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2109428u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109432u32;
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
pub fn block_0x00202ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109436u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966204u32, 2109440u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109444u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2109448u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2109452u32);
    emu.apc_no_count(1usize, 2109452u32, 4096u32, 2109456u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109460u32;
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
pub fn block_0x00203014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2109464u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2109468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202f34));
}
#[inline(never)]
pub fn block_0x0020301c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 16usize, 8u32, 2109472u32);
    emu.adi_no_count(5usize, 5usize, 4294967288u32, 2109476u32);
    emu.lbu_no_count(7usize, 16usize, 4u32, 2109480u32);
    emu.lbu_no_count(28usize, 16usize, 5u32, 2109484u32);
    emu.lbu_no_count(29usize, 16usize, 6u32, 2109488u32);
    emu.lbu_no_count(16usize, 16usize, 7u32, 2109492u32);
    emu.sw_no_count(6usize, 9usize, 0u32, 2109496u32)?;
    emu.sw_no_count(5usize, 9usize, 4u32, 2109500u32)?;
    emu.sli_no_count(17usize, 17usize, 8u32, 2109504u32);
    emu.orr_no_count(13usize, 17usize, 13usize, 2109508u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2109512u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2109516u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2109520u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2109524u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2109528u32);
    emu.sli_no_count(29usize, 29usize, 16u32, 2109532u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2109536u32);
    emu.orr_no_count(14usize, 28usize, 7usize, 2109540u32);
    emu.orr_no_count(15usize, 16usize, 29usize, 2109544u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2109548u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2109552u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2109556u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2109560u32)?;
    emu.sw_no_count(13usize, 8usize, 12u32, 2109564u32)?;
    emu.sw_no_count(14usize, 8usize, 16u32, 2109568u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2109572u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2109576u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2109580u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2109584u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2109588u32);
    emu.add_memory_rw_events(31usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109592u32;
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
pub fn block_0x00203098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109596u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966204u32, 2109600u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109604u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2109608u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2109612u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2109616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109636u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002030c4));
}
#[inline(always)]
pub fn block_0x002030b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109620u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966204u32, 2109624u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2109628u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2109632u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2109636u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2109636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002030c4));
}
#[inline(always)]
pub fn block_0x002030c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2109636u32, 4096u32, 2109640u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002030cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2109648u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202fd4));
}
#[inline]
pub fn block_0x002030d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2109652u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2109656u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2109660u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2109664u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2109668u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2109672u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2109676u32)?;
    emu.sw_no_count(21usize, 2usize, 180u32, 2109680u32)?;
    emu.sw_no_count(22usize, 2usize, 176u32, 2109684u32)?;
    emu.sw_no_count(23usize, 2usize, 172u32, 2109688u32)?;
    emu.sw_no_count(24usize, 2usize, 168u32, 2109692u32)?;
    emu.sw_no_count(25usize, 2usize, 164u32, 2109696u32)?;
    emu.sw_no_count(26usize, 2usize, 160u32, 2109700u32)?;
    emu.sw_no_count(27usize, 2usize, 156u32, 2109704u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2109708u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2110244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203324));
    } else {
        emu.pc = 2109712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203110));
    }
}
#[inline(always)]
pub fn block_0x00203110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 15usize, 0u32, 2109716u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2109720u32);
    emu.adi_no_count(10usize, 2usize, 72u32, 2109724u32);
    emu.apc_no_count(1usize, 2109724u32, 4294963200u32, 2109728u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109732u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 72u32, 2109736u32)?;
    emu.lw_no_count(9usize, 2usize, 76u32, 2109740u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109744u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2109748u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2109768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203148));
    } else {
        emu.pc = 2109752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203138));
    }
}
#[inline(always)]
pub fn block_0x00203138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109756u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109760u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2109764u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline]
pub fn block_0x00203148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 80u32, 2109772u32)?;
    emu.lw_no_count(11usize, 2usize, 84u32, 2109776u32)?;
    emu.lw_no_count(12usize, 2usize, 88u32, 2109780u32)?;
    emu.lw_no_count(13usize, 2usize, 92u32, 2109784u32)?;
    emu.lw_no_count(20usize, 2usize, 96u32, 2109788u32)?;
    emu.lw_no_count(26usize, 2usize, 100u32, 2109792u32)?;
    emu.lw_no_count(5usize, 2usize, 104u32, 2109796u32)?;
    emu.lw_no_count(6usize, 2usize, 108u32, 2109800u32)?;
    emu.lw_no_count(14usize, 2usize, 112u32, 2109804u32)?;
    emu.lw_no_count(15usize, 2usize, 116u32, 2109808u32)?;
    emu.lw_no_count(16usize, 2usize, 120u32, 2109812u32)?;
    emu.lw_no_count(17usize, 2usize, 124u32, 2109816u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2109820u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2109824u32)?;
    emu.sw_no_count(12usize, 2usize, 64u32, 2109828u32)?;
    emu.sw_no_count(13usize, 2usize, 68u32, 2109832u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2109836u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2109840u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2109844u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2109848u32)?;
    emu.sw_no_count(17usize, 2usize, 48u32, 2109852u32)?;
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2109856u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2109860u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020339c));
    } else {
        emu.pc = 2109864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031a8));
    }
}
#[inline]
pub fn block_0x002031a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(6usize, 2usize, 28u32, 2109868u32)?;
    emu.sw_no_count(5usize, 2usize, 32u32, 2109872u32)?;
    emu.lw_no_count(27usize, 2usize, 132u32, 2109876u32)?;
    emu.lw_no_count(21usize, 2usize, 136u32, 2109880u32)?;
    emu.lw_no_count(23usize, 2usize, 140u32, 2109884u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2109888u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2109892u32);
    emu.apc_no_count(1usize, 2109892u32, 4294963200u32, 2109896u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002031cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 72u32, 2109904u32)?;
    emu.lw_no_count(25usize, 2usize, 76u32, 2109908u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109912u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2109932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031ec));
    } else {
        emu.pc = 2109916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031dc));
    }
}
#[inline(always)]
pub fn block_0x002031dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2109920u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2109924u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2109928u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2109932u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline(always)]
pub fn block_0x002031ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2109936u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203468));
    } else {
        emu.pc = 2109940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002031f4));
    }
}
#[inline(always)]
pub fn block_0x002031f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 24u32, 2109944u32)?;
    emu.lw_no_count(10usize, 18usize, 4u32, 2109948u32)?;
    emu.lw_no_count(11usize, 2usize, 80u32, 2109952u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2109956u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2109960u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2110016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203240));
    } else {
        emu.pc = 2109964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020320c));
    }
}
#[inline(always)]
pub fn block_0x0020320c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2109968u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2109972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2109976u32);
    emu.sw_no_count(11usize, 2usize, 144u32, 2109980u32)?;
    emu.sw_no_count(10usize, 2usize, 148u32, 2109984u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2109988u32);
    emu.apc_no_count(1usize, 2109988u32, 12288u32, 2109992u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020322c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2110000u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2110004u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110008u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2110136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032b8));
    } else {
        emu.pc = 2110012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020323c));
    }
}
#[inline(always)]
pub fn block_0x0020323c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032dc));
}
#[inline(never)]
pub fn block_0x00203240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2110020u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2110024u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2110028u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2110032u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2110036u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2110040u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2110044u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2110048u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2110052u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2110056u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2110060u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2110064u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2110068u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2110072u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2110076u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2110080u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2110084u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2110088u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2110092u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2110096u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2110100u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2110104u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2110108u32);
    emu.sw_no_count(17usize, 18usize, 0u32, 2110112u32)?;
    emu.sw_no_count(16usize, 18usize, 4u32, 2110116u32)?;
    emu.apc_no_count(1usize, 2110116u32, 12288u32, 2110120u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965940u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 11usize, 0u32, 2110128u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110132u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2110172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032dc));
    } else {
        emu.pc = 2110136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032b8));
    }
}
#[inline(always)]
pub fn block_0x002032b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2110140u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110144u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2110148u32);
    emu.apc_no_count(1usize, 2110148u32, 0u32, 2110152u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 72u32, 2110160u32)?;
    emu.lw_no_count(20usize, 2usize, 76u32, 2110164u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110168u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203348));
    } else {
        emu.pc = 2110172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002032dc));
    }
}
#[inline(always)]
pub fn block_0x002032dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110176u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110180u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2110184u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2110184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline]
pub fn block_0x002032e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 204u32, 2110188u32)?;
    emu.lw_no_count(8usize, 2usize, 200u32, 2110192u32)?;
    emu.lw_no_count(9usize, 2usize, 196u32, 2110196u32)?;
    emu.lw_no_count(18usize, 2usize, 192u32, 2110200u32)?;
    emu.lw_no_count(19usize, 2usize, 188u32, 2110204u32)?;
    emu.lw_no_count(20usize, 2usize, 184u32, 2110208u32)?;
    emu.lw_no_count(21usize, 2usize, 180u32, 2110212u32)?;
    emu.lw_no_count(22usize, 2usize, 176u32, 2110216u32)?;
    emu.lw_no_count(23usize, 2usize, 172u32, 2110220u32)?;
    emu.lw_no_count(24usize, 2usize, 168u32, 2110224u32)?;
    emu.lw_no_count(25usize, 2usize, 164u32, 2110228u32)?;
    emu.lw_no_count(26usize, 2usize, 160u32, 2110232u32)?;
    emu.lw_no_count(27usize, 2usize, 156u32, 2110236u32)?;
    emu.adi_no_count(2usize, 2usize, 208u32, 2110240u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110244u32;
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
pub fn block_0x00203324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110248u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966144u32, 2110252u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2110260u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2110264u32);
    emu.apc_no_count(1usize, 2110264u32, 4096u32, 2110268u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(88u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2110276u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203138));
}
#[inline(always)]
pub fn block_0x00203348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2110284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020359c));
    } else {
        emu.pc = 2110288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203350));
    }
}
#[inline(always)]
pub fn block_0x00203350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 16u32, 2110292u32)?;
    emu.lw_no_count(10usize, 18usize, 4u32, 2110296u32)?;
    emu.lw_no_count(11usize, 2usize, 80u32, 2110300u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2110304u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2110308u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2110400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002033c0));
    } else {
        emu.pc = 2110312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203368));
    }
}
#[inline(always)]
pub fn block_0x00203368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2110316u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2110324u32);
    emu.sw_no_count(11usize, 2usize, 144u32, 2110328u32)?;
    emu.sw_no_count(10usize, 2usize, 148u32, 2110332u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2110336u32);
    emu.apc_no_count(1usize, 2110336u32, 12288u32, 2110340u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110344u32;
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
#[inline(always)]
pub fn block_0x00203388(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2110348u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2110352u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110356u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2110520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203438));
    } else {
        emu.pc = 2110360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203398));
    }
}
#[inline(always)]
pub fn block_0x00203398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203458));
}
#[inline(always)]
pub fn block_0x0020339c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110368u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966144u32, 2110372u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110376u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2110380u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2110384u32);
    emu.apc_no_count(1usize, 2110384u32, 4096u32, 2110388u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110392u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002033b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 10usize, 0u32, 2110396u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110400u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2109916u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002031dc));
}
#[inline(never)]
pub fn block_0x002033c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2110404u32)?;
    emu.lbu_no_count(12usize, 11usize, 1u32, 2110408u32);
    emu.lbu_no_count(13usize, 11usize, 2u32, 2110412u32);
    emu.lbu_no_count(14usize, 11usize, 3u32, 2110416u32);
    emu.lbu_no_count(15usize, 11usize, 0u32, 2110420u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2110424u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2110428u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2110432u32);
    emu.orr_no_count(12usize, 12usize, 15usize, 2110436u32);
    emu.lbu_no_count(15usize, 11usize, 4u32, 2110440u32);
    emu.lbu_no_count(16usize, 11usize, 5u32, 2110444u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2110448u32);
    emu.lbu_no_count(14usize, 11usize, 6u32, 2110452u32);
    emu.lbu_no_count(17usize, 11usize, 7u32, 2110456u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2110460u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2110464u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2110468u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2110472u32);
    emu.orr_no_count(14usize, 17usize, 14usize, 2110476u32);
    emu.adi_no_count(16usize, 10usize, 4294967288u32, 2110480u32);
    emu.adi_no_count(17usize, 11usize, 8u32, 2110484u32);
    emu.orr_no_count(10usize, 13usize, 12usize, 2110488u32);
    emu.orr_no_count(11usize, 14usize, 15usize, 2110492u32);
    emu.sw_no_count(17usize, 18usize, 0u32, 2110496u32)?;
    emu.sw_no_count(16usize, 18usize, 4u32, 2110500u32)?;
    emu.apc_no_count(1usize, 2110500u32, 12288u32, 2110504u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020342c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2110512u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2110516u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2110552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203458));
    } else {
        emu.pc = 2110520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203438));
    }
}
#[inline(always)]
pub fn block_0x00203438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2110524u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110528u32);
    emu.apc_no_count(1usize, 2110528u32, 0u32, 2110532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 72u32, 2110540u32)?;
    emu.lw_no_count(12usize, 2usize, 76u32, 2110544u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110548u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2110604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020348c));
    } else {
        emu.pc = 2110552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203458));
    }
}
#[inline(always)]
pub fn block_0x00203458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110556u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110560u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2110564u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline(always)]
pub fn block_0x00203468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966144u32, 2110576u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110580u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2110584u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2110588u32);
    emu.apc_no_count(1usize, 2110588u32, 4096u32, 2110592u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2110600u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110604u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110172u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032dc));
}
#[inline(always)]
pub fn block_0x0020348c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2110608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2110912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035c0));
    } else {
        emu.pc = 2110612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203494));
    }
}
#[inline(always)]
pub fn block_0x00203494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 11usize, 0u32, 2110616u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2110620u32)?;
    emu.lw_no_count(10usize, 2usize, 80u32, 2110624u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2110628u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2110632u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110636u32);
    emu.apc_no_count(1usize, 2110636u32, 4294963200u32, 2110640u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110644u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 72u32, 2110648u32)?;
    emu.lw_no_count(10usize, 2usize, 76u32, 2110652u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110656u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2110660u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2110680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034d8));
    } else {
        emu.pc = 2110664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034c8));
    }
}
#[inline(always)]
pub fn block_0x002034c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110668u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2110672u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2110676u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline(never)]
pub fn block_0x002034d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 49u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 80u32, 2110684u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2110688u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2110692u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2110696u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2110700u32)?;
    emu.sw_no_count(24usize, 8usize, 0u32, 2110704u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2110708u32)?;
    emu.lw_no_count(17usize, 2usize, 20u32, 2110712u32)?;
    emu.sw_no_count(17usize, 8usize, 8u32, 2110716u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2110720u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2110724u32)?;
    emu.lw_no_count(17usize, 2usize, 36u32, 2110728u32)?;
    emu.lw_no_count(5usize, 2usize, 40u32, 2110732u32)?;
    emu.lw_no_count(6usize, 2usize, 44u32, 2110736u32)?;
    emu.lw_no_count(7usize, 2usize, 48u32, 2110740u32)?;
    emu.sw_no_count(20usize, 8usize, 16u32, 2110744u32)?;
    emu.lw_no_count(28usize, 2usize, 12u32, 2110748u32)?;
    emu.sw_no_count(28usize, 8usize, 20u32, 2110752u32)?;
    emu.sw_no_count(19usize, 8usize, 24u32, 2110756u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2110760u32)?;
    emu.sw_no_count(28usize, 8usize, 28u32, 2110764u32)?;
    emu.lw_no_count(28usize, 2usize, 52u32, 2110768u32)?;
    emu.sw_no_count(27usize, 8usize, 96u32, 2110772u32)?;
    emu.sw_no_count(21usize, 8usize, 100u32, 2110776u32)?;
    emu.sw_no_count(23usize, 8usize, 104u32, 2110780u32)?;
    emu.sw_no_count(12usize, 8usize, 108u32, 2110784u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2110788u32)?;
    emu.sw_no_count(12usize, 8usize, 32u32, 2110792u32)?;
    emu.sw_no_count(22usize, 8usize, 36u32, 2110796u32)?;
    emu.sw_no_count(9usize, 8usize, 40u32, 2110800u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2110804u32)?;
    emu.sw_no_count(26usize, 8usize, 64u32, 2110808u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2110812u32)?;
    emu.sw_no_count(12usize, 8usize, 68u32, 2110816u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2110820u32)?;
    emu.sw_no_count(12usize, 8usize, 72u32, 2110824u32)?;
    emu.sw_no_count(17usize, 8usize, 76u32, 2110828u32)?;
    emu.sw_no_count(5usize, 8usize, 80u32, 2110832u32)?;
    emu.sw_no_count(6usize, 8usize, 84u32, 2110836u32)?;
    emu.sw_no_count(7usize, 8usize, 88u32, 2110840u32)?;
    emu.sw_no_count(28usize, 8usize, 92u32, 2110844u32)?;
    emu.sw_no_count(14usize, 8usize, 48u32, 2110848u32)?;
    emu.sw_no_count(15usize, 8usize, 52u32, 2110852u32)?;
    emu.sw_no_count(16usize, 8usize, 56u32, 2110856u32)?;
    emu.lw_no_count(12usize, 2usize, 24u32, 2110860u32)?;
    emu.sw_no_count(12usize, 8usize, 60u32, 2110864u32)?;
    emu.sw_no_count(10usize, 8usize, 112u32, 2110868u32)?;
    emu.sw_no_count(11usize, 8usize, 116u32, 2110872u32)?;
    emu.add_memory_rw_events(49usize);
    let return_addr = 2110876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002032e8));
}
#[inline(always)]
pub fn block_0x0020359c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110880u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966144u32, 2110884u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2110892u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2110896u32);
    emu.apc_no_count(1usize, 2110896u32, 4096u32, 2110900u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2110908u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2110912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203458));
}
#[inline(always)]
pub fn block_0x002035c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110916u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966144u32, 2110920u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2110924u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2110928u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2110932u32);
    emu.apc_no_count(1usize, 2110932u32, 4096u32, 2110936u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110940u32;
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
#[inline(always)]
pub fn block_0x002035dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110944u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002034c8));
}
#[inline]
pub fn block_0x002035e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2110948u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2110952u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2110956u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2110960u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2110964u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2110968u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2110972u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2110976u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2110980u32)?;
    emu.adi_no_count(19usize, 12usize, 0u32, 2110984u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2110988u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2110992u32);
    emu.lw_no_count(9usize, 11usize, 16u32, 2110996u32)?;
    emu.lw_no_count(20usize, 11usize, 12u32, 2111000u32)?;
    emu.lw_no_count(11usize, 11usize, 8u32, 2111004u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2111008u32)?;
    emu.adi_no_count(9usize, 9usize, 1u32, 2111012u32);
    emu.adi_no_count(20usize, 20usize, 4294967295u32, 2111016u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2111020u32);
    emu.add_memory_rw_events(19usize);
    emu.pc = 2111020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020362c));
}
#[inline(always)]
pub fn block_0x0020362c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2111360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203780));
    } else {
        emu.pc = 2111024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203630));
    }
}
#[inline(always)]
pub fn block_0x00203630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 9usize, 11usize, 2111028u32);
    emu.sw_no_count(22usize, 18usize, 8u32, 2111032u32)?;
    emu.sw_no_count(20usize, 18usize, 12u32, 2111036u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2111040u32);
    emu.apc_no_count(1usize, 2111040u32, 4294955008u32, 2111044u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2111052u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2111056u32);
    emu.adi_no_count(20usize, 20usize, 4294967295u32, 2111060u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2111064u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2111020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020362c));
    } else {
        emu.pc = 2111068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020365c));
    }
}
#[inline(always)]
pub fn block_0x0020365c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 12usize, 0u32, 2111072u32);
    emu.apc_no_count(1usize, 2111072u32, 4096u32, 2111076u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2111088u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2111092u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2111096u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2111100u32);
    emu.adi_no_count(22usize, 0usize, 8u32, 2111104u32);
    emu.apc_no_count(1usize, 2111104u32, 8192u32, 2111108u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002037d4));
    } else {
        emu.pc = 2111116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020368c));
    }
}
#[inline]
pub fn block_0x0020368c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2111120u32);
    emu.sb_no_count(21usize, 10usize, 0u32, 2111124u32);
    emu.lw_no_count(10usize, 18usize, 16u32, 2111128u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2111132u32)?;
    emu.sw_no_count(22usize, 2usize, 8u32, 2111136u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2111140u32)?;
    emu.sw_no_count(20usize, 2usize, 16u32, 2111144u32)?;
    emu.adi_no_count(20usize, 2usize, 20u32, 2111148u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2111152u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2111156u32)?;
    emu.lw_no_count(12usize, 18usize, 0u32, 2111160u32)?;
    emu.lw_no_count(13usize, 18usize, 4u32, 2111164u32)?;
    emu.lw_no_count(14usize, 18usize, 8u32, 2111168u32)?;
    emu.lw_no_count(11usize, 18usize, 12u32, 2111172u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2111176u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2111180u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2111184u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2111188u32)?;
    emu.sw_no_count(20usize, 2usize, 44u32, 2111192u32)?;
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2111380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203794));
    } else {
        emu.pc = 2111196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036dc));
    }
}
#[inline(always)]
pub fn block_0x002036dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 28u32, 2111200u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2111204u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2111204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002036e4));
}
#[inline(always)]
pub fn block_0x002036e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 1u32, 2111208u32);
    emu.sbr_no_count(22usize, 0usize, 11usize, 2111212u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2111212u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002036ec));
}
#[inline(always)]
pub fn block_0x002036ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 44u32, 2111216u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111220u32);
    emu.apc_no_count(1usize, 2111220u32, 4294955008u32, 2111224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2111232u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2111252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203714));
    } else {
        emu.pc = 2111236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203704));
    }
}
#[inline(always)]
pub fn block_0x00203704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 1u32, 2111240u32);
    emu.adr_no_count(19usize, 19usize, 21usize, 2111244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a != b {
        emu.pc = 2111212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036ec));
    } else {
        emu.pc = 2111248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203710));
    }
}
#[inline(always)]
pub fn block_0x00203710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203794));
}
#[inline(always)]
pub fn block_0x00203714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2111256u32)?;
    emu.adr_no_count(19usize, 21usize, 19usize, 2111260u32);
    emu.xri_no_count(12usize, 22usize, 4294967295u32, 2111264u32);
    emu.sw_no_count(19usize, 2usize, 28u32, 2111268u32)?;
    emu.sw_no_count(12usize, 2usize, 32u32, 2111272u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2111316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203754));
    } else {
        emu.pc = 2111276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020372c));
    }
}
#[inline]
pub fn block_0x0020372c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 9usize, 18usize, 2111280u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2111284u32);
    emu.lw_no_count(19usize, 2usize, 28u32, 2111288u32)?;
    emu.lw_no_count(11usize, 2usize, 32u32, 2111292u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2111296u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2111300u32);
    emu.sw_no_count(18usize, 2usize, 16u32, 2111304u32)?;
    emu.sw_no_count(20usize, 2usize, 44u32, 2111308u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2111204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036e4));
    } else {
        emu.pc = 2111312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203750));
    }
}
#[inline(always)]
pub fn block_0x00203750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111316u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111380u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203794));
}
#[inline(always)]
pub fn block_0x00203754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2111320u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2111324u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2111328u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2111332u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2111336u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2111340u32);
    emu.apc_no_count(1usize, 2111340u32, 4096u32, 2111344u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2111352u32);
    emu.lw_no_count(9usize, 2usize, 12u32, 2111356u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2111360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020372c));
}
#[inline(always)]
pub fn block_0x00203780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2111364u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2111368u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2111372u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2111376u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2111380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037ac));
}
#[inline(always)]
pub fn block_0x00203794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2111384u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2111388u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2111392u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2111396u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2111400u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2111404u32)?;
    emu.add_memory_rw_events(6usize);
    emu.pc = 2111404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037ac));
}
#[inline]
pub fn block_0x002037ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2111408u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2111412u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2111416u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2111420u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2111424u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2111428u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2111432u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2111436u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2111440u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111444u32;
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
pub fn block_0x002037d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2111448u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2111452u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2111456u32);
    emu.apc_no_count(1usize, 2111456u32, 40960u32, 2111460u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002037e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2111468u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2111472u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2111476u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2111480u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2111484u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2111488u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2111492u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2111496u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2111500u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2111504u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2111508u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2111512u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2111516u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2111520u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111524u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2111528u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2111536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203830));
    } else {
        emu.pc = 2111532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020382c));
    }
}
#[inline(always)]
pub fn block_0x0020382c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2111536u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203830));
}
#[inline(always)]
pub fn block_0x00203830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038d8));
    } else {
        emu.pc = 2111540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203834));
    }
}
#[inline(always)]
pub fn block_0x00203834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2111540u32, 4096u32, 2111544u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111548u32;
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
pub fn block_0x0020383c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111552u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2111556u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2111560u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2111564u32);
    emu.apc_no_count(1usize, 2111564u32, 8192u32, 2111568u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111572u32;
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
pub fn block_0x00203854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203960));
    } else {
        emu.pc = 2111576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203858));
    }
}
#[inline]
pub fn block_0x00203858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2111580u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2111584u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2111588u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2111592u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2111596u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2111600u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2111604u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2111608u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2111612u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294966460u32, 2111616u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2111620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038a0));
}
#[inline(always)]
pub fn block_0x00203884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2111624u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2111628u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2111632u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2111636u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2111640u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2111644u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2111720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038e8));
    } else {
        emu.pc = 2111648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038a0));
    }
}
#[inline(always)]
pub fn block_0x002038a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 22usize, 20usize, 2111652u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2111748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203904));
    } else {
        emu.pc = 2111656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038a8));
    }
}
#[inline(always)]
pub fn block_0x002038a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2111660u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2111664u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2111668u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2111672u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2111676u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2111680u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2111620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203884));
    } else {
        emu.pc = 2111684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c4));
    }
}
#[inline(always)]
pub fn block_0x002038c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2111688u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111692u32);
    emu.apc_no_count(1usize, 2111692u32, 40960u32, 2111696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002038d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111704u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111620u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203884));
}
#[inline(always)]
pub fn block_0x002038d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2111708u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2111712u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2111716u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2111720u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2111720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038e8));
}
#[inline(always)]
pub fn block_0x002038e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2111724u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2111728u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2111732u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2111736u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2111740u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2111744u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2111748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203930));
}
#[inline(always)]
pub fn block_0x00203904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2111752u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111756u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2111760u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2111764u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2111768u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2111772u32);
    emu.apc_no_count(1usize, 2111772u32, 8192u32, 2111776u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1320u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111784u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2111788u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2111792u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2111792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203930));
}
#[inline]
pub fn block_0x00203930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2111796u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2111800u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2111804u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2111808u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2111812u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2111816u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2111820u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2111824u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2111828u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2111832u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2111836u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111840u32;
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
pub fn block_0x00203960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111844u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966444u32, 2111848u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2111852u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111856u32);
    emu.apc_no_count(1usize, 2111856u32, 40960u32, 2111860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
