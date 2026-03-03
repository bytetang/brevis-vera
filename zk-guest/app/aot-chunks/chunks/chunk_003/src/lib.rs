pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2106748u32;
pub const PC_MAX: u32 = 2108924u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020257c,
        block_0x00202598,
        block_0x002025b4,
        block_0x002025bc,
        block_0x002025cc,
        block_0x002025d8,
        block_0x002025fc,
        block_0x0020261c,
        block_0x00202628,
        block_0x00202644,
        block_0x0020264c,
        block_0x00202664,
        block_0x00202668,
        block_0x0020267c,
        block_0x00202698,
        block_0x002026ac,
        block_0x002026c0,
        block_0x002026d0,
        block_0x00202700,
        block_0x00202720,
        block_0x00202730,
        block_0x00202734,
        block_0x002027a0,
        block_0x002027ac,
        block_0x002027b0,
        block_0x002027c8,
        block_0x002027d8,
        block_0x002027f0,
        block_0x002027f4,
        block_0x002027fc,
        block_0x00202808,
        block_0x00202814,
        block_0x00202834,
        block_0x0020284c,
        block_0x00202850,
        block_0x00202868,
        block_0x0020286c,
        block_0x00202874,
        block_0x00202890,
        block_0x00202894,
        block_0x002028a0,
        block_0x002028a8,
        block_0x002028bc,
        block_0x002028d0,
        block_0x002028d8,
        block_0x002028e8,
        block_0x00202900,
        block_0x00202904,
        block_0x00202914,
        block_0x00202924,
        block_0x00202934,
        block_0x0020296c,
        block_0x00202980,
        block_0x00202990,
        block_0x002029a0,
        block_0x002029a8,
        block_0x002029bc,
        block_0x002029cc,
        block_0x002029dc,
        block_0x002029fc,
        block_0x00202a0c,
        block_0x00202a1c,
        block_0x00202a2c,
        block_0x00202a34,
        block_0x00202a48,
        block_0x00202a58,
        block_0x00202a68,
        block_0x00202a84,
        block_0x00202a8c,
        block_0x00202ad4,
        block_0x00202b08,
        block_0x00202b24,
        block_0x00202b2c,
        block_0x00202b48,
        block_0x00202b50,
        block_0x00202b6c,
        block_0x00202b70,
        block_0x00202bb0,
        block_0x00202bc4,
        block_0x00202bd4,
        block_0x00202be4,
        block_0x00202bec,
        block_0x00202c00,
        block_0x00202c10,
        block_0x00202c20,
        block_0x00202c28,
        block_0x00202c3c,
        block_0x00202c50,
        block_0x00202c60,
        block_0x00202c68,
        block_0x00202c84,
        block_0x00202c98,
        block_0x00202cc0,
        block_0x00202cd0,
        block_0x00202ce4,
        block_0x00202d4c,
        block_0x00202d68,
        block_0x00202d70,
        block_0x00202d8c,
        block_0x00202d94,
        block_0x00202db0,
        block_0x00202db8,
        block_0x00202dd0,
        block_0x00202de4,
        block_0x00202dec,
        block_0x00202df0,
        block_0x00202dfc,
    ];
    const IDX: [u16; 545usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 3u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16,
        11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 20u16, 0u16, 0u16, 0u16, 21u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 24u16, 25u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 29u16, 0u16, 30u16, 0u16, 0u16, 31u16, 0u16, 0u16, 32u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 0u16, 38u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16,
        0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16,
        57u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16,
        79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16,
        89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 106u16,
        0u16, 0u16, 107u16,
    ];
    if pc < 2106748u32 || pc > 2108924u32 {
        return None;
    }
    let word_offset = ((pc - 2106748u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020257c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2106752u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2106756u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2106760u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2106764u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2106768u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2106772u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025fc));
    } else {
        emu.pc = 2106776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202598));
    }
}
#[inline(always)]
pub fn block_0x00202598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2106780u32)?;
    emu.lbu_no_count(9usize, 10usize, 0u32, 2106784u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2106788u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2106792u32);
    emu.sw_no_count(10usize, 11usize, 0u32, 2106796u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2106800u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2106920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202628));
    } else {
        emu.pc = 2106804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025b4));
    }
}
#[inline(always)]
pub fn block_0x002025b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106808u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2106948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202644));
    } else {
        emu.pc = 2106812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025bc));
    }
}
#[inline(always)]
pub fn block_0x002025bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2106816u32);
    emu.adi_no_count(15usize, 0usize, 5u32, 2106820u32);
    emu.apc_no_count(1usize, 2106820u32, 0u32, 2106824u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002025cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2106832u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106836u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2107032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202698));
    } else {
        emu.pc = 2106840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025d8));
    }
}
#[inline]
pub fn block_0x002025d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 12u32, 2106844u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2106848u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2106852u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106856u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2106860u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2106864u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2106868u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2106872u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106876u32;
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
pub fn block_0x002025fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106880u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2106888u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2106892u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2106896u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2106900u32);
    emu.apc_no_count(1usize, 2106900u32, 16384u32, 2106904u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020261c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106912u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2106916u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2106920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107004u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020267c));
}
#[inline(always)]
pub fn block_0x00202628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106924u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2106928u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2106932u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2106936u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2106940u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2106944u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106948u32;
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
pub fn block_0x00202644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106948u32, 8192u32, 2106952u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106956u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020264c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106960u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2106964u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106968u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2106972u32);
    emu.apc_no_count(1usize, 2106972u32, 12288u32, 2106976u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026c0));
    } else {
        emu.pc = 2106984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202668));
    }
}
#[inline(always)]
pub fn block_0x00202668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106988u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1u32, 2106992u32);
    emu.adi_no_count(12usize, 11usize, 3u32, 2106996u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2107000u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2107004u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107004u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020267c));
}
#[inline(always)]
pub fn block_0x0020267c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2107008u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2107012u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2107016u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107020u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107024u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107028u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107032u32;
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
pub fn block_0x00202698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 8u32, 2107036u32);
    emu.adi_no_count(12usize, 0usize, 72u32, 2107040u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2107044u32);
    emu.apc_no_count(1usize, 2107044u32, 12288u32, 2107048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2107056u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2107060u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2107064u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2107068u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107072u32;
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
pub fn block_0x002026c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107076u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107080u32);
    emu.apc_no_count(1usize, 2107080u32, 45056u32, 2107084u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002026d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2107092u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2107096u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2107100u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2107104u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2107108u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2107112u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2107116u32)?;
    emu.adi_no_count(18usize, 11usize, 0u32, 2107120u32);
    emu.lw_no_count(20usize, 11usize, 4u32, 2107124u32)?;
    emu.adi_no_count(11usize, 0usize, 7u32, 2107128u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2107132u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2107188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202734));
    } else {
        emu.pc = 2107136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202700));
    }
}
#[inline(always)]
pub fn block_0x00202700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2107140u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107144u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2107148u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2107152u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2107156u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107160u32);
    emu.apc_no_count(1usize, 2107160u32, 12288u32, 2107164u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107172u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2107176u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2107180u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2107308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027ac));
    } else {
        emu.pc = 2107184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202730));
    }
}
#[inline(always)]
pub fn block_0x00202730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107188u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107400u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202808));
}
#[inline(never)]
pub fn block_0x00202734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2107192u32)?;
    emu.lbu_no_count(11usize, 10usize, 1u32, 2107196u32);
    emu.lbu_no_count(12usize, 10usize, 2u32, 2107200u32);
    emu.lbu_no_count(13usize, 10usize, 3u32, 2107204u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2107208u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2107212u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2107216u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2107220u32);
    emu.orr_no_count(11usize, 11usize, 14usize, 2107224u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2107228u32);
    emu.lbu_no_count(15usize, 10usize, 5u32, 2107232u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2107236u32);
    emu.lbu_no_count(13usize, 10usize, 6u32, 2107240u32);
    emu.lbu_no_count(16usize, 10usize, 7u32, 2107244u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2107248u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2107252u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2107256u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2107260u32);
    emu.orr_no_count(13usize, 16usize, 13usize, 2107264u32);
    emu.adi_no_count(20usize, 20usize, 4294967288u32, 2107268u32);
    emu.adi_no_count(15usize, 10usize, 8u32, 2107272u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2107276u32);
    emu.orr_no_count(11usize, 13usize, 14usize, 2107280u32);
    emu.sw_no_count(15usize, 18usize, 0u32, 2107284u32)?;
    emu.sw_no_count(20usize, 18usize, 4u32, 2107288u32)?;
    emu.apc_no_count(1usize, 2107288u32, 12288u32, 2107292u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2107300u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2107304u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202808));
    } else {
        emu.pc = 2107308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027ac));
    }
}
#[inline(always)]
pub fn block_0x002027ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2107444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202834));
    } else {
        emu.pc = 2107312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027b0));
    }
}
#[inline(always)]
pub fn block_0x002027b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107316u32);
    emu.adi_no_count(11usize, 0usize, 37u32, 2107320u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107324u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2107328u32);
    emu.apc_no_count(1usize, 2107328u32, 16384u32, 2107332u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 8u32, 2107340u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2107344u32)?;
    emu.apc_no_count(1usize, 2107344u32, 8192u32, 2107348u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(828u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107356u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2107360u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107364u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2107368u32);
    emu.apc_no_count(1usize, 2107368u32, 12288u32, 2107372u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002027f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202924));
    } else {
        emu.pc = 2107380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027f4));
    }
}
#[inline(always)]
pub fn block_0x002027f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107384u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107388u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2107388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002027fc));
}
#[inline(always)]
pub fn block_0x002027fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 9usize, 0u32, 2107392u32)?;
    emu.sw_no_count(18usize, 9usize, 4u32, 2107396u32)?;
    emu.sw_no_count(19usize, 9usize, 8u32, 2107400u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2107400u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202808));
}
#[inline(always)]
pub fn block_0x00202808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107404u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2107408u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2107412u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2107412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202814));
}
#[inline(always)]
pub fn block_0x00202814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2107416u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2107420u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2107424u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2107428u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2107432u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2107436u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2107440u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107444u32;
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
pub fn block_0x00202834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 18usize, 0u32, 2107448u32)?;
    emu.sbr_no_count(10usize, 20usize, 9usize, 2107452u32);
    emu.adr_no_count(11usize, 19usize, 9usize, 2107456u32);
    emu.sw_no_count(11usize, 18usize, 0u32, 2107460u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2107464u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2107496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202868));
    } else {
        emu.pc = 2107468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020284c));
    }
}
#[inline(always)]
pub fn block_0x0020284c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2107472u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202850));
}
#[inline(always)]
pub fn block_0x00202850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107476u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966428u32, 2107480u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2107484u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107488u32);
    emu.apc_no_count(1usize, 2107488u32, 45056u32, 2107492u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002028a0));
    } else {
        emu.pc = 2107500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020286c));
    }
}
#[inline(always)]
pub fn block_0x0020286c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107500u32, 8192u32, 2107504u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107512u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2107516u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2107520u32);
    emu.adi_no_count(20usize, 0usize, 1u32, 2107524u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2107528u32);
    emu.apc_no_count(1usize, 2107528u32, 12288u32, 2107532u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202850));
    } else {
        emu.pc = 2107540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202894));
    }
}
#[inline(always)]
pub fn block_0x00202894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2107544u32);
    emu.adi_no_count(20usize, 9usize, 0u32, 2107548u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2107552u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002028a8));
}
#[inline(always)]
pub fn block_0x002028a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2107556u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2107560u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2107560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002028a8));
}
#[inline(always)]
pub fn block_0x002028a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2107564u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2107568u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2107572u32);
    emu.apc_no_count(1usize, 2107572u32, 12288u32, 2107576u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002028bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2107584u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2107588u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2107592u32);
    emu.apc_no_count(1usize, 2107592u32, 73728u32, 2107596u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107600u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002028d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2107604u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2107668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202914));
    } else {
        emu.pc = 2107608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002028d8));
    }
}
#[inline(always)]
pub fn block_0x002028d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 12u32, 2107612u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2107616u32)?;
    emu.apc_no_count(1usize, 2107616u32, 8192u32, 2107620u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107624u32;
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
#[inline(always)]
pub fn block_0x002028e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107628u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2107632u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2107636u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2107640u32);
    emu.apc_no_count(1usize, 2107640u32, 12288u32, 2107644u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202900(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202924));
    } else {
        emu.pc = 2107652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202904));
    }
}
#[inline(always)]
pub fn block_0x00202904(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107656u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107660u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2107664u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002027fc));
}
#[inline(always)]
pub fn block_0x00202914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 8usize, 0u32, 2107672u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2107676u32)?;
    emu.sw_no_count(9usize, 8usize, 8u32, 2107680u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107412u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202814));
}
#[inline(always)]
pub fn block_0x00202924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107688u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107692u32);
    emu.apc_no_count(1usize, 2107692u32, 45056u32, 2107696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(960u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2107704u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2107708u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2107712u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2107716u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2107720u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2107724u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2107728u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2107732u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2107736u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2107740u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2107744u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2107748u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107752u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2108008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a68));
    } else {
        emu.pc = 2107756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020296c));
    }
}
#[inline(always)]
pub fn block_0x0020296c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 15usize, 0u32, 2107760u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2107764u32);
    emu.adi_no_count(10usize, 2usize, 16u32, 2107768u32);
    emu.apc_no_count(1usize, 2107768u32, 0u32, 2107772u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 2usize, 16u32, 2107780u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2107784u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107788u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2107808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029a0));
    } else {
        emu.pc = 2107792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202990));
    }
}
#[inline(always)]
pub fn block_0x00202990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2107796u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2107800u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2107804u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107808u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ad4));
}
#[inline(always)]
pub fn block_0x002029a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2108168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b08));
    } else {
        emu.pc = 2107816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029a8));
    }
}
#[inline(always)]
pub fn block_0x002029a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 24u32, 2107820u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2107824u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2107828u32);
    emu.apc_no_count(1usize, 2107828u32, 4096u32, 2107832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002029bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 16u32, 2107840u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2107844u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2107848u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2107868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029dc));
    } else {
        emu.pc = 2107852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029cc));
    }
}
#[inline(always)]
pub fn block_0x002029cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2107856u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2107860u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2107864u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ad4));
}
#[inline(always)]
pub fn block_0x002029dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2107872u32)?;
    emu.lw_no_count(11usize, 2usize, 28u32, 2107876u32)?;
    emu.lw_no_count(12usize, 2usize, 32u32, 2107880u32)?;
    emu.adi_no_count(25usize, 20usize, 4294967294u32, 2107884u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2107888u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2107892u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2107896u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2108204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b2c));
    } else {
        emu.pc = 2107900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029fc));
    }
}
#[inline(always)]
pub fn block_0x002029fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 16u32, 2107904u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2107908u32);
    emu.apc_no_count(1usize, 2107908u32, 0u32, 2107912u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107916u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 16u32, 2107920u32)?;
    emu.lw_no_count(20usize, 2usize, 20u32, 2107924u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107928u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2107948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a2c));
    } else {
        emu.pc = 2107932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a1c));
    }
}
#[inline(always)]
pub fn block_0x00202a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2107936u32);
    emu.sw_no_count(10usize, 8usize, 0u32, 2107940u32)?;
    emu.sw_no_count(20usize, 8usize, 4u32, 2107944u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2107948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ad4));
}
#[inline(always)]
pub fn block_0x00202a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2107952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2108240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202b50));
    } else {
        emu.pc = 2107956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a34));
    }
}
#[inline(always)]
pub fn block_0x00202a34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 24u32, 2107960u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2107964u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2107968u32);
    emu.apc_no_count(1usize, 2107968u32, 0u32, 2107972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 16u32, 2107980u32)?;
    emu.lw_no_count(10usize, 2usize, 20u32, 2107984u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107988u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2108044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a8c));
    } else {
        emu.pc = 2107992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202a58));
    }
}
#[inline(always)]
pub fn block_0x00202a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2107996u32);
    emu.sw_no_count(11usize, 8usize, 0u32, 2108000u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2108004u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ad4));
}
#[inline(always)]
pub fn block_0x00202a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966296u32, 2108016u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108020u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108024u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2108028u32);
    emu.apc_no_count(1usize, 2108028u32, 8192u32, 2108032u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108036u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2108040u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107792u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202990));
}
#[inline]
pub fn block_0x00202a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 24u32, 2108048u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2108052u32)?;
    emu.lw_no_count(14usize, 2usize, 8u32, 2108056u32)?;
    emu.lw_no_count(15usize, 2usize, 12u32, 2108060u32)?;
    emu.sw_no_count(24usize, 8usize, 32u32, 2108064u32)?;
    emu.sw_no_count(20usize, 8usize, 36u32, 2108068u32)?;
    emu.sw_no_count(25usize, 8usize, 40u32, 2108072u32)?;
    emu.sw_no_count(11usize, 8usize, 44u32, 2108076u32)?;
    emu.sw_no_count(15usize, 8usize, 16u32, 2108080u32)?;
    emu.sw_no_count(21usize, 8usize, 20u32, 2108084u32)?;
    emu.sw_no_count(9usize, 8usize, 24u32, 2108088u32)?;
    emu.sw_no_count(22usize, 8usize, 28u32, 2108092u32)?;
    emu.sw_no_count(23usize, 8usize, 0u32, 2108096u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2108100u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2108104u32)?;
    emu.sw_no_count(14usize, 8usize, 12u32, 2108108u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2108112u32)?;
    emu.sw_no_count(12usize, 8usize, 52u32, 2108116u32)?;
    emu.add_memory_rw_events(18usize);
    emu.pc = 2108116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202ad4));
}
#[inline]
pub fn block_0x00202ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2108120u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2108124u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2108128u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2108132u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2108136u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2108140u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2108144u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2108148u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2108152u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2108156u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2108160u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2108164u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108168u32;
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
pub fn block_0x00202b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108172u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966296u32, 2108176u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108180u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108184u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108188u32);
    emu.apc_no_count(1usize, 2108188u32, 8192u32, 2108192u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2108200u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029cc));
}
#[inline(always)]
pub fn block_0x00202b2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108208u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966296u32, 2108212u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108216u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108220u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108224u32);
    emu.apc_no_count(1usize, 2108224u32, 8192u32, 2108228u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2108236u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108240u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107932u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a1c));
}
#[inline(always)]
pub fn block_0x00202b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108244u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966296u32, 2108248u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108252u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108256u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108260u32);
    emu.apc_no_count(1usize, 2108260u32, 8192u32, 2108264u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202b6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2108272u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107992u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202a58));
}
#[inline]
pub fn block_0x00202b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2108276u32);
    emu.sw_no_count(1usize, 2usize, 108u32, 2108280u32)?;
    emu.sw_no_count(8usize, 2usize, 104u32, 2108284u32)?;
    emu.sw_no_count(9usize, 2usize, 100u32, 2108288u32)?;
    emu.sw_no_count(18usize, 2usize, 96u32, 2108292u32)?;
    emu.sw_no_count(19usize, 2usize, 92u32, 2108296u32)?;
    emu.sw_no_count(20usize, 2usize, 88u32, 2108300u32)?;
    emu.sw_no_count(21usize, 2usize, 84u32, 2108304u32)?;
    emu.sw_no_count(22usize, 2usize, 80u32, 2108308u32)?;
    emu.sw_no_count(23usize, 2usize, 76u32, 2108312u32)?;
    emu.sw_no_count(24usize, 2usize, 72u32, 2108316u32)?;
    emu.sw_no_count(25usize, 2usize, 68u32, 2108320u32)?;
    emu.sw_no_count(26usize, 2usize, 64u32, 2108324u32)?;
    emu.sw_no_count(27usize, 2usize, 60u32, 2108328u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2108332u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2108748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d4c));
    } else {
        emu.pc = 2108336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bb0));
    }
}
#[inline(always)]
pub fn block_0x00202bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 15usize, 0u32, 2108340u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2108344u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2108348u32);
    emu.apc_no_count(1usize, 2108348u32, 0u32, 2108352u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 36u32, 2108360u32)?;
    emu.lw_no_count(9usize, 2usize, 40u32, 2108364u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108368u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2108388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202be4));
    } else {
        emu.pc = 2108372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bd4));
    }
}
#[inline(always)]
pub fn block_0x00202bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108376u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2108380u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2108384u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108388u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dfc));
}
#[inline(always)]
pub fn block_0x00202be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2108392u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2108784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d70));
    } else {
        emu.pc = 2108396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bec));
    }
}
#[inline(always)]
pub fn block_0x00202bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 44u32, 2108400u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2108404u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2108408u32);
    emu.apc_no_count(1usize, 2108408u32, 0u32, 2108412u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108416u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2108420u32)?;
    emu.lw_no_count(18usize, 2usize, 40u32, 2108424u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108428u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2108448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c20));
    } else {
        emu.pc = 2108432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c10));
    }
}
#[inline(always)]
pub fn block_0x00202c10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108436u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2108440u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2108444u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dfc));
}
#[inline(always)]
pub fn block_0x00202c20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2108452u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2108820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202d94));
    } else {
        emu.pc = 2108456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c28));
    }
}
#[inline(always)]
pub fn block_0x00202c28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 44u32, 2108460u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2108464u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2108468u32);
    emu.apc_no_count(1usize, 2108468u32, 4294963200u32, 2108472u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108476u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(27usize, 2usize, 36u32, 2108480u32)?;
    emu.lw_no_count(21usize, 2usize, 40u32, 2108484u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108488u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2108492u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2108512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c60));
    } else {
        emu.pc = 2108496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c50));
    }
}
#[inline(always)]
pub fn block_0x00202c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108500u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2108504u32)?;
    emu.sw_no_count(21usize, 8usize, 4u32, 2108508u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2108512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dfc));
}
#[inline(always)]
pub fn block_0x00202c60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2108516u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2108856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202db8));
    } else {
        emu.pc = 2108520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c68));
    }
}
#[inline(always)]
pub fn block_0x00202c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 16u32, 2108524u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2108528u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2108532u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2108536u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2108540u32);
    emu.apc_no_count(1usize, 2108540u32, 4294963200u32, 2108544u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108548u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202c84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 36u32, 2108552u32)?;
    emu.lw_no_count(22usize, 2usize, 40u32, 2108556u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108560u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2108564u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2108912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202df0));
    } else {
        emu.pc = 2108568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202c98));
    }
}
#[inline]
pub fn block_0x00202c98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2108572u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2108576u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2108580u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2108584u32)?;
    emu.adi_no_count(14usize, 0usize, 4u32, 2108588u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2108592u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2108596u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2108600u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2108604u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2108880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dd0));
    } else {
        emu.pc = 2108608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202cc0));
    }
}
#[inline(always)]
pub fn block_0x00202cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2108612u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2108616u32);
    emu.apc_no_count(1usize, 2108616u32, 4294963200u32, 2108620u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108624u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 36u32, 2108628u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2108632u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108636u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1u32, 2108640u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2108908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dec));
    } else {
        emu.pc = 2108644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ce4));
    }
}
#[inline(never)]
pub fn block_0x00202ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 44u32, 2108648u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2108652u32)?;
    emu.lw_no_count(14usize, 2usize, 24u32, 2108656u32)?;
    emu.lw_no_count(15usize, 2usize, 28u32, 2108660u32)?;
    emu.lw_no_count(16usize, 2usize, 32u32, 2108664u32)?;
    emu.sw_no_count(23usize, 8usize, 0u32, 2108668u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2108672u32)?;
    emu.sw_no_count(24usize, 8usize, 8u32, 2108676u32)?;
    emu.sw_no_count(25usize, 8usize, 12u32, 2108680u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2108684u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2108688u32)?;
    emu.sw_no_count(17usize, 8usize, 20u32, 2108692u32)?;
    emu.sw_no_count(27usize, 8usize, 24u32, 2108696u32)?;
    emu.sw_no_count(21usize, 8usize, 28u32, 2108700u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2108704u32)?;
    emu.sw_no_count(17usize, 8usize, 32u32, 2108708u32)?;
    emu.sw_no_count(26usize, 8usize, 36u32, 2108712u32)?;
    emu.sw_no_count(22usize, 8usize, 40u32, 2108716u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2108720u32)?;
    emu.sw_no_count(14usize, 8usize, 48u32, 2108724u32)?;
    emu.sw_no_count(15usize, 8usize, 52u32, 2108728u32)?;
    emu.sw_no_count(16usize, 8usize, 56u32, 2108732u32)?;
    emu.sw_no_count(11usize, 8usize, 60u32, 2108736u32)?;
    emu.sw_no_count(10usize, 8usize, 64u32, 2108740u32)?;
    emu.sw_no_count(12usize, 8usize, 68u32, 2108744u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2108748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108924u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dfc));
}
#[inline(always)]
pub fn block_0x00202d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108752u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966248u32, 2108756u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108760u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108764u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2108768u32);
    emu.apc_no_count(1usize, 2108768u32, 4096u32, 2108772u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2108780u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202bd4));
}
#[inline(always)]
pub fn block_0x00202d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108788u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966248u32, 2108792u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108796u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108800u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108804u32);
    emu.apc_no_count(1usize, 2108804u32, 4096u32, 2108808u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2108816u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c10));
}
#[inline(always)]
pub fn block_0x00202d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108824u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966248u32, 2108828u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108832u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108836u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108840u32);
    emu.apc_no_count(1usize, 2108840u32, 4096u32, 2108844u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108848u32;
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
pub fn block_0x00202db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2108852u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2108856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108496u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202c50));
}
#[inline(always)]
pub fn block_0x00202db8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108860u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966248u32, 2108864u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108868u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108872u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108876u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2108880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2108900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202de4));
}
#[inline(always)]
pub fn block_0x00202dd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2108884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966248u32, 2108888u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108892u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966152u32, 2108896u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108900u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2108900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202de4));
}
#[inline(always)]
pub fn block_0x00202de4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2108900u32, 4096u32, 2108904u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 10usize, 0u32, 2108912u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2108912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202df0));
}
#[inline(always)]
pub fn block_0x00202df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108916u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2108920u32)?;
    emu.sw_no_count(22usize, 8usize, 4u32, 2108924u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2108924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202dfc));
}
#[inline]
pub fn block_0x00202dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 108u32, 2108928u32)?;
    emu.lw_no_count(8usize, 2usize, 104u32, 2108932u32)?;
    emu.lw_no_count(9usize, 2usize, 100u32, 2108936u32)?;
    emu.lw_no_count(18usize, 2usize, 96u32, 2108940u32)?;
    emu.lw_no_count(19usize, 2usize, 92u32, 2108944u32)?;
    emu.lw_no_count(20usize, 2usize, 88u32, 2108948u32)?;
    emu.lw_no_count(21usize, 2usize, 84u32, 2108952u32)?;
    emu.lw_no_count(22usize, 2usize, 80u32, 2108956u32)?;
    emu.lw_no_count(23usize, 2usize, 76u32, 2108960u32)?;
    emu.lw_no_count(24usize, 2usize, 72u32, 2108964u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2108968u32)?;
    emu.lw_no_count(26usize, 2usize, 64u32, 2108972u32)?;
    emu.lw_no_count(27usize, 2usize, 60u32, 2108976u32)?;
    emu.adi_no_count(2usize, 2usize, 112u32, 2108980u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108984u32;
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
