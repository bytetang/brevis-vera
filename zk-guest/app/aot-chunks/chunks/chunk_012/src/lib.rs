pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2139356u32;
pub const PC_MAX: u32 = 2141964u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x0020a4dc,
        block_0x0020a4f0,
        block_0x0020a4f8,
        block_0x0020a52c,
        block_0x0020a534,
        block_0x0020a53c,
        block_0x0020a544,
        block_0x0020a54c,
        block_0x0020a554,
        block_0x0020a57c,
        block_0x0020a5dc,
        block_0x0020a5e4,
        block_0x0020a5f4,
        block_0x0020a5fc,
        block_0x0020a600,
        block_0x0020a630,
        block_0x0020a638,
        block_0x0020a650,
        block_0x0020a658,
        block_0x0020a66c,
        block_0x0020a684,
        block_0x0020a6e4,
        block_0x0020a6ec,
        block_0x0020a70c,
        block_0x0020a76c,
        block_0x0020a770,
        block_0x0020a774,
        block_0x0020a77c,
        block_0x0020a784,
        block_0x0020a78c,
        block_0x0020a7b8,
        block_0x0020a7c0,
        block_0x0020a7c8,
        block_0x0020a7cc,
        block_0x0020a7f0,
        block_0x0020a7fc,
        block_0x0020a890,
        block_0x0020a8e4,
        block_0x0020a8e8,
        block_0x0020a904,
        block_0x0020a9a0,
        block_0x0020a9cc,
        block_0x0020a9d8,
        block_0x0020a9e4,
        block_0x0020a9ec,
        block_0x0020aa10,
        block_0x0020aa1c,
        block_0x0020aa28,
        block_0x0020aa64,
        block_0x0020aa80,
        block_0x0020aab0,
        block_0x0020aad0,
        block_0x0020aaf4,
        block_0x0020ab1c,
        block_0x0020ab20,
        block_0x0020ab28,
        block_0x0020ab38,
        block_0x0020ab48,
        block_0x0020ab70,
        block_0x0020ab98,
        block_0x0020abac,
        block_0x0020abc4,
        block_0x0020ac28,
        block_0x0020ac30,
        block_0x0020ac40,
        block_0x0020ac44,
        block_0x0020ac74,
        block_0x0020ac84,
        block_0x0020aca4,
        block_0x0020acb4,
        block_0x0020acb8,
        block_0x0020acdc,
        block_0x0020acec,
        block_0x0020ad00,
        block_0x0020ad1c,
        block_0x0020ad2c,
        block_0x0020ad34,
        block_0x0020ad48,
        block_0x0020ad4c,
        block_0x0020ad5c,
        block_0x0020ad60,
        block_0x0020ad68,
        block_0x0020ad78,
        block_0x0020ad7c,
        block_0x0020ad88,
        block_0x0020ad8c,
        block_0x0020ad90,
        block_0x0020adac,
        block_0x0020adb4,
        block_0x0020adb8,
        block_0x0020ade4,
        block_0x0020ade8,
        block_0x0020adf4,
        block_0x0020ae04,
        block_0x0020ae08,
        block_0x0020ae10,
        block_0x0020ae1c,
        block_0x0020ae30,
        block_0x0020ae48,
        block_0x0020ae50,
        block_0x0020ae74,
        block_0x0020ae7c,
        block_0x0020ae88,
        block_0x0020aeb0,
        block_0x0020aeb4,
        block_0x0020aecc,
        block_0x0020aed8,
        block_0x0020aee4,
        block_0x0020aef8,
        block_0x0020aefc,
        block_0x0020af08,
        block_0x0020af0c,
    ];
    const IDX: [u16; 653usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 5u16, 0u16, 6u16, 0u16,
        7u16, 0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 12u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 27u16, 0u16, 28u16,
        0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 31u16, 0u16, 32u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 45u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 55u16, 0u16,
        56u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 80u16,
        81u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 85u16, 86u16,
        87u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 92u16, 0u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 94u16, 95u16, 0u16, 96u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 103u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        0u16, 109u16, 110u16, 0u16, 0u16, 111u16, 112u16,
    ];
    if pc < 2139356u32 || pc > 2141964u32 {
        return None;
    }
    let word_offset = ((pc - 2139356u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020a4dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(13usize, 11usize, 3u32, 2139360u32);
    emu.sltiu_no_count(13usize, 13usize, 1u32, 2139364u32);
    emu.sltiu_no_count(14usize, 12usize, 1u32, 2139368u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2139372u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2139620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5e4));
    } else {
        emu.pc = 2139376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4f0));
    }
}
#[inline(always)]
pub fn block_0x0020a4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 11usize, 1u32, 2139380u32);
    emu.adi_no_count(16usize, 10usize, 0u32, 2139384u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a4f8));
}
#[inline]
pub fn block_0x0020a4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 11usize, 0u32, 2139388u32);
    emu.adi_no_count(14usize, 11usize, 1u32, 2139392u32);
    emu.adi_no_count(13usize, 16usize, 1u32, 2139396u32);
    emu.sb_no_count(17usize, 16usize, 0u32, 2139400u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2139404u32);
    emu.ani_no_count(11usize, 15usize, 3u32, 2139408u32);
    emu.sltru_no_count(11usize, 0usize, 11usize, 2139412u32);
    emu.sltru_no_count(16usize, 0usize, 12usize, 2139416u32);
    emu.anr_no_count(17usize, 11usize, 16usize, 2139420u32);
    emu.adi_no_count(15usize, 15usize, 1u32, 2139424u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2139428u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2139432u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2139384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4f8));
    } else {
        emu.pc = 2139436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a52c));
    }
}
#[inline(always)]
pub fn block_0x0020a52c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 13usize, 3u32, 2139440u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2139636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5f4));
    } else {
        emu.pc = 2139444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a534));
    }
}
#[inline(always)]
pub fn block_0x0020a534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 32u32, 2139448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2140020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a774));
    } else {
        emu.pc = 2139452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a53c));
    }
}
#[inline(always)]
pub fn block_0x0020a53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 3u32, 2139456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2139756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a66c));
    } else {
        emu.pc = 2139460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a544));
    }
}
#[inline(always)]
pub fn block_0x0020a544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2139464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2139884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6ec));
    } else {
        emu.pc = 2139468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a54c));
    }
}
#[inline(always)]
pub fn block_0x0020a54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2139472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a774));
    } else {
        emu.pc = 2139476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a554));
    }
}
#[inline]
pub fn block_0x0020a554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2139480u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2139484u32);
    emu.sri_no_count(11usize, 15usize, 8u32, 2139488u32);
    emu.sb_no_count(11usize, 13usize, 1u32, 2139492u32);
    emu.sri_no_count(16usize, 15usize, 16u32, 2139496u32);
    emu.adi_no_count(11usize, 13usize, 3u32, 2139500u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2139504u32);
    emu.adi_no_count(12usize, 12usize, 4294967293u32, 2139508u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2139512u32);
    emu.adi_no_count(14usize, 0usize, 16u32, 2139516u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2139516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a57c));
}
#[inline]
pub fn block_0x0020a57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2139520u32)?;
    emu.sri_no_count(15usize, 15usize, 24u32, 2139524u32);
    emu.sli_no_count(17usize, 16usize, 8u32, 2139528u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2139532u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2139536u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2139540u32)?;
    emu.sri_no_count(15usize, 16usize, 24u32, 2139544u32);
    emu.sli_no_count(16usize, 5usize, 8u32, 2139548u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2139552u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2139556u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2139560u32)?;
    emu.sri_no_count(16usize, 5usize, 24u32, 2139564u32);
    emu.sli_no_count(5usize, 17usize, 8u32, 2139568u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2139572u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2139576u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2139580u32)?;
    emu.sri_no_count(16usize, 17usize, 24u32, 2139584u32);
    emu.sli_no_count(17usize, 15usize, 8u32, 2139588u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2139592u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2139596u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2139600u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2139604u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2139608u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2139516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a57c));
    } else {
        emu.pc = 2139612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5dc));
    }
}
#[inline(always)]
pub fn block_0x0020a5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967283u32, 2139616u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a770));
}
#[inline(always)]
pub fn block_0x0020a5e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2139624u32);
    emu.adi_no_count(14usize, 11usize, 0u32, 2139628u32);
    emu.ani_no_count(11usize, 13usize, 3u32, 2139632u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2139444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a534));
    } else {
        emu.pc = 2139636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5f4));
    }
}
#[inline(always)]
pub fn block_0x0020a5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2139640u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2139696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a630));
    } else {
        emu.pc = 2139644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5fc));
    }
}
#[inline(always)]
pub fn block_0x0020a5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 15u32, 2139648u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a600));
}
#[inline]
pub fn block_0x0020a600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2139652u32)?;
    emu.lw_no_count(16usize, 14usize, 4u32, 2139656u32)?;
    emu.lw_no_count(17usize, 14usize, 8u32, 2139660u32)?;
    emu.lw_no_count(5usize, 14usize, 12u32, 2139664u32)?;
    emu.sw_no_count(15usize, 13usize, 0u32, 2139668u32)?;
    emu.sw_no_count(16usize, 13usize, 4u32, 2139672u32)?;
    emu.sw_no_count(17usize, 13usize, 8u32, 2139676u32)?;
    emu.sw_no_count(5usize, 13usize, 12u32, 2139680u32)?;
    emu.adi_no_count(14usize, 14usize, 16u32, 2139684u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2139688u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2139692u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2139648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a600));
    } else {
        emu.pc = 2139696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a630));
    }
}
#[inline(always)]
pub fn block_0x0020a630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2139700u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2139728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a650));
    } else {
        emu.pc = 2139704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a638));
    }
}
#[inline(always)]
pub fn block_0x0020a638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2139708u32)?;
    emu.lw_no_count(15usize, 14usize, 4u32, 2139712u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2139716u32)?;
    emu.sw_no_count(15usize, 13usize, 4u32, 2139720u32)?;
    emu.adi_no_count(13usize, 13usize, 8u32, 2139724u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2139728u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2139728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a650));
}
#[inline(always)]
pub fn block_0x0020a650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2139732u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2140088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7b8));
    } else {
        emu.pc = 2139736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a658));
    }
}
#[inline(always)]
pub fn block_0x0020a658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 14usize, 0u32, 2139740u32)?;
    emu.sw_no_count(11usize, 13usize, 0u32, 2139744u32)?;
    emu.adi_no_count(13usize, 13usize, 4u32, 2139748u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2139752u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2139756u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140088u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7b8));
}
#[inline(always)]
pub fn block_0x0020a66c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2139760u32)?;
    emu.adi_no_count(11usize, 13usize, 1u32, 2139764u32);
    emu.sb_no_count(15usize, 13usize, 0u32, 2139768u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2139772u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2139776u32);
    emu.adi_no_count(14usize, 0usize, 18u32, 2139780u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2139780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a684));
}
#[inline]
pub fn block_0x0020a684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2139784u32)?;
    emu.sri_no_count(15usize, 15usize, 8u32, 2139788u32);
    emu.sli_no_count(17usize, 16usize, 24u32, 2139792u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2139796u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2139800u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2139804u32)?;
    emu.sri_no_count(15usize, 16usize, 8u32, 2139808u32);
    emu.sli_no_count(16usize, 5usize, 24u32, 2139812u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2139816u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2139820u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2139824u32)?;
    emu.sri_no_count(16usize, 5usize, 8u32, 2139828u32);
    emu.sli_no_count(5usize, 17usize, 24u32, 2139832u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2139836u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2139840u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2139844u32)?;
    emu.sri_no_count(16usize, 17usize, 8u32, 2139848u32);
    emu.sli_no_count(17usize, 15usize, 24u32, 2139852u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2139856u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2139860u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2139864u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2139868u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2139872u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2139780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a684));
    } else {
        emu.pc = 2139876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6e4));
    }
}
#[inline(always)]
pub fn block_0x0020a6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967281u32, 2139880u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a770));
}
#[inline(always)]
pub fn block_0x0020a6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2139888u32)?;
    emu.sb_no_count(15usize, 13usize, 0u32, 2139892u32);
    emu.sri_no_count(16usize, 15usize, 8u32, 2139896u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2139900u32);
    emu.sb_no_count(16usize, 13usize, 1u32, 2139904u32);
    emu.adi_no_count(12usize, 12usize, 4294967294u32, 2139908u32);
    emu.adi_no_count(13usize, 14usize, 16u32, 2139912u32);
    emu.adi_no_count(14usize, 0usize, 17u32, 2139916u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2139916u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a70c));
}
#[inline]
pub fn block_0x0020a70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 13usize, 4294967284u32, 2139920u32)?;
    emu.sri_no_count(15usize, 15usize, 16u32, 2139924u32);
    emu.sli_no_count(17usize, 16usize, 16u32, 2139928u32);
    emu.lw_no_count(5usize, 13usize, 4294967288u32, 2139932u32)?;
    emu.orr_no_count(15usize, 17usize, 15usize, 2139936u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2139940u32)?;
    emu.sri_no_count(15usize, 16usize, 16u32, 2139944u32);
    emu.sli_no_count(16usize, 5usize, 16u32, 2139948u32);
    emu.lw_no_count(17usize, 13usize, 4294967292u32, 2139952u32)?;
    emu.orr_no_count(15usize, 16usize, 15usize, 2139956u32);
    emu.sw_no_count(15usize, 11usize, 4u32, 2139960u32)?;
    emu.sri_no_count(16usize, 5usize, 16u32, 2139964u32);
    emu.sli_no_count(5usize, 17usize, 16u32, 2139968u32);
    emu.lw_no_count(15usize, 13usize, 0u32, 2139972u32)?;
    emu.orr_no_count(16usize, 5usize, 16usize, 2139976u32);
    emu.sw_no_count(16usize, 11usize, 8u32, 2139980u32)?;
    emu.sri_no_count(16usize, 17usize, 16u32, 2139984u32);
    emu.sli_no_count(17usize, 15usize, 16u32, 2139988u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2139992u32);
    emu.sw_no_count(16usize, 11usize, 12u32, 2139996u32)?;
    emu.adi_no_count(11usize, 11usize, 16u32, 2140000u32);
    emu.adi_no_count(12usize, 12usize, 4294967280u32, 2140004u32);
    emu.adi_no_count(13usize, 13usize, 16u32, 2140008u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2139916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a70c));
    } else {
        emu.pc = 2140012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a76c));
    }
}
#[inline(always)]
pub fn block_0x0020a76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 13usize, 4294967282u32, 2140016u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a770));
}
#[inline(always)]
pub fn block_0x0020a770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 0u32, 2140020u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a774));
}
#[inline(always)]
pub fn block_0x0020a774(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 16u32, 2140024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7fc));
    } else {
        emu.pc = 2140028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a77c));
    }
}
#[inline(always)]
pub fn block_0x0020a77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 8u32, 2140032u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a890));
    } else {
        emu.pc = 2140036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a784));
    }
}
#[inline(always)]
pub fn block_0x0020a784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 4u32, 2140040u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2140088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7b8));
    } else {
        emu.pc = 2140044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a78c));
    }
}
#[inline]
pub fn block_0x0020a78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2140048u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2140052u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2140056u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2140060u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2140064u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2140068u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2140072u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2140076u32);
    emu.adi_no_count(15usize, 13usize, 4u32, 2140080u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2140084u32);
    emu.adi_no_count(13usize, 15usize, 0u32, 2140088u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2140088u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7b8));
}
#[inline(always)]
pub fn block_0x0020a7b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 2u32, 2140092u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7cc));
    } else {
        emu.pc = 2140096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7c0));
    }
}
#[inline(always)]
pub fn block_0x0020a7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 12usize, 1u32, 2140100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f0));
    } else {
        emu.pc = 2140104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7c8));
    }
}
#[inline(always)]
pub fn block_0x0020a7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140108u32;
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
pub fn block_0x0020a7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2140112u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2140116u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2140120u32);
    emu.adi_no_count(14usize, 14usize, 2u32, 2140124u32);
    emu.adi_no_count(11usize, 13usize, 2u32, 2140128u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2140132u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2140136u32);
    emu.ani_no_count(11usize, 12usize, 1u32, 2140140u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2140104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7c8));
    } else {
        emu.pc = 2140144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7f0));
    }
}
#[inline(always)]
pub fn block_0x0020a7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2140148u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2140152u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140156u32;
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
pub fn block_0x0020a7fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2140160u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2140164u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2140168u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2140172u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2140176u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2140180u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2140184u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2140188u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2140192u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2140196u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2140200u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2140204u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2140208u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2140212u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2140216u32);
    emu.lb_no_count(11usize, 14usize, 8u32, 2140220u32);
    emu.lb_no_count(16usize, 14usize, 9u32, 2140224u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2140228u32);
    emu.lb_no_count(15usize, 14usize, 10u32, 2140232u32);
    emu.sb_no_count(11usize, 13usize, 8u32, 2140236u32);
    emu.sb_no_count(16usize, 13usize, 9u32, 2140240u32);
    emu.lb_no_count(11usize, 14usize, 11u32, 2140244u32);
    emu.sb_no_count(15usize, 13usize, 10u32, 2140248u32);
    emu.lb_no_count(15usize, 14usize, 12u32, 2140252u32);
    emu.lb_no_count(16usize, 14usize, 13u32, 2140256u32);
    emu.sb_no_count(11usize, 13usize, 11u32, 2140260u32);
    emu.lb_no_count(11usize, 14usize, 14u32, 2140264u32);
    emu.sb_no_count(15usize, 13usize, 12u32, 2140268u32);
    emu.sb_no_count(16usize, 13usize, 13u32, 2140272u32);
    emu.lb_no_count(15usize, 14usize, 15u32, 2140276u32);
    emu.sb_no_count(11usize, 13usize, 14u32, 2140280u32);
    emu.adi_no_count(14usize, 14usize, 16u32, 2140284u32);
    emu.adi_no_count(11usize, 13usize, 16u32, 2140288u32);
    emu.sb_no_count(15usize, 13usize, 15u32, 2140292u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2140296u32);
    emu.ani_no_count(11usize, 12usize, 8u32, 2140300u32);
    emu.add_memory_rw_events(36usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2140036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a784));
    } else {
        emu.pc = 2140304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a890));
    }
}
#[inline]
pub fn block_0x0020a890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 14usize, 0u32, 2140308u32);
    emu.lb_no_count(15usize, 14usize, 1u32, 2140312u32);
    emu.lb_no_count(16usize, 14usize, 2u32, 2140316u32);
    emu.sb_no_count(11usize, 13usize, 0u32, 2140320u32);
    emu.sb_no_count(15usize, 13usize, 1u32, 2140324u32);
    emu.lb_no_count(11usize, 14usize, 3u32, 2140328u32);
    emu.sb_no_count(16usize, 13usize, 2u32, 2140332u32);
    emu.lb_no_count(15usize, 14usize, 4u32, 2140336u32);
    emu.lb_no_count(16usize, 14usize, 5u32, 2140340u32);
    emu.sb_no_count(11usize, 13usize, 3u32, 2140344u32);
    emu.lb_no_count(11usize, 14usize, 6u32, 2140348u32);
    emu.sb_no_count(15usize, 13usize, 4u32, 2140352u32);
    emu.sb_no_count(16usize, 13usize, 5u32, 2140356u32);
    emu.lb_no_count(15usize, 14usize, 7u32, 2140360u32);
    emu.sb_no_count(11usize, 13usize, 6u32, 2140364u32);
    emu.adi_no_count(14usize, 14usize, 8u32, 2140368u32);
    emu.adi_no_count(11usize, 13usize, 8u32, 2140372u32);
    emu.sb_no_count(15usize, 13usize, 7u32, 2140376u32);
    emu.adi_no_count(13usize, 11usize, 0u32, 2140380u32);
    emu.ani_no_count(11usize, 12usize, 4u32, 2140384u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a78c));
    } else {
        emu.pc = 2140388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e4));
    }
}
#[inline(always)]
pub fn block_0x0020a8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2140392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140088u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7b8));
}
#[inline(always)]
pub fn block_0x0020a8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(3usize, 2140392u32, 4292894720u32, 2140396u32);
    emu.adi_no_count(3usize, 3usize, 4294967064u32, 2140400u32);
    emu.apc_no_count(2usize, 2140400u32, 122880u32, 2140404u32);
    emu.adi_no_count(2usize, 2usize, 1716u32, 2140408u32);
    emu.lw_no_count(2usize, 2usize, 0u32, 2140412u32)?;
    emu.apc_no_count(1usize, 2140412u32, 0u32, 2140416u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140420u32;
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
#[inline(never)]
pub fn block_0x0020a904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140424u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140428u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140432u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140436u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2140440u32)?;
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2140444u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 0usize, 1u32, 2140448u32);
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140452u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140456u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140460u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140464u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2140468u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2140472u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 8usize, 56u32, 2140476u32);
    emu.adi_no_count(10usize, 10usize, 1639u32, 2140480u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2140484u32);
    emu.sw_no_count(9usize, 18usize, 40u32, 2140488u32)?;
    emu.sw_no_count(0usize, 18usize, 44u32, 2140492u32)?;
    emu.sw_no_count(10usize, 18usize, 48u32, 2140496u32)?;
    emu.sw_no_count(11usize, 18usize, 52u32, 2140500u32)?;
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140504u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 12usize, 882u32, 2140508u32);
    emu.adi_no_count(12usize, 13usize, 1338u32, 2140512u32);
    emu.adi_no_count(13usize, 14usize, 639u32, 2140516u32);
    emu.adi_no_count(14usize, 15usize, 4294965388u32, 2140520u32);
    emu.sw_no_count(11usize, 18usize, 56u32, 2140524u32)?;
    emu.sw_no_count(12usize, 18usize, 60u32, 2140528u32)?;
    emu.sw_no_count(13usize, 18usize, 64u32, 2140532u32)?;
    emu.sw_no_count(14usize, 18usize, 68u32, 2140536u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140540u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965675u32, 2140544u32);
    emu.adi_no_count(11usize, 11usize, 4294966553u32, 2140548u32);
    emu.sw_no_count(10usize, 18usize, 72u32, 2140552u32)?;
    emu.sw_no_count(11usize, 18usize, 76u32, 2140556u32)?;
    emu.adi_no_count(10usize, 18usize, 80u32, 2140560u32);
    emu.adi_no_count(12usize, 0usize, 73u32, 2140564u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2140568u32);
    emu.apc_no_count(1usize, 2140568u32, 0u32, 2140572u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 56u32, 2140580u32)?;
    emu.sw_no_count(0usize, 8usize, 60u32, 2140584u32)?;
    emu.sw_no_count(0usize, 18usize, 8u32, 2140588u32)?;
    emu.sw_no_count(0usize, 18usize, 12u32, 2140592u32)?;
    emu.sw_no_count(0usize, 18usize, 16u32, 2140596u32)?;
    emu.sw_no_count(0usize, 18usize, 20u32, 2140600u32)?;
    emu.sw_no_count(0usize, 18usize, 24u32, 2140604u32)?;
    emu.sw_no_count(0usize, 18usize, 28u32, 2140608u32)?;
    emu.sw_no_count(0usize, 18usize, 32u32, 2140612u32)?;
    emu.apc_no_count(1usize, 2140612u32, 4294959104u32, 2140616u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140624u32);
    emu.apc_no_count(1usize, 2140624u32, 4294963200u32, 2140628u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140632u32;
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
pub fn block_0x0020a9d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140636u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 48u32, 2140640u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2140652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9ec));
    } else {
        emu.pc = 2140644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9e4));
    }
}
#[inline(always)]
pub fn block_0x0020a9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140648u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 284u32, 2140652u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2140652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9ec));
}
#[inline]
pub fn block_0x0020a9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 11usize, 4294967295u32, 2140656u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2140660u32);
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2140664u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2140668u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2140672u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2140676u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2140680u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2140684u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2140712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa28));
    } else {
        emu.pc = 2140688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa10));
    }
}
#[inline(always)]
pub fn block_0x0020aa10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2013265920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140692u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2140696u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2140712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa28));
    } else {
        emu.pc = 2140700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa1c));
    }
}
#[inline(always)]
pub fn block_0x0020aa1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2293760u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140704u32;
    emu.update_insn_clock();
    emu.sw_no_count(12usize, 11usize, 48u32, 2140708u32)?;
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140712u32;
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
pub fn block_0x0020aa28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2140716u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140720u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967180u32, 2140724u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140728u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2140732u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140736u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2140740u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2140744u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2140748u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2140752u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140756u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967188u32, 2140760u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2140764u32);
    emu.apc_no_count(1usize, 2140764u32, 77824u32, 2140768u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140772u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140776u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967208u32, 2140780u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2140784u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140788u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140792u32);
    emu.apc_no_count(6usize, 2140792u32, 98304u32, 2140796u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2140800u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020aa80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140804u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140808u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140812u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140816u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2140820u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2140824u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2140828u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2140832u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2140836u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2140840u32);
    emu.apc_no_count(1usize, 2140840u32, 4294963200u32, 2140844u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140848u32;
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
pub fn block_0x0020aab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140852u32);
    emu.sb_no_count(10usize, 9usize, 0u32, 2140856u32);
    emu.sw_no_count(8usize, 9usize, 4u32, 2140860u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2140864u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140868u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2140872u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140876u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140880u32;
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
pub fn block_0x0020aad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2140884u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2140888u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2140892u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2140896u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2140900u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2140904u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140908u32);
    emu.apc_no_count(1usize, 2140908u32, 0u32, 2140912u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020aaf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2140920u32);
    emu.ani_no_count(10usize, 10usize, 3u32, 2140924u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2140928u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2140932u32);
    emu.adr_no_count(10usize, 10usize, 9usize, 2140936u32);
    emu.ani_no_count(18usize, 10usize, 4294967292u32, 2140940u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2140944u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2140948u32);
    emu.apc_no_count(1usize, 2140948u32, 73728u32, 2140952u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ab1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab70));
    } else {
        emu.pc = 2140960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab20));
    }
}
#[inline(always)]
pub fn block_0x0020ab20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2140960u32, 4294963200u32, 2140964u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ab28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2140972u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2140976u32);
    emu.apc_no_count(1usize, 2140976u32, 4294930432u32, 2140980u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ab38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2140988u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2140992u32);
    emu.apc_no_count(1usize, 2140992u32, 0u32, 2140996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ab48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 0u32, 2141004u32)?;
    emu.sw_no_count(19usize, 8usize, 4u32, 2141008u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2141012u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2141016u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2141020u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2141024u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2141028u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2141032u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2141036u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141040u32;
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
pub fn block_0x0020ab70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2141044u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1035u32, 2141048u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2141052u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967220u32, 2141056u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2141060u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967236u32, 2141064u32);
    emu.adi_no_count(11usize, 0usize, 16u32, 2141068u32);
    emu.adi_no_count(12usize, 2usize, 11u32, 2141072u32);
    emu.apc_no_count(1usize, 2141072u32, 90112u32, 2141076u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ab98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2141084u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2141088u32)?;
    emu.sw_no_count(10usize, 2usize, 0u32, 2141092u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2141096u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2141124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abc4));
    } else {
        emu.pc = 2141100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abac));
    }
}
#[inline(always)]
pub fn block_0x0020abac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2141104u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2141108u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2141112u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2141116u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2141120u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141124u32;
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
pub fn block_0x0020abc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2141128u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1176u32, 2141136u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141140u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 572u32, 2141144u32);
    let a = 0u32.wrapping_add(2215936u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2141148u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2141152u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2141156u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4u32, 2141160u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2141164u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2141168u32)?;
    emu.adi_no_count(16usize, 2usize, 44u32, 2141172u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2141176u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2141180u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2141184u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2141188u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2141192u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2141196u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2141200u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2141204u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2141208u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2141212u32);
    emu.adi_no_count(11usize, 2usize, 20u32, 2141216u32);
    emu.apc_no_count(1usize, 2141216u32, 69632u32, 2141220u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141224u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2141224u32, 4294963200u32, 2141228u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2141236u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2141240u32);
    emu.apc_no_count(1usize, 2141240u32, 4294930432u32, 2141244u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac74));
    } else {
        emu.pc = 2141252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac44));
    }
}
#[inline]
pub fn block_0x0020ac44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2141256u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2141260u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2141264u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2141268u32)?;
    emu.sw_no_count(10usize, 12usize, 0u32, 2141272u32)?;
    emu.sw_no_count(11usize, 12usize, 4u32, 2141276u32)?;
    emu.sw_no_count(13usize, 12usize, 8u32, 2141280u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2141284u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2141288u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2141292u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2141296u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141300u32;
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
pub fn block_0x0020ac74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2141304u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2141308u32);
    emu.apc_no_count(1usize, 2141308u32, 69632u32, 2141312u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ac84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141320u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141324u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2141328u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2141332u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2141336u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2141340u32)?;
    emu.apc_no_count(1usize, 2141340u32, 4294963200u32, 2141344u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966448u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2141352u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2141356u32);
    emu.apc_no_count(1usize, 2141356u32, 4294930432u32, 2141360u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141364u32;
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
pub fn block_0x0020acb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acdc));
    } else {
        emu.pc = 2141368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acb8));
    }
}
#[inline]
pub fn block_0x0020acb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141372u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 10usize, 0u32, 2141376u32)?;
    emu.sw_no_count(8usize, 10usize, 4u32, 2141380u32)?;
    emu.sw_no_count(9usize, 10usize, 8u32, 2141384u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2141388u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141392u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2141396u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141400u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141404u32;
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
pub fn block_0x0020acdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2141408u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2141412u32);
    emu.apc_no_count(1usize, 2141412u32, 69632u32, 2141416u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141420u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966948u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020acec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141424u32);
    emu.sb_no_count(10usize, 2usize, 15u32, 2141428u32);
    emu.lbu_no_count(10usize, 2usize, 15u32, 2141432u32);
    emu.adi_no_count(2usize, 2usize, 16u32, 2141436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141440u32;
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
pub fn block_0x0020ad00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2141444u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2141448u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2141452u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2141456u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2141460u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2141464u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2141576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad88));
    } else {
        emu.pc = 2141468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad1c));
    }
}
#[inline(always)]
pub fn block_0x0020ad1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2141472u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2141476u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2141480u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2141532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad5c));
    } else {
        emu.pc = 2141484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad2c));
    }
}
#[inline(always)]
pub fn block_0x0020ad2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 13usize, 8u32, 2141488u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2141532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad5c));
    } else {
        emu.pc = 2141492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad34));
    }
}
#[inline(always)]
pub fn block_0x0020ad34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 13usize, 0u32, 2141496u32)?;
    emu.adi_no_count(12usize, 18usize, 0u32, 2141500u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2141504u32);
    emu.apc_no_count(1usize, 2141504u32, 4294930432u32, 2141508u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ad48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad7c));
    } else {
        emu.pc = 2141516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad4c));
    }
}
#[inline(always)]
pub fn block_0x0020ad4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2141520u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2141524u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2141528u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2141532u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad90));
}
#[inline(always)]
pub fn block_0x0020ad5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adac));
    } else {
        emu.pc = 2141536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad60));
    }
}
#[inline(always)]
pub fn block_0x0020ad60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2141536u32, 4294963200u32, 2141540u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ad68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2141548u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2141552u32);
    emu.apc_no_count(1usize, 2141552u32, 4294930432u32, 2141556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ad78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad4c));
    } else {
        emu.pc = 2141564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad7c));
    }
}
#[inline(always)]
pub fn block_0x0020ad7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 8usize, 4u32, 2141568u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2141572u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2141576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141580u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad8c));
}
#[inline(always)]
pub fn block_0x0020ad88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2141580u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad8c));
}
#[inline(always)]
pub fn block_0x0020ad8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2141584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad90));
}
#[inline(always)]
pub fn block_0x0020ad90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2141588u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2141592u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2141596u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2141600u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2141604u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2141608u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141612u32;
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
pub fn block_0x0020adac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2141616u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2141516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad4c));
    } else {
        emu.pc = 2141620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adb4));
    }
}
#[inline(always)]
pub fn block_0x0020adb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2141624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141564u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad7c));
}
#[inline]
pub fn block_0x0020adb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2141628u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2141632u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2141636u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2141640u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2141644u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2141648u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2141652u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2141656u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2141660u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2141664u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2141672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ade8));
    } else {
        emu.pc = 2141668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ade4));
    }
}
#[inline(always)]
pub fn block_0x0020ade4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2141672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ade8));
}
#[inline(always)]
pub fn block_0x0020ade8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 13usize, 29u32, 2141676u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2141680u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2141712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae10));
    } else {
        emu.pc = 2141684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adf4));
    }
}
#[inline(always)]
pub fn block_0x0020adf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 18usize, 2u32, 2141688u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2141692u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2141696u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2141820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae7c));
    } else {
        emu.pc = 2141700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae04));
    }
}
#[inline(always)]
pub fn block_0x0020ae04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2141724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae1c));
    } else {
        emu.pc = 2141704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae08));
    }
}
#[inline(always)]
pub fn block_0x0020ae08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2141708u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2141712u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae30));
}
#[inline(always)]
pub fn block_0x0020ae10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2141716u32);
    emu.apc_no_count(1usize, 2141716u32, 69632u32, 2141720u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ae1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2141728u32)?;
    emu.sli_no_count(13usize, 13usize, 2u32, 2141732u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2141736u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2141740u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2141744u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2141744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae30));
}
#[inline(always)]
pub fn block_0x0020ae30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2141748u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2141752u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2141756u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2141760u32);
    emu.apc_no_count(1usize, 2141760u32, 0u32, 2141764u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ae48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2141772u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2141812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae74));
    } else {
        emu.pc = 2141776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae50));
    }
}
#[inline]
pub fn block_0x0020ae50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2141780u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2141784u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2141788u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2141792u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2141796u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2141800u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2141804u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2141808u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141812u32;
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
pub fn block_0x0020ae74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2141816u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2141820u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2141820u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae7c));
}
#[inline(always)]
pub fn block_0x0020ae7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2141824u32);
    emu.apc_no_count(1usize, 2141824u32, 69632u32, 2141828u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ae88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2141836u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2141840u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2141844u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2141848u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2141852u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2141856u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2141860u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2141864u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2141868u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2141900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aecc));
    } else {
        emu.pc = 2141872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aeb0));
    }
}
#[inline(always)]
pub fn block_0x0020aeb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2141876u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aeb4));
}
#[inline(always)]
pub fn block_0x0020aeb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141880u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 88u32, 2141884u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2141888u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2141892u32);
    emu.apc_no_count(1usize, 2141892u32, 69632u32, 2141896u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2141904u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2141908u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2141960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af08));
    } else {
        emu.pc = 2141912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aed8));
    }
}
#[inline(always)]
pub fn block_0x0020aed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 12usize, 0u32, 2141916u32);
    emu.apc_no_count(1usize, 2141916u32, 4294963200u32, 2141920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141924u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2141928u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2141932u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2141936u32);
    emu.apc_no_count(1usize, 2141936u32, 4294930432u32, 2141940u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141944u32;
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
pub fn block_0x0020aef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aeb4));
    } else {
        emu.pc = 2141948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aefc));
    }
}
#[inline(always)]
pub fn block_0x0020aefc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2141952u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2141956u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2141960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2141964u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af0c));
}
#[inline(always)]
pub fn block_0x0020af08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2141964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af0c));
}
#[inline(always)]
pub fn block_0x0020af0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2141968u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2141972u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2141976u32);
    emu.apc_no_count(1usize, 2141976u32, 4294963200u32, 2141980u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141984u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
