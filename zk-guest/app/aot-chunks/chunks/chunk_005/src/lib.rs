pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2114668u32;
pub const PC_MAX: u32 = 2116640u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0020446c,
        block_0x00204490,
        block_0x002044a0,
        block_0x002044a8,
        block_0x002044c4,
        block_0x002044c8,
        block_0x002044dc,
        block_0x002044ec,
        block_0x002044f0,
        block_0x002044f8,
        block_0x00204510,
        block_0x0020451c,
        block_0x00204520,
        block_0x00204528,
        block_0x00204538,
        block_0x00204548,
        block_0x00204550,
        block_0x00204558,
        block_0x0020455c,
        block_0x00204580,
        block_0x002045ac,
        block_0x002045b0,
        block_0x002045c0,
        block_0x002045d8,
        block_0x002045dc,
        block_0x002045e4,
        block_0x002045f0,
        block_0x0020460c,
        block_0x00204624,
        block_0x0020462c,
        block_0x00204650,
        block_0x00204658,
        block_0x00204664,
        block_0x00204690,
        block_0x00204694,
        block_0x002046a4,
        block_0x002046b4,
        block_0x002046b8,
        block_0x002046c0,
        block_0x002046cc,
        block_0x002046e4,
        block_0x002046fc,
        block_0x00204704,
        block_0x00204728,
        block_0x00204730,
        block_0x0020473c,
        block_0x00204750,
        block_0x00204758,
        block_0x0020476c,
        block_0x00204770,
        block_0x0020477c,
        block_0x00204784,
        block_0x00204788,
        block_0x00204790,
        block_0x00204794,
        block_0x00204798,
        block_0x0020479c,
        block_0x002047b8,
        block_0x002047c8,
        block_0x002047cc,
        block_0x002047d4,
        block_0x002047e4,
        block_0x002047f8,
        block_0x00204810,
        block_0x00204818,
        block_0x00204838,
        block_0x0020484c,
        block_0x00204854,
        block_0x00204864,
        block_0x002048d0,
        block_0x002048e4,
        block_0x002048f8,
        block_0x0020490c,
        block_0x00204920,
        block_0x00204934,
        block_0x00204948,
        block_0x0020495c,
        block_0x00204970,
        block_0x00204984,
        block_0x00204998,
        block_0x002049ac,
        block_0x002049c0,
        block_0x002049d4,
        block_0x002049ec,
        block_0x00204a00,
        block_0x00204a0c,
        block_0x00204a28,
        block_0x00204a40,
        block_0x00204a48,
        block_0x00204a50,
        block_0x00204a64,
        block_0x00204a78,
        block_0x00204a7c,
        block_0x00204a94,
        block_0x00204abc,
        block_0x00204aec,
        block_0x00204af4,
        block_0x00204b14,
        block_0x00204b30,
        block_0x00204b48,
        block_0x00204b60,
        block_0x00204b74,
        block_0x00204b80,
        block_0x00204b94,
        block_0x00204ba8,
        block_0x00204bbc,
        block_0x00204bc0,
        block_0x00204bd0,
        block_0x00204be0,
        block_0x00204bf0,
        block_0x00204c00,
        block_0x00204c14,
        block_0x00204c20,
    ];
    const IDX: [u16; 494usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16,
        15u16, 0u16, 0u16, 0u16, 16u16, 0u16, 17u16, 0u16, 18u16, 19u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16,
        0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 39u16, 0u16, 0u16, 40u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16,
        43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16,
        0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        49u16, 50u16, 0u16, 0u16, 51u16, 0u16, 52u16, 53u16, 0u16, 54u16, 55u16, 56u16,
        57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 60u16,
        0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16,
        0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16, 0u16,
        78u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16,
        0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16,
        86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        88u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16,
        102u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 107u16, 0u16, 0u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 111u16,
        0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 0u16, 113u16,
    ];
    if pc < 2114668u32 || pc > 2116640u32 {
        return None;
    }
    let word_offset = ((pc - 2114668u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020446c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2114672u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2114676u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2114680u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2114684u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2114688u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2114692u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2114696u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2114700u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2114848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204520));
    } else {
        emu.pc = 2114704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204490));
    }
}
#[inline(always)]
pub fn block_0x00204490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 12usize, 0u32, 2114708u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2114712u32);
    emu.lw_no_count(10usize, 13usize, 4u32, 2114716u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2114796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044ec));
    } else {
        emu.pc = 2114720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044a0));
    }
}
#[inline(always)]
pub fn block_0x002044a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 13usize, 8u32, 2114724u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2114796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044ec));
    } else {
        emu.pc = 2114728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044a8));
    }
}
#[inline(always)]
pub fn block_0x002044a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 13usize, 0u32, 2114732u32)?;
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2114736u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2114740u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2114744u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2114748u32);
    emu.apc_no_count(1usize, 2114748u32, 24576u32, 2114752u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002044c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204548));
    } else {
        emu.pc = 2114760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044c8));
    }
}
#[inline(always)]
pub fn block_0x002044c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2114764u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2114768u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2114772u32);
    emu.apc_no_count(1usize, 2114772u32, 24576u32, 2114776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114780u32;
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
pub fn block_0x002044dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2114784u32);
    emu.adi_no_count(11usize, 8usize, 4u32, 2114788u32);
    emu.adi_no_count(12usize, 8usize, 8u32, 2114792u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2114796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204538));
}
#[inline(always)]
pub fn block_0x002044ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204528));
    } else {
        emu.pc = 2114800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002044f0));
    }
}
#[inline(always)]
pub fn block_0x002044f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2114800u32, 20480u32, 2114804u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114808u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002044f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2114812u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966828u32, 2114816u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2114820u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2114824u32);
    emu.apc_no_count(1usize, 2114824u32, 24576u32, 2114828u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 4u32, 2114836u32);
    emu.adi_no_count(12usize, 8usize, 8u32, 2114840u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2114872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204538));
    } else {
        emu.pc = 2114844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020451c));
    }
}
#[inline(always)]
pub fn block_0x0020451c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2114848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204550));
}
#[inline(always)]
pub fn block_0x00204520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2114852u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2114856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204558));
}
#[inline(always)]
pub fn block_0x00204528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2114860u32);
    emu.adi_no_count(11usize, 8usize, 4u32, 2114864u32);
    emu.adi_no_count(12usize, 8usize, 8u32, 2114868u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2114896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204550));
    } else {
        emu.pc = 2114872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204538));
    }
}
#[inline(always)]
pub fn block_0x00204538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2114876u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2114880u32)?;
    emu.sw_no_count(9usize, 12usize, 0u32, 2114884u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2114888u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020455c));
}
#[inline(always)]
pub fn block_0x00204548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 4u32, 2114892u32);
    emu.adi_no_count(12usize, 8usize, 8u32, 2114896u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2114896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204550));
}
#[inline(always)]
pub fn block_0x00204550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 11usize, 0u32, 2114900u32)?;
    emu.sw_no_count(9usize, 12usize, 0u32, 2114904u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2114904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204558));
}
#[inline(always)]
pub fn block_0x00204558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 1u32, 2114908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2114908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020455c));
}
#[inline]
pub fn block_0x0020455c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 8usize, 0u32, 2114912u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2114916u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2114920u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2114924u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2114928u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2114932u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2114936u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2114940u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114944u32;
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
pub fn block_0x00204580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2114948u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2114952u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2114956u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2114960u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2114964u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2114968u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2114972u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2114976u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2114980u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2114984u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2114992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045b0));
    } else {
        emu.pc = 2114988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045ac));
    }
}
#[inline(always)]
pub fn block_0x002045ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2114992u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2114992u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002045b0));
}
#[inline(always)]
pub fn block_0x002045b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 56u32, 2114996u32);
    emu.mulhu_no_count(11usize, 18usize, 10usize, 2115000u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2115004u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2115044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045e4));
    } else {
        emu.pc = 2115008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045c0));
    }
}
#[inline(always)]
pub fn block_0x002045c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 18usize, 3u32, 2115012u32);
    emu.sli_no_count(12usize, 18usize, 6u32, 2115016u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2115020u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 11usize, 2115024u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2115028u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2115160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204658));
    } else {
        emu.pc = 2115032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045d8));
    }
}
#[inline(always)]
pub fn block_0x002045d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2115056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045f0));
    } else {
        emu.pc = 2115036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002045dc));
    }
}
#[inline(always)]
pub fn block_0x002045dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2115040u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2115044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2115084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020460c));
}
#[inline(always)]
pub fn block_0x002045e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2115048u32);
    emu.apc_no_count(1usize, 2115048u32, 94208u32, 2115052u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002045f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2115060u32)?;
    emu.sli_no_count(11usize, 13usize, 3u32, 2115064u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2115068u32);
    emu.sbr_no_count(13usize, 13usize, 11usize, 2115072u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2115076u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2115080u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2115084u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2115084u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020460c));
}
#[inline(always)]
pub fn block_0x0020460c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2115088u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2115092u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2115096u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2115100u32);
    emu.apc_no_count(1usize, 2115100u32, 0u32, 2115104u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2115112u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2115152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204650));
    } else {
        emu.pc = 2115116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020462c));
    }
}
#[inline]
pub fn block_0x0020462c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2115120u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2115124u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2115128u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2115132u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2115136u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2115140u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2115144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2115148u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115152u32;
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
pub fn block_0x00204650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2115156u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2115160u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2115160u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204658));
}
#[inline(always)]
pub fn block_0x00204658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2115164u32);
    emu.apc_no_count(1usize, 2115164u32, 94208u32, 2115168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115172u32;
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
#[inline]
pub fn block_0x00204664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2115176u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2115180u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2115184u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2115188u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2115192u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2115196u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2115200u32)?;
    emu.sli_no_count(18usize, 13usize, 1u32, 2115204u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2115208u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2115212u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2115220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204694));
    } else {
        emu.pc = 2115216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204690));
    }
}
#[inline(always)]
pub fn block_0x00204690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 4u32, 2115220u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2115220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204694));
}
#[inline(always)]
pub fn block_0x00204694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 20u32, 2115224u32);
    emu.mulhu_no_count(12usize, 18usize, 11usize, 2115228u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2115232u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2115264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002046c0));
    } else {
        emu.pc = 2115236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002046a4));
    }
}
#[inline(always)]
pub fn block_0x002046a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 18usize, 11usize, 2115240u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2115244u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 11usize, 4294967292u32, 2115248u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2115376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204730));
    } else {
        emu.pc = 2115252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002046b4));
    }
}
#[inline(always)]
pub fn block_0x002046b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2115276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002046cc));
    } else {
        emu.pc = 2115256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002046b8));
    }
}
#[inline(always)]
pub fn block_0x002046b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2115260u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2115264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2115300u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002046e4));
}
#[inline(always)]
pub fn block_0x002046c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2115268u32);
    emu.apc_no_count(1usize, 2115268u32, 94208u32, 2115272u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115276u32;
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
pub fn block_0x002046cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 4u32, 2115280u32)?;
    emu.adi_no_count(11usize, 0usize, 20u32, 2115284u32);
    emu.mul_no_count(11usize, 13usize, 11usize, 2115288u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2115292u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2115296u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2115300u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2115300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002046e4));
}
#[inline(always)]
pub fn block_0x002046e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 24u32, 2115304u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2115308u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2115312u32);
    emu.adi_no_count(13usize, 2usize, 20u32, 2115316u32);
    emu.apc_no_count(1usize, 2115316u32, 0u32, 2115320u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002046fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2115328u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2115368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204728));
    } else {
        emu.pc = 2115332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204704));
    }
}
#[inline]
pub fn block_0x00204704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2115336u32)?;
    emu.sw_no_count(18usize, 9usize, 0u32, 2115340u32)?;
    emu.sw_no_count(10usize, 9usize, 4u32, 2115344u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2115348u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2115352u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2115356u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2115360u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2115364u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115368u32;
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
pub fn block_0x00204728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2115372u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2115376u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2115376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204730));
}
#[inline(always)]
pub fn block_0x00204730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2115380u32);
    emu.apc_no_count(1usize, 2115380u32, 94208u32, 2115384u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020473c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2115392u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2115396u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2115400u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2115404u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2115640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204838));
    } else {
        emu.pc = 2115408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204750));
    }
}
#[inline(always)]
pub fn block_0x00204750(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 11usize, 12usize, 2115412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2115640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204838));
    } else {
        emu.pc = 2115416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204758));
    }
}
#[inline(always)]
pub fn block_0x00204758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 13usize, 0u32, 2115420u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2115424u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2115428u32)?;
    emu.sli_no_count(10usize, 13usize, 1u32, 2115432u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2115440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204770));
    } else {
        emu.pc = 2115436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020476c));
    }
}
#[inline(always)]
pub fn block_0x0020476c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2115440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2115440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204770));
}
#[inline(always)]
pub fn block_0x00204770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1025u32, 2115444u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2115448u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2115464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204788));
    } else {
        emu.pc = 2115452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020477c));
    }
}
#[inline(always)]
pub fn block_0x0020477c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2115456u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2115472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204790));
    } else {
        emu.pc = 2115460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204784));
    }
}
#[inline(always)]
pub fn block_0x00204784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2115464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2115476u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204794));
}
#[inline(always)]
pub fn block_0x00204788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2115468u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2115476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204794));
    } else {
        emu.pc = 2115472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204790));
    }
}
#[inline(always)]
pub fn block_0x00204790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2115476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2115476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204794));
}
#[inline(always)]
pub fn block_0x00204794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2115484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020479c));
    } else {
        emu.pc = 2115480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204798));
    }
}
#[inline(always)]
pub fn block_0x00204798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2115484u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2115484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020479c));
}
#[inline(always)]
pub fn block_0x0020479c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 15usize, 14usize, 2115488u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2115492u32);
    emu.sbr_no_count(11usize, 0usize, 15usize, 2115496u32);
    emu.anr_no_count(11usize, 10usize, 11usize, 2115500u32);
    emu.mulhu_no_count(12usize, 11usize, 9usize, 2115504u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2115508u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2115540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002047d4));
    } else {
        emu.pc = 2115512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002047b8));
    }
}
#[inline(always)]
pub fn block_0x002047b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 11usize, 9usize, 2115516u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2115520u32;
    emu.update_insn_clock();
    emu.sbr_no_count(16usize, 11usize, 15usize, 2115524u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2115668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204854));
    } else {
        emu.pc = 2115528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002047c8));
    }
}
#[inline(always)]
pub fn block_0x002047c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2115556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002047e4));
    } else {
        emu.pc = 2115532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002047cc));
    }
}
#[inline(always)]
pub fn block_0x002047cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2115536u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2115540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2115576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002047f8));
}
#[inline(always)]
pub fn block_0x002047d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2115544u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 320u32, 2115548u32);
    emu.apc_no_count(1usize, 2115548u32, 94208u32, 2115552u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002047e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2115560u32)?;
    emu.mul_no_count(11usize, 13usize, 14usize, 2115564u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2115568u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2115572u32)?;
    emu.adi_no_count(10usize, 15usize, 0u32, 2115576u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2115576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002047f8));
}
#[inline(always)]
pub fn block_0x002047f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 28u32, 2115580u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2115584u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2115588u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2115592u32);
    emu.apc_no_count(1usize, 2115592u32, 0u32, 2115596u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2115604u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2115660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020484c));
    } else {
        emu.pc = 2115608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204818));
    }
}
#[inline(always)]
pub fn block_0x00204818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2115612u32)?;
    emu.sw_no_count(9usize, 8usize, 0u32, 2115616u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2115620u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2115624u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2115628u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2115632u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2115636u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115640u32;
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
pub fn block_0x00204838(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2115644u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2115648u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 320u32, 2115652u32);
    emu.apc_no_count(1usize, 2115652u32, 94208u32, 2115656u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115660u32;
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
#[inline(always)]
pub fn block_0x0020484c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2115664u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2115668u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2115668u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204854));
}
#[inline(always)]
pub fn block_0x00204854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2115672u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 320u32, 2115676u32);
    emu.apc_no_count(1usize, 2115676u32, 94208u32, 2115680u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115684u32;
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
#[inline(never)]
pub fn block_0x00204864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965264u32, 2115688u32);
    emu.sw_no_count(1usize, 2usize, 2028u32, 2115692u32)?;
    emu.sw_no_count(8usize, 2usize, 2024u32, 2115696u32)?;
    emu.sw_no_count(9usize, 2usize, 2020u32, 2115700u32)?;
    emu.sw_no_count(18usize, 2usize, 2016u32, 2115704u32)?;
    emu.sw_no_count(19usize, 2usize, 2012u32, 2115708u32)?;
    emu.sw_no_count(20usize, 2usize, 2008u32, 2115712u32)?;
    emu.sw_no_count(21usize, 2usize, 2004u32, 2115716u32)?;
    emu.sw_no_count(22usize, 2usize, 2000u32, 2115720u32)?;
    emu.sw_no_count(23usize, 2usize, 1996u32, 2115724u32)?;
    emu.sw_no_count(24usize, 2usize, 1992u32, 2115728u32)?;
    emu.sw_no_count(25usize, 2usize, 1988u32, 2115732u32)?;
    emu.sw_no_count(26usize, 2usize, 1984u32, 2115736u32)?;
    emu.sw_no_count(27usize, 2usize, 1980u32, 2115740u32)?;
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2115744u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2115748u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2115752u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2115756u32)?;
    emu.adi_no_count(20usize, 2usize, 108u32, 2115760u32);
    emu.adi_no_count(19usize, 2usize, 204u32, 2115764u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2115768u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 192u32, 2115772u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115776u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2115780u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115784u32);
    emu.apc_no_count(1usize, 2115784u32, 24576u32, 2115788u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115792u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002048d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 300u32, 2115796u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115800u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115804u32);
    emu.apc_no_count(1usize, 2115804u32, 24576u32, 2115808u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002048e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 396u32, 2115816u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115820u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115824u32);
    emu.apc_no_count(1usize, 2115824u32, 24576u32, 2115828u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115832u32;
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
pub fn block_0x002048f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 492u32, 2115836u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115840u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115844u32);
    emu.apc_no_count(1usize, 2115844u32, 24576u32, 2115848u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020490c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 588u32, 2115856u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115860u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115864u32);
    emu.apc_no_count(1usize, 2115864u32, 24576u32, 2115868u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2115876u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115880u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115884u32);
    emu.apc_no_count(1usize, 2115884u32, 24576u32, 2115888u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 780u32, 2115896u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115900u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115904u32);
    emu.apc_no_count(1usize, 2115904u32, 24576u32, 2115908u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 876u32, 2115916u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115920u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115924u32);
    emu.apc_no_count(1usize, 2115924u32, 24576u32, 2115928u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020495c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 972u32, 2115936u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115940u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115944u32);
    emu.apc_no_count(1usize, 2115944u32, 24576u32, 2115948u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1068u32, 2115956u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115960u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115964u32);
    emu.apc_no_count(1usize, 2115964u32, 24576u32, 2115968u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966112u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1164u32, 2115976u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2115980u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2115984u32);
    emu.apc_no_count(1usize, 2115984u32, 24576u32, 2115988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2115992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1260u32, 2115996u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116000u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2116004u32);
    emu.apc_no_count(1usize, 2116004u32, 24576u32, 2116008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116012u32;
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
#[inline(always)]
pub fn block_0x002049ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1356u32, 2116016u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116020u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2116024u32);
    emu.apc_no_count(1usize, 2116024u32, 24576u32, 2116028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002049c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1452u32, 2116036u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116040u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2116044u32);
    emu.apc_no_count(1usize, 2116044u32, 24576u32, 2116048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116052u32;
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
pub fn block_0x002049d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2116056u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116060u32);
    emu.adi_no_count(8usize, 2usize, 12u32, 2116064u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2116068u32);
    emu.apc_no_count(1usize, 2116068u32, 24576u32, 2116072u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116076u32;
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
pub fn block_0x002049ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 96u32, 2116080u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2116084u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2116088u32);
    emu.apc_no_count(1usize, 2116088u32, 24576u32, 2116092u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2116100u32);
    emu.adi_no_count(20usize, 0usize, 16u32, 2116104u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2116108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2116168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204a48));
}
#[inline(always)]
pub fn block_0x00204a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 9usize, 4u32, 2116112u32);
    emu.sli_no_count(11usize, 9usize, 6u32, 2116116u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2116120u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2116124u32);
    emu.adi_no_count(10usize, 2usize, 1932u32, 2116128u32);
    emu.apc_no_count(1usize, 2116128u32, 4096u32, 2116132u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 1u32, 2116140u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2116144u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116148u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2116152u32);
    emu.apc_no_count(1usize, 2116152u32, 24576u32, 2116156u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 96u32, 2116164u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2116220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204a7c));
    } else {
        emu.pc = 2116168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204a48));
    }
}
#[inline(always)]
pub fn block_0x00204a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 1u32, 2116172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2116108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204a0c));
    } else {
        emu.pc = 2116176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204a50));
    }
}
#[inline(always)]
pub fn block_0x00204a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 4294967200u32, 2116180u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2116184u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116188u32);
    emu.apc_no_count(1usize, 2116188u32, 24576u32, 2116192u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2116200u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2116204u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2116208u32);
    emu.apc_no_count(1usize, 2116208u32, 4096u32, 2116212u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2116220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2116136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204a28));
}
#[inline(always)]
pub fn block_0x00204a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2116224u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 192u32, 2116228u32);
    emu.adi_no_count(10usize, 2usize, 1548u32, 2116232u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116236u32);
    emu.apc_no_count(1usize, 2116236u32, 24576u32, 2116240u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 2usize, 1676u32, 2116248u32);
    emu.adi_no_count(20usize, 2usize, 1708u32, 2116252u32);
    emu.adi_no_count(21usize, 2usize, 2047u32, 2116256u32);
    emu.adi_no_count(21usize, 21usize, 13u32, 2116260u32);
    emu.adi_no_count(22usize, 2usize, 2047u32, 2116264u32);
    emu.adi_no_count(22usize, 22usize, 45u32, 2116268u32);
    emu.adi_no_count(26usize, 0usize, 252u32, 2116272u32);
    emu.adi_no_count(10usize, 0usize, 31u32, 2116276u32);
    emu.adi_no_count(27usize, 2usize, 172u32, 2116280u32);
    emu.adi_no_count(8usize, 0usize, 16u32, 2116284u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2116284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204abc));
}
#[inline]
pub fn block_0x00204abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2116288u32)?;
    emu.adr_no_count(10usize, 11usize, 10usize, 2116292u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2116296u32);
    emu.ani_no_count(11usize, 26usize, 4u32, 2116300u32);
    emu.srr_no_count(9usize, 10usize, 11usize, 2116304u32);
    emu.ani_no_count(9usize, 9usize, 15u32, 2116308u32);
    emu.adi_no_count(10usize, 2usize, 1644u32, 2116312u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116316u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2116320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 192u32, 2116324u32);
    emu.apc_no_count(1usize, 2116324u32, 24576u32, 2116328u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204aec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2116336u32);
    emu.adi_no_count(23usize, 27usize, 0u32, 2116340u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2116340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204af4));
}
#[inline(always)]
pub fn block_0x00204af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 1u32, 2116344u32);
    emu.adi_no_count(24usize, 23usize, 4294967232u32, 2116348u32);
    emu.xrr_no_count(10usize, 10usize, 9usize, 2116352u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2116356u32);
    emu.sli_no_count(10usize, 10usize, 23u32, 2116360u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2116364u32);
    emu.apc_no_count(1usize, 2116364u32, 24576u32, 2116368u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(25usize, 10usize, 255u32, 2116376u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2116380u32);
    emu.adi_no_count(11usize, 2usize, 1644u32, 2116384u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2116388u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2116392u32);
    emu.apc_no_count(1usize, 2116392u32, 57344u32, 2116396u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966168u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 4294967264u32, 2116404u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2116408u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2116412u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2116416u32);
    emu.apc_no_count(1usize, 2116416u32, 57344u32, 2116420u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 22usize, 0u32, 2116428u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2116432u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2116436u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2116440u32);
    emu.apc_no_count(1usize, 2116440u32, 57344u32, 2116444u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116448u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1644u32, 2116452u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2116456u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116460u32);
    emu.apc_no_count(1usize, 2116460u32, 24576u32, 2116464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116468u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 23usize, 96u32, 2116472u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2116476u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2116340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204af4));
    } else {
        emu.pc = 2116480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204b80));
    }
}
#[inline(always)]
pub fn block_0x00204b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2116484u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2116488u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116492u32);
    emu.apc_no_count(1usize, 2116492u32, 24576u32, 2116496u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116500u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2116504u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2116508u32);
    emu.adi_no_count(12usize, 2usize, 1644u32, 2116512u32);
    emu.apc_no_count(1usize, 2116512u32, 0u32, 2116516u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116520u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2116524u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2116528u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116532u32);
    emu.apc_no_count(1usize, 2116532u32, 24576u32, 2116536u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2116640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204c20));
    } else {
        emu.pc = 2116544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204bc0));
    }
}
#[inline(always)]
pub fn block_0x00204bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2116548u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2116552u32);
    emu.apc_no_count(1usize, 2116552u32, 4096u32, 2116556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116560u32;
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
pub fn block_0x00204bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2116564u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2116568u32);
    emu.apc_no_count(1usize, 2116568u32, 4096u32, 2116572u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1836u32, 2116580u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2116584u32);
    emu.apc_no_count(1usize, 2116584u32, 4096u32, 2116588u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204bf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1740u32, 2116596u32);
    emu.adi_no_count(11usize, 2usize, 1836u32, 2116600u32);
    emu.apc_no_count(1usize, 2116600u32, 4096u32, 2116604u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2116612u32);
    emu.adi_no_count(11usize, 2usize, 1740u32, 2116616u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116620u32);
    emu.apc_no_count(1usize, 2116620u32, 24576u32, 2116624u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116628u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00204c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 26usize, 4294967292u32, 2116632u32);
    emu.sri_no_count(10usize, 26usize, 3u32, 2116636u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2116640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2116284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204abc));
}
#[inline(always)]
pub fn block_0x00204c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1548u32, 2116644u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2116648u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2116652u32)?;
    emu.apc_no_count(1usize, 2116652u32, 24576u32, 2116656u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
