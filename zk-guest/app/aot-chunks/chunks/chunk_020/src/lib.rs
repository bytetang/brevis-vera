pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2166740u32;
pub const PC_MAX: u32 = 2168852u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x00210fd4,
        block_0x00210fd8,
        block_0x00210fe4,
        block_0x00210fec,
        block_0x00210ff8,
        block_0x00211000,
        block_0x00211038,
        block_0x00211040,
        block_0x0021106c,
        block_0x00211078,
        block_0x00211080,
        block_0x0021108c,
        block_0x002110a0,
        block_0x002110bc,
        block_0x002110c8,
        block_0x002110d0,
        block_0x002110dc,
        block_0x002110f8,
        block_0x00211114,
        block_0x00211130,
        block_0x00211168,
        block_0x00211174,
        block_0x00211190,
        block_0x002111c0,
        block_0x002111c8,
        block_0x002111d0,
        block_0x002111d8,
        block_0x002111ec,
        block_0x00211214,
        block_0x0021121c,
        block_0x00211224,
        block_0x0021123c,
        block_0x00211244,
        block_0x00211250,
        block_0x00211270,
        block_0x00211274,
        block_0x002112a4,
        block_0x002112ac,
        block_0x002112c4,
        block_0x002112c8,
        block_0x002112d8,
        block_0x002112dc,
        block_0x002112e4,
        block_0x002112f4,
        block_0x002112f8,
        block_0x00211308,
        block_0x00211310,
        block_0x00211330,
        block_0x00211338,
        block_0x00211354,
        block_0x0021136c,
        block_0x0021138c,
        block_0x00211394,
        block_0x002113a4,
        block_0x002113ac,
        block_0x002113b4,
        block_0x002113c0,
        block_0x002113dc,
        block_0x002113ec,
        block_0x002113fc,
        block_0x00211438,
        block_0x00211450,
        block_0x0021146c,
        block_0x00211488,
        block_0x002114c4,
        block_0x002114e4,
        block_0x00211514,
        block_0x0021151c,
        block_0x00211524,
        block_0x0021152c,
        block_0x00211540,
        block_0x00211568,
        block_0x00211570,
        block_0x00211578,
        block_0x00211590,
        block_0x00211598,
        block_0x002115a4,
        block_0x002115c4,
        block_0x002115c8,
        block_0x002115f8,
        block_0x00211600,
        block_0x00211618,
        block_0x0021161c,
        block_0x0021162c,
        block_0x00211630,
        block_0x00211638,
        block_0x00211648,
        block_0x0021164c,
        block_0x0021165c,
        block_0x00211664,
        block_0x00211688,
        block_0x00211690,
        block_0x00211698,
        block_0x0021169c,
        block_0x002116c4,
        block_0x002116cc,
        block_0x002116ec,
        block_0x002116f0,
        block_0x0021170c,
        block_0x00211714,
        block_0x00211730,
        block_0x00211738,
        block_0x00211748,
        block_0x00211750,
        block_0x00211758,
        block_0x00211764,
        block_0x00211780,
        block_0x00211798,
        block_0x002117a8,
        block_0x002117b8,
        block_0x002117f8,
        block_0x00211814,
    ];
    const IDX: [u16; 529usize] = [
        1u16, 2u16, 0u16, 0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16,
        0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 16u16, 0u16,
        0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 25u16, 0u16, 26u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 30u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16,
        33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16, 41u16,
        42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 55u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        67u16, 0u16, 68u16, 0u16, 69u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16,
        74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 84u16, 85u16, 0u16, 86u16, 0u16, 0u16,
        0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16,
        0u16, 103u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 112u16,
    ];
    if pc < 2166740u32 || pc > 2168852u32 {
        return None;
    }
    let word_offset = ((pc - 2166740u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00210fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110dc));
    } else {
        emu.pc = 2166744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fd8));
    }
}
#[inline(always)]
pub fn block_0x00210fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 0u32, 2166748u32);
    emu.adi_no_count(17usize, 0usize, 48u32, 2166752u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2167032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110f8));
    } else {
        emu.pc = 2166756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fe4));
    }
}
#[inline(always)]
pub fn block_0x00210fe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 3u32, 2166760u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2167060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211114));
    } else {
        emu.pc = 2166764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fec));
    }
}
#[inline(always)]
pub fn block_0x00210fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 2u32, 2166768u32);
    emu.sh_no_count(16usize, 14usize, 0u32, 2166772u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(0usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2166848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211040));
    } else {
        emu.pc = 2166776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ff8));
    }
}
#[inline(always)]
pub fn block_0x00210ff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 14usize, 4u32, 2166780u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2166924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021108c));
    } else {
        emu.pc = 2166784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211000));
    }
}
#[inline]
pub fn block_0x00211000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2166788u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2166792u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1816u32, 2166796u32);
    emu.adi_no_count(17usize, 0usize, 1u32, 2166800u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2166804u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2166808u32);
    emu.sw_no_count(12usize, 14usize, 8u32, 2166812u32)?;
    emu.sh_no_count(15usize, 14usize, 12u32, 2166816u32)?;
    emu.sw_no_count(16usize, 14usize, 16u32, 2166820u32)?;
    emu.sw_no_count(17usize, 14usize, 20u32, 2166824u32)?;
    emu.sh_no_count(15usize, 14usize, 24u32, 2166828u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2166832u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2166836u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2166912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211080));
    } else {
        emu.pc = 2166840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211038));
    }
}
#[inline(always)]
pub fn block_0x00211038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2166844u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2166848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110bc));
}
#[inline]
pub fn block_0x00211040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 0usize, 12usize, 2166852u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2166856u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1817u32, 2166860u32);
    emu.sh_no_count(16usize, 14usize, 24u32, 2166864u32)?;
    emu.sw_no_count(10usize, 14usize, 28u32, 2166868u32)?;
    emu.sw_no_count(11usize, 14usize, 32u32, 2166872u32)?;
    emu.sw_no_count(17usize, 14usize, 4u32, 2166876u32)?;
    emu.sw_no_count(16usize, 14usize, 8u32, 2166880u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2166884u32)?;
    emu.sw_no_count(15usize, 14usize, 16u32, 2166888u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2166912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211080));
    } else {
        emu.pc = 2166892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021106c));
    }
}
#[inline(always)]
pub fn block_0x0021106c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(13usize, 13usize, 11usize, 2166896u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2166900u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2166904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211078));
    }
}
#[inline(always)]
pub fn block_0x00211078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 12usize, 2166908u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2166912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166972u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110bc));
}
#[inline(always)]
pub fn block_0x00211080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2166916u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2166920u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166924u32;
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
pub fn block_0x0021108c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 12usize, 11usize, 2166928u32);
    emu.sw_no_count(11usize, 14usize, 8u32, 2166932u32)?;
    emu.sh_no_count(0usize, 14usize, 12u32, 2166936u32)?;
    emu.sw_no_count(12usize, 14usize, 16u32, 2166940u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2166992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110d0));
    } else {
        emu.pc = 2166944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110a0));
    }
}
#[inline(always)]
pub fn block_0x002110a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2166948u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166952u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1816u32, 2166956u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166960u32);
    emu.sh_no_count(10usize, 14usize, 24u32, 2166964u32)?;
    emu.sw_no_count(11usize, 14usize, 28u32, 2166968u32)?;
    emu.sw_no_count(12usize, 14usize, 32u32, 2166972u32)?;
    emu.add_memory_rw_events(7usize);
    emu.pc = 2166972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110bc));
}
#[inline(always)]
pub fn block_0x002110bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(0usize, 14usize, 36u32, 2166976u32)?;
    emu.sw_no_count(13usize, 14usize, 40u32, 2166980u32)?;
    emu.adi_no_count(11usize, 0usize, 4u32, 2166984u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2166984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110c8));
}
#[inline(always)]
pub fn block_0x002110c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2166988u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166992u32;
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
pub fn block_0x002110d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2166996u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2167000u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167004u32;
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
pub fn block_0x002110dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167008u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1664u32, 2167012u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2167016u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1700u32, 2167020u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2167024u32);
    emu.apc_no_count(1usize, 2167024u32, 4294963200u32, 2167028u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965860u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002110f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167036u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1716u32, 2167040u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2167044u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1748u32, 2167048u32);
    emu.adi_no_count(11usize, 0usize, 31u32, 2167052u32);
    emu.apc_no_count(1usize, 2167052u32, 4294963200u32, 2167056u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167064u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1764u32, 2167068u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2167072u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1800u32, 2167076u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2167080u32);
    emu.apc_no_count(1usize, 2167080u32, 4294963200u32, 2167084u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2167092u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2167096u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2167100u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2167104u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2167108u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2167112u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2167116u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2167120u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2167124u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2167128u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2167132u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2167136u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2167140u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2167888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211450));
    } else {
        emu.pc = 2167144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211168));
    }
}
#[inline(always)]
pub fn block_0x00211168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2167148u32);
    emu.adi_no_count(10usize, 0usize, 16u32, 2167152u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2167916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021146c));
    } else {
        emu.pc = 2167156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211174));
    }
}
#[inline(always)]
pub fn block_0x00211174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2167160u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2167164u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167168u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2167172u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2167176u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2167180u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2167276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111ec));
    } else {
        emu.pc = 2167184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211190));
    }
}
#[inline]
pub fn block_0x00211190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2167188u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2167192u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2167196u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2167200u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2167204u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2167208u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2167212u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2167216u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2167220u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2167224u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2167228u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2167316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211214));
    } else {
        emu.pc = 2167232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c0));
    }
}
#[inline(always)]
pub fn block_0x002111c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2167236u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2167324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021121c));
    } else {
        emu.pc = 2167240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c8));
    }
}
#[inline(always)]
pub fn block_0x002111c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2167244u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2167332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211224));
    } else {
        emu.pc = 2167248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111d0));
    }
}
#[inline(always)]
pub fn block_0x002111d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2167252u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2167408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211270));
    } else {
        emu.pc = 2167256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111d8));
    }
}
#[inline(always)]
pub fn block_0x002111d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2167260u32);
    emu.adi_no_count(17usize, 17usize, 4294966221u32, 2167264u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2167268u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2167272u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2167276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211274));
}
#[inline]
pub fn block_0x002111ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2167280u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2167284u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2167288u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2167292u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2167296u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2167300u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2167304u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2167308u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2167312u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2167232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c0));
    } else {
        emu.pc = 2167316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211214));
    }
}
#[inline(always)]
pub fn block_0x00211214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2167320u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167324u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211274));
}
#[inline(always)]
pub fn block_0x0021121c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2167328u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167332u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211274));
}
#[inline(always)]
pub fn block_0x00211224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167336u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2167340u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2167344u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2167348u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2167352u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2167364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211244));
    } else {
        emu.pc = 2167356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021123c));
    }
}
#[inline(always)]
pub fn block_0x0021123c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167360u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167364u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211250));
}
#[inline(always)]
pub fn block_0x00211244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2167368u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2167372u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2167376u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211250));
}
#[inline(always)]
pub fn block_0x00211250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2167380u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2167384u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2167388u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2167392u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2167396u32);
    emu.adi_no_count(17usize, 11usize, 4294966220u32, 2167400u32);
    emu.xri_no_count(11usize, 7usize, 1u32, 2167404u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2167408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211274));
}
#[inline(always)]
pub fn block_0x00211270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2167412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211274));
}
#[inline]
pub fn block_0x00211274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 11usize, 255u32, 2167416u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2167420u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2167424u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2167428u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2167432u32)?;
    emu.sw_no_count(22usize, 2usize, 8u32, 2167436u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2167440u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2167444u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2167448u32)?;
    emu.sh_no_count(17usize, 2usize, 24u32, 2167452u32)?;
    emu.sb_no_count(11usize, 2usize, 26u32, 2167456u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2167492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c4));
    } else {
        emu.pc = 2167460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112a4));
    }
}
#[inline(always)]
pub fn block_0x002112a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2167464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2167516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112dc));
    } else {
        emu.pc = 2167468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112ac));
    }
}
#[inline(always)]
pub fn block_0x002112ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 0u32, 2167472u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167476u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1902u32, 2167480u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2167484u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2167488u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2167492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113ec));
}
#[inline(always)]
pub fn block_0x002112c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2167544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112f8));
    } else {
        emu.pc = 2167496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c8));
    }
}
#[inline(always)]
pub fn block_0x002112c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2167500u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1901u32, 2167504u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2167508u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2167560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211308));
    } else {
        emu.pc = 2167512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112d8));
    }
}
#[inline(always)]
pub fn block_0x002112d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167568u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211310));
}
#[inline(always)]
pub fn block_0x002112dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 1u32, 2167520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2167700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211394));
    } else {
        emu.pc = 2167524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112e4));
    }
}
#[inline(always)]
pub fn block_0x002112e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2167528u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1901u32, 2167532u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2167536u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2167716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113a4));
    } else {
        emu.pc = 2167540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112f4));
    }
}
#[inline(always)]
pub fn block_0x002112f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113ac));
}
#[inline(always)]
pub fn block_0x002112f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2167548u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1900u32, 2167552u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2167556u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2167568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211310));
    } else {
        emu.pc = 2167560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211308));
    }
}
#[inline(always)]
pub fn block_0x00211308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2167564u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2167568u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2167568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211310));
}
#[inline(always)]
pub fn block_0x00211310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 48u32, 2167572u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2167576u32);
    emu.adi_no_count(21usize, 15usize, 0u32, 2167580u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2167584u32);
    emu.adi_no_count(20usize, 16usize, 0u32, 2167588u32);
    emu.adi_no_count(13usize, 16usize, 0u32, 2167592u32);
    emu.apc_no_count(1usize, 2167592u32, 0u32, 2167596u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2167604u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2167636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211354));
    } else {
        emu.pc = 2167608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211338));
    }
}
#[inline(always)]
pub fn block_0x00211338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 48u32, 2167612u32)?;
    emu.lw_no_count(11usize, 2usize, 52u32, 2167616u32)?;
    emu.lw_no_count(12usize, 2usize, 56u32, 2167620u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2167624u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2167628u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2167632u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2167636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021136c));
}
#[inline(always)]
pub fn block_0x00211354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2167640u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2167644u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2167648u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2167652u32);
    emu.apc_no_count(1usize, 2167652u32, 16384u32, 2167656u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167660u32;
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
pub fn block_0x0021136c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 36u32, 2167664u32)?;
    emu.lw_no_count(11usize, 2usize, 40u32, 2167668u32)?;
    emu.lh_no_count(12usize, 2usize, 44u32, 2167672u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2167676u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2167680u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2167684u32);
    emu.apc_no_count(1usize, 2167684u32, 0u32, 2167688u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966352u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021138c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2167696u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113fc));
}
#[inline(always)]
pub fn block_0x00211394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2167704u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 1900u32, 2167708u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2167712u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2167724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113ac));
    } else {
        emu.pc = 2167716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113a4));
    }
}
#[inline(always)]
pub fn block_0x002113a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 12usize, 0u32, 2167720u32);
    emu.adi_no_count(23usize, 10usize, 0u32, 2167724u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2167724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113ac));
}
#[inline(always)]
pub fn block_0x002113ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2167728u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2167772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113dc));
    } else {
        emu.pc = 2167732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113b4));
    }
}
#[inline(always)]
pub fn block_0x002113b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2167736u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2167740u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2167864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211438));
    } else {
        emu.pc = 2167744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113c0));
    }
}
#[inline(always)]
pub fn block_0x002113c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167748u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1817u32, 2167752u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2167756u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2167760u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2167764u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2167768u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2167772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113fc));
}
#[inline(always)]
pub fn block_0x002113dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2167776u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1905u32, 2167784u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2167788u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2167788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113ec));
}
#[inline(always)]
pub fn block_0x002113ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2167792u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2167796u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2167800u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2167804u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2167804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113fc));
}
#[inline]
pub fn block_0x002113fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 9usize, 0u32, 2167808u32)?;
    emu.sw_no_count(22usize, 9usize, 4u32, 2167812u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2167816u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2167820u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2167824u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2167828u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2167832u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2167836u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2167840u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2167844u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2167848u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2167852u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2167856u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2167860u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167864u32;
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
pub fn block_0x00211438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167868u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1908u32, 2167872u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2167876u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2167880u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2167884u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2167888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113fc));
}
#[inline(always)]
pub fn block_0x00211450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167892u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1764u32, 2167896u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2167900u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1820u32, 2167904u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2167908u32);
    emu.apc_no_count(1usize, 2167908u32, 4294959104u32, 2167912u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021146c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167920u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1836u32, 2167924u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2167928u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1884u32, 2167932u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2167936u32);
    emu.apc_no_count(1usize, 2167936u32, 4294959104u32, 2167940u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2167948u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2167952u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2167956u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2167960u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2167964u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2167968u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2167972u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2167976u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2167980u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2167984u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2167988u32)?;
    emu.adi_no_count(18usize, 14usize, 0u32, 2167992u32);
    emu.lw_no_count(19usize, 2usize, 96u32, 2167996u32)?;
    emu.adi_no_count(14usize, 0usize, 3u32, 2168000u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a >= b {
        emu.pc = 2168824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002117f8));
    } else {
        emu.pc = 2168004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114c4));
    }
}
#[inline(always)]
pub fn block_0x002114c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 17usize, 0u32, 2168008u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2168012u32);
    emu.sli_no_count(6usize, 12usize, 1u32, 2168016u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168020u32;
    emu.update_insn_clock();
    emu.sri_no_count(17usize, 6usize, 21u32, 2168024u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2168028u32);
    emu.anr_no_count(5usize, 12usize, 10usize, 2168032u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2168128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211540));
    } else {
        emu.pc = 2168036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114e4));
    }
}
#[inline]
pub fn block_0x002114e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 0usize, 11usize, 2168040u32);
    emu.adi_no_count(7usize, 11usize, 4294967295u32, 2168044u32);
    emu.adr_no_count(28usize, 5usize, 14usize, 2168048u32);
    emu.adi_no_count(14usize, 7usize, 1u32, 2168052u32);
    emu.sltiu_no_count(7usize, 14usize, 1u32, 2168056u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2168060u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2168064u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2168068u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2168072u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2168076u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2168080u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2168168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211568));
    } else {
        emu.pc = 2168084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211514));
    }
}
#[inline(always)]
pub fn block_0x00211514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 12usize, 6usize, 2168088u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2168176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211570));
    } else {
        emu.pc = 2168092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021151c));
    }
}
#[inline(always)]
pub fn block_0x0021151c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 14usize, 1u32, 2168096u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2168184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211578));
    } else {
        emu.pc = 2168100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211524));
    }
}
#[inline(always)]
pub fn block_0x00211524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 11usize, 5usize, 2168104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2168260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115c4));
    } else {
        emu.pc = 2168108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021152c));
    }
}
#[inline(always)]
pub fn block_0x0021152c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2168112u32);
    emu.adi_no_count(11usize, 17usize, 4294966221u32, 2168116u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2168120u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2168124u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2168128u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115c8));
}
#[inline]
pub fn block_0x00211540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 11usize, 31u32, 2168132u32);
    emu.orr_no_count(10usize, 6usize, 10usize, 2168136u32);
    emu.sli_no_count(14usize, 11usize, 1u32, 2168140u32);
    emu.sli_no_count(10usize, 10usize, 11u32, 2168144u32);
    emu.sri_no_count(10usize, 10usize, 11u32, 2168148u32);
    emu.sri_no_count(7usize, 6usize, 1u32, 2168152u32);
    let a = 0u32.wrapping_add(2146435072u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2168156u32;
    emu.update_insn_clock();
    emu.xrr_no_count(7usize, 7usize, 6usize, 2168160u32);
    emu.orr_no_count(7usize, 11usize, 7usize, 2168164u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2168084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211514));
    } else {
        emu.pc = 2168168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211568));
    }
}
#[inline(always)]
pub fn block_0x00211568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 3u32, 2168172u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115c8));
}
#[inline(always)]
pub fn block_0x00211570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 2u32, 2168180u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115c8));
}
#[inline(always)]
pub fn block_0x00211578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2168188u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 10usize, 11usize, 2168192u32);
    emu.orr_no_count(5usize, 14usize, 11usize, 2168196u32);
    emu.sltiu_no_count(11usize, 5usize, 1u32, 2168200u32);
    emu.sli_no_count(6usize, 14usize, 1u32, 2168204u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2168216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211598));
    } else {
        emu.pc = 2168208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211590));
    }
}
#[inline(always)]
pub fn block_0x00211590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4194304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168212u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115a4));
}
#[inline(always)]
pub fn block_0x00211598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 14usize, 31u32, 2168220u32);
    emu.sli_no_count(10usize, 10usize, 1u32, 2168224u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2168228u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2168228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115a4));
}
#[inline(always)]
pub fn block_0x002115a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 11usize, 4294967295u32, 2168232u32);
    emu.adi_no_count(5usize, 11usize, 1u32, 2168236u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2168240u32);
    emu.anr_no_count(14usize, 14usize, 6usize, 2168244u32);
    emu.sltiu_no_count(6usize, 5usize, 1u32, 2168248u32);
    emu.adi_no_count(11usize, 11usize, 4294966220u32, 2168252u32);
    emu.xri_no_count(17usize, 7usize, 1u32, 2168256u32);
    emu.add_memory_rw_events(8usize);
    let return_addr = 2168260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115c8));
}
#[inline(always)]
pub fn block_0x002115c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 4u32, 2168264u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115c8));
}
#[inline]
pub fn block_0x002115c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(7usize, 17usize, 255u32, 2168268u32);
    emu.sri_no_count(12usize, 12usize, 31u32, 2168272u32);
    emu.adi_no_count(23usize, 0usize, 1u32, 2168276u32);
    emu.sw_no_count(14usize, 2usize, 0u32, 2168280u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2168284u32)?;
    emu.sw_no_count(23usize, 2usize, 8u32, 2168288u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2168292u32)?;
    emu.sw_no_count(5usize, 2usize, 16u32, 2168296u32)?;
    emu.sw_no_count(6usize, 2usize, 20u32, 2168300u32)?;
    emu.sh_no_count(11usize, 2usize, 24u32, 2168304u32)?;
    emu.sb_no_count(17usize, 2usize, 26u32, 2168308u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2168344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211618));
    } else {
        emu.pc = 2168312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115f8));
    }
}
#[inline(always)]
pub fn block_0x002115f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2168316u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2168368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211630));
    } else {
        emu.pc = 2168320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211600));
    }
}
#[inline(always)]
pub fn block_0x00211600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2168324u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2168328u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1902u32, 2168332u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2168336u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2168340u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2168344u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168744u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117a8));
}
#[inline(always)]
pub fn block_0x00211618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2168396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021164c));
    } else {
        emu.pc = 2168348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021161c));
    }
}
#[inline(always)]
pub fn block_0x0021161c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2168352u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1901u32, 2168356u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2168360u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2168412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021165c));
    } else {
        emu.pc = 2168364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021162c));
    }
}
#[inline(always)]
pub fn block_0x0021162c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2168368u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211664));
}
#[inline(always)]
pub fn block_0x00211630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 1u32, 2168372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2168632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211738));
    } else {
        emu.pc = 2168376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211638));
    }
}
#[inline(always)]
pub fn block_0x00211638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2168380u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1901u32, 2168384u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2168388u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2168648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211748));
    } else {
        emu.pc = 2168392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211648));
    }
}
#[inline(always)]
pub fn block_0x00211648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2168396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211750));
}
#[inline(always)]
pub fn block_0x0021164c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2168400u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1900u32, 2168404u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2168408u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2168420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211664));
    } else {
        emu.pc = 2168412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021165c));
    }
}
#[inline(always)]
pub fn block_0x0021165c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2168416u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2168420u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2168420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211664));
}
#[inline]
pub fn block_0x00211664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 16u32, 2168424u32);
    emu.sai_no_count(10usize, 11usize, 1055u32, 2168428u32);
    emu.ani_no_count(10usize, 10usize, 4294967279u32, 2168432u32);
    emu.adi_no_count(10usize, 10usize, 5u32, 2168436u32);
    emu.sai_no_count(11usize, 11usize, 1040u32, 2168440u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2168444u32);
    emu.sri_no_count(10usize, 10usize, 4u32, 2168448u32);
    emu.adi_no_count(21usize, 10usize, 21u32, 2168452u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2168852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211814));
    } else {
        emu.pc = 2168456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211688));
    }
}
#[inline(always)]
pub fn block_0x00211688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 15u32, 2168460u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2168472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211698));
    } else {
        emu.pc = 2168464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211690));
    }
}
#[inline(always)]
pub fn block_0x00211690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4294934528u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168468u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168472u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021169c));
}
#[inline(always)]
pub fn block_0x00211698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 18usize, 2168476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021169c));
}
#[inline]
pub fn block_0x0021169c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 16u32, 2168480u32);
    emu.sai_no_count(20usize, 10usize, 1040u32, 2168484u32);
    emu.adi_no_count(10usize, 2usize, 44u32, 2168488u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2168492u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2168496u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2168500u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2168504u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2168508u32);
    emu.apc_no_count(1usize, 2168508u32, 4096u32, 2168512u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002116c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2168520u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2168560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116f0));
    } else {
        emu.pc = 2168524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116cc));
    }
}
#[inline(always)]
pub fn block_0x002116cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2168528u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2168532u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2168536u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2168540u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2168544u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2168548u32)?;
    emu.lh_no_count(12usize, 2usize, 40u32, 2168552u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2168596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211714));
    } else {
        emu.pc = 2168556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002116ec));
    }
}
#[inline(always)]
pub fn block_0x002116ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2168560u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211758));
}
#[inline(always)]
pub fn block_0x002116f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2168564u32);
    emu.adi_no_count(11usize, 2usize, 0u32, 2168568u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2168572u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2168576u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2168580u32);
    emu.apc_no_count(1usize, 2168580u32, 20480u32, 2168584u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021170c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(12usize, 2usize, 40u32, 2168592u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(20usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2168664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211758));
    } else {
        emu.pc = 2168596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211714));
    }
}
#[inline(always)]
pub fn block_0x00211714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 32u32, 2168600u32)?;
    emu.lw_no_count(11usize, 2usize, 36u32, 2168604u32)?;
    emu.adi_no_count(13usize, 18usize, 0u32, 2168608u32);
    emu.adi_no_count(14usize, 8usize, 0u32, 2168612u32);
    emu.adi_no_count(15usize, 19usize, 0u32, 2168616u32);
    emu.apc_no_count(1usize, 2168616u32, 0u32, 2168620u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168624u32;
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
#[inline(always)]
pub fn block_0x00211730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 10usize, 0u32, 2168628u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168632u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117b8));
}
#[inline(always)]
pub fn block_0x00211738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2168636u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 1900u32, 2168640u32);
    emu.adi_no_count(10usize, 24usize, 0u32, 2168644u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2168656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211750));
    } else {
        emu.pc = 2168648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211748));
    }
}
#[inline(always)]
pub fn block_0x00211748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 12usize, 0u32, 2168652u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2168656u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2168656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211750));
}
#[inline(always)]
pub fn block_0x00211750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2168660u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2168728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211798));
    } else {
        emu.pc = 2168664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211758));
    }
}
#[inline(always)]
pub fn block_0x00211758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2168668u32);
    emu.sh_no_count(11usize, 8usize, 0u32, 2168672u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2168704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211780));
    } else {
        emu.pc = 2168676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211764));
    }
}
#[inline(always)]
pub fn block_0x00211764(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168680u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1817u32, 2168684u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2168688u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2168692u32)?;
    emu.sh_no_count(0usize, 8usize, 12u32, 2168696u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2168700u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2168704u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117b8));
}
#[inline(always)]
pub fn block_0x00211780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168708u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1908u32, 2168712u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2168716u32);
    emu.sw_no_count(10usize, 8usize, 4u32, 2168720u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2168724u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2168728u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168760u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117b8));
}
#[inline(always)]
pub fn block_0x00211798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2168732u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2168736u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1905u32, 2168740u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2168744u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2168744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117a8));
}
#[inline(always)]
pub fn block_0x002117a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sh_no_count(10usize, 8usize, 0u32, 2168748u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2168752u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2168756u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2168760u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2168760u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002117b8));
}
#[inline]
pub fn block_0x002117b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(24usize, 9usize, 0u32, 2168764u32)?;
    emu.sw_no_count(23usize, 9usize, 4u32, 2168768u32)?;
    emu.sw_no_count(8usize, 9usize, 8u32, 2168772u32)?;
    emu.sw_no_count(11usize, 9usize, 12u32, 2168776u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2168780u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2168784u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2168788u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2168792u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2168796u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2168800u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2168804u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2168808u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2168812u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2168816u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2168820u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168824u32;
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
pub fn block_0x002117f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168828u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1764u32, 2168832u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2168836u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1912u32, 2168840u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2168844u32);
    emu.apc_no_count(1usize, 2168844u32, 4294959104u32, 2168848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2168856u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1928u32, 2168860u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2168864u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1968u32, 2168868u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2168872u32);
    emu.apc_no_count(1usize, 2168872u32, 4294959104u32, 2168876u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
