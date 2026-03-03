pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2111864u32;
pub const PC_MAX: u32 = 2114292u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 108usize] = [
        block_0x00203978,
        block_0x002039c8,
        block_0x002039cc,
        block_0x002039d0,
        block_0x002039e4,
        block_0x002039fc,
        block_0x00203a00,
        block_0x00203a30,
        block_0x00203a64,
        block_0x00203a70,
        block_0x00203a90,
        block_0x00203a9c,
        block_0x00203ab0,
        block_0x00203abc,
        block_0x00203acc,
        block_0x00203ad0,
        block_0x00203ae0,
        block_0x00203afc,
        block_0x00203b08,
        block_0x00203b40,
        block_0x00203b58,
        block_0x00203bac,
        block_0x00203bb0,
        block_0x00203bb4,
        block_0x00203bc4,
        block_0x00203bdc,
        block_0x00203be0,
        block_0x00203c14,
        block_0x00203c50,
        block_0x00203c70,
        block_0x00203c7c,
        block_0x00203ca0,
        block_0x00203cb0,
        block_0x00203cb4,
        block_0x00203cc4,
        block_0x00203ce0,
        block_0x00203cec,
        block_0x00203d28,
        block_0x00203d40,
        block_0x00203d60,
        block_0x00203d80,
        block_0x00203d84,
        block_0x00203da0,
        block_0x00203de0,
        block_0x00203de8,
        block_0x00203df0,
        block_0x00203df8,
        block_0x00203e18,
        block_0x00203e34,
        block_0x00203e4c,
        block_0x00203e60,
        block_0x00203e78,
        block_0x00203e8c,
        block_0x00203ecc,
        block_0x00203ef8,
        block_0x00203efc,
        block_0x00203f64,
        block_0x00203f70,
        block_0x00203f80,
        block_0x00203fb0,
        block_0x00203fbc,
        block_0x00203fd0,
        block_0x00203ff0,
        block_0x00203ffc,
        block_0x0020400c,
        block_0x00204014,
        block_0x0020401c,
        block_0x00204024,
        block_0x0020402c,
        block_0x0020403c,
        block_0x00204044,
        block_0x0020404c,
        block_0x00204054,
        block_0x0020405c,
        block_0x0020407c,
        block_0x00204088,
        block_0x00204094,
        block_0x0020409c,
        block_0x002040a4,
        block_0x002040ac,
        block_0x002040b4,
        block_0x002040c4,
        block_0x002040d0,
        block_0x002040e0,
        block_0x002040e8,
        block_0x002040ec,
        block_0x00204120,
        block_0x00204134,
        block_0x0020415c,
        block_0x00204170,
        block_0x00204190,
        block_0x002041a0,
        block_0x002041b4,
        block_0x002041d4,
        block_0x002041e4,
        block_0x002041f8,
        block_0x00204218,
        block_0x00204228,
        block_0x0020423c,
        block_0x0020425c,
        block_0x0020426c,
        block_0x00204280,
        block_0x0020429c,
        block_0x002042ac,
        block_0x002042c0,
        block_0x002042dc,
        block_0x002042ec,
        block_0x002042f4,
    ];
    const IDX: [u16; 608usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 3u16, 4u16, 0u16, 0u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16,
        13u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 0u16, 0u16,
        17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        23u16, 24u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        33u16, 34u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16,
        0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16,
        45u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16,
        0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16,
        0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 59u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16,
        61u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 65u16, 0u16, 66u16, 0u16, 67u16,
        0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 72u16,
        0u16, 73u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16,
        0u16, 76u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 79u16, 0u16, 80u16, 0u16,
        81u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16,
        0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 108u16,
    ];
    if pc < 2111864u32 || pc > 2114292u32 {
        return None;
    }
    let word_offset = ((pc - 2111864u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00203978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967120u32, 2111868u32);
    emu.sw_no_count(1usize, 2usize, 172u32, 2111872u32)?;
    emu.sw_no_count(8usize, 2usize, 168u32, 2111876u32)?;
    emu.sw_no_count(9usize, 2usize, 164u32, 2111880u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2111884u32)?;
    emu.sw_no_count(19usize, 2usize, 156u32, 2111888u32)?;
    emu.sw_no_count(20usize, 2usize, 152u32, 2111892u32)?;
    emu.sw_no_count(21usize, 2usize, 148u32, 2111896u32)?;
    emu.sw_no_count(22usize, 2usize, 144u32, 2111900u32)?;
    emu.sw_no_count(23usize, 2usize, 140u32, 2111904u32)?;
    emu.sw_no_count(24usize, 2usize, 136u32, 2111908u32)?;
    emu.sw_no_count(25usize, 2usize, 132u32, 2111912u32)?;
    emu.sw_no_count(26usize, 2usize, 128u32, 2111916u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2111920u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2111924u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2111928u32);
    let a = 0u32.wrapping_add(20480u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111932u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965540u32, 2111936u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2111940u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2111948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002039cc));
    } else {
        emu.pc = 2111944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002039c8));
    }
}
#[inline(always)]
pub fn block_0x002039c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2111948u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002039cc));
}
#[inline(always)]
pub fn block_0x002039cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2112208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ad0));
    } else {
        emu.pc = 2111952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002039d0));
    }
}
#[inline(always)]
pub fn block_0x002039d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 20usize, 3u32, 2111956u32);
    emu.sli_no_count(11usize, 20usize, 6u32, 2111960u32);
    emu.sbr_no_count(19usize, 11usize, 10usize, 2111964u32);
    emu.apc_no_count(1usize, 2111964u32, 4096u32, 2111968u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002039e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111976u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2111980u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2111984u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2111988u32);
    emu.apc_no_count(1usize, 2111988u32, 8192u32, 2111992u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002039fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2112320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203b40));
    } else {
        emu.pc = 2112000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203a00));
    }
}
#[inline]
pub fn block_0x00203a00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 12u32, 2112004u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2112008u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2112012u32)?;
    emu.adi_no_count(19usize, 2usize, 80u32, 2112016u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2112020u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966848u32, 2112024u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2112028u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294966904u32, 2112032u32);
    emu.adi_no_count(23usize, 0usize, 3u32, 2112036u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(22usize, a);
    emu.pc = 2112040u32;
    emu.update_insn_clock();
    emu.adi_no_count(22usize, 22usize, 4294966460u32, 2112044u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2112048u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112112u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203a70));
}
#[inline]
pub fn block_0x00203a30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2112052u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2112056u32)?;
    emu.sli_no_count(11usize, 26usize, 3u32, 2112060u32);
    emu.sli_no_count(12usize, 26usize, 6u32, 2112064u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2112068u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2112072u32);
    emu.sw_no_count(25usize, 10usize, 0u32, 2112076u32)?;
    emu.sw_no_count(24usize, 10usize, 4u32, 2112080u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2112084u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2112088u32);
    emu.adi_no_count(12usize, 0usize, 48u32, 2112092u32);
    emu.apc_no_count(1usize, 2112092u32, 8192u32, 2112096u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 26usize, 1u32, 2112104u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2112108u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2112224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ae0));
    } else {
        emu.pc = 2112112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203a70));
    }
}
#[inline(always)]
pub fn block_0x00203a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 72u32, 2112116u32);
    emu.adi_no_count(13usize, 0usize, 17u32, 2112120u32);
    emu.adi_no_count(15usize, 0usize, 4u32, 2112124u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2112128u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2112132u32);
    emu.adi_no_count(14usize, 21usize, 0u32, 2112136u32);
    emu.apc_no_count(1usize, 2112136u32, 4294963200u32, 2112140u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 72u32, 2112148u32)?;
    emu.lw_no_count(24usize, 2usize, 76u32, 2112152u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a == b {
        emu.pc = 2112252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203afc));
    } else {
        emu.pc = 2112156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203a9c));
    }
}
#[inline(always)]
pub fn block_0x00203a9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2112160u32);
    emu.adi_no_count(12usize, 0usize, 48u32, 2112164u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2112168u32);
    emu.apc_no_count(1usize, 2112168u32, 8192u32, 2112172u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2112180u32)?;
    emu.lw_no_count(26usize, 2usize, 20u32, 2112184u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a != b {
        emu.pc = 2112048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203a30));
    } else {
        emu.pc = 2112188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203abc));
    }
}
#[inline(always)]
pub fn block_0x00203abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2112192u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2112196u32);
    emu.apc_no_count(1usize, 2112196u32, 4096u32, 2112200u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966064u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2112208u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112048u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203a30));
}
#[inline(always)]
pub fn block_0x00203ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2112212u32);
    emu.sw_no_count(0usize, 2usize, 12u32, 2112216u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2112220u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2112224u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2112224u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203ae0));
}
#[inline(always)]
pub fn block_0x00203ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2112228u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2112232u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2112236u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2112240u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2112244u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2112248u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2112252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203b08));
}
#[inline(always)]
pub fn block_0x00203afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112256u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2112260u32)?;
    emu.sw_no_count(24usize, 8usize, 4u32, 2112264u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2112264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203b08));
}
#[inline]
pub fn block_0x00203b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 172u32, 2112268u32)?;
    emu.lw_no_count(8usize, 2usize, 168u32, 2112272u32)?;
    emu.lw_no_count(9usize, 2usize, 164u32, 2112276u32)?;
    emu.lw_no_count(18usize, 2usize, 160u32, 2112280u32)?;
    emu.lw_no_count(19usize, 2usize, 156u32, 2112284u32)?;
    emu.lw_no_count(20usize, 2usize, 152u32, 2112288u32)?;
    emu.lw_no_count(21usize, 2usize, 148u32, 2112292u32)?;
    emu.lw_no_count(22usize, 2usize, 144u32, 2112296u32)?;
    emu.lw_no_count(23usize, 2usize, 140u32, 2112300u32)?;
    emu.lw_no_count(24usize, 2usize, 136u32, 2112304u32)?;
    emu.lw_no_count(25usize, 2usize, 132u32, 2112308u32)?;
    emu.lw_no_count(26usize, 2usize, 128u32, 2112312u32)?;
    emu.adi_no_count(2usize, 2usize, 176u32, 2112316u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112320u32;
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
pub fn block_0x00203b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2112324u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966444u32, 2112328u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2112332u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2112336u32);
    emu.apc_no_count(1usize, 2112336u32, 40960u32, 2112340u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2112348u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2112352u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2112356u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2112360u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2112364u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2112368u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2112372u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2112376u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2112380u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2112384u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2112388u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2112392u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2112396u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2112400u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2112404u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2112408u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2112412u32);
    let a = 0u32.wrapping_add(53248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112416u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966476u32, 2112420u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2112424u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2112432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203bb0));
    } else {
        emu.pc = 2112428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203bac));
    }
}
#[inline(always)]
pub fn block_0x00203bac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2112432u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2112432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203bb0));
}
#[inline(always)]
pub fn block_0x00203bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2112692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203cb4));
    } else {
        emu.pc = 2112436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203bb4));
    }
}
#[inline(always)]
pub fn block_0x00203bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2112440u32);
    emu.mul_no_count(19usize, 20usize, 10usize, 2112444u32);
    emu.apc_no_count(1usize, 2112444u32, 4096u32, 2112448u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112456u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966108u32, 2112460u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2112464u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2112468u32);
    emu.apc_no_count(1usize, 2112468u32, 8192u32, 2112472u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112476u32;
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
pub fn block_0x00203bdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2112808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203d28));
    } else {
        emu.pc = 2112480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203be0));
    }
}
#[inline]
pub fn block_0x00203be0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 0u32, 2112484u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2112488u32)?;
    emu.sw_no_count(0usize, 2usize, 8u32, 2112492u32)?;
    emu.adi_no_count(22usize, 2usize, 32u32, 2112496u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2112500u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294966760u32, 2112504u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2112508u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966780u32, 2112512u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2112516u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2112520u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294966460u32, 2112524u32);
    emu.adi_no_count(24usize, 0usize, 20u32, 2112528u32);
    emu.add_memory_rw_events(13usize);
    let return_addr = 2112532u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203c50));
}
#[inline]
pub fn block_0x00203c14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2112536u32)?;
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2112540u32);
    emu.mul_no_count(11usize, 27usize, 24usize, 2112544u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2112548u32);
    emu.lw_no_count(11usize, 2usize, 12u32, 2112552u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2112556u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2112560u32)?;
    emu.adi_no_count(27usize, 27usize, 1u32, 2112564u32);
    emu.sw_no_count(26usize, 10usize, 0u32, 2112568u32)?;
    emu.sw_no_count(25usize, 10usize, 4u32, 2112572u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2112576u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2112580u32)?;
    emu.sw_no_count(13usize, 10usize, 16u32, 2112584u32)?;
    emu.sw_no_count(27usize, 2usize, 8u32, 2112588u32)?;
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2112708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203cc4));
    } else {
        emu.pc = 2112592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c50));
    }
}
#[inline(always)]
pub fn block_0x00203c50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2112596u32);
    emu.adi_no_count(13usize, 0usize, 12u32, 2112600u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2112604u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2112608u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2112612u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2112616u32);
    emu.apc_no_count(1usize, 2112616u32, 4294963200u32, 2112620u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112624u32;
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
pub fn block_0x00203c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 24u32, 2112628u32)?;
    emu.lw_no_count(25usize, 2usize, 28u32, 2112632u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2112736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ce0));
    } else {
        emu.pc = 2112636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c7c));
    }
}
#[inline]
pub fn block_0x00203c7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 22usize, 0u32, 2112640u32)?;
    emu.lw_no_count(11usize, 22usize, 4u32, 2112644u32)?;
    emu.lw_no_count(12usize, 22usize, 8u32, 2112648u32)?;
    emu.lw_no_count(13usize, 2usize, 0u32, 2112652u32)?;
    emu.lw_no_count(27usize, 2usize, 8u32, 2112656u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2112660u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2112664u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2112668u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2112532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c14));
    } else {
        emu.pc = 2112672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ca0));
    }
}
#[inline(always)]
pub fn block_0x00203ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2112676u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2112680u32);
    emu.apc_no_count(1usize, 2112680u32, 4096u32, 2112684u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112688u32;
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
pub fn block_0x00203cb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2112692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203c14));
}
#[inline(always)]
pub fn block_0x00203cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2112696u32);
    emu.sw_no_count(0usize, 2usize, 0u32, 2112700u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2112704u32)?;
    emu.sw_no_count(0usize, 2usize, 8u32, 2112708u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2112708u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203cc4));
}
#[inline(always)]
pub fn block_0x00203cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2112712u32)?;
    emu.lw_no_count(11usize, 2usize, 4u32, 2112716u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2112720u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2112724u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2112728u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2112732u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2112736u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112748u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203cec));
}
#[inline(always)]
pub fn block_0x00203ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112740u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2112744u32)?;
    emu.sw_no_count(25usize, 8usize, 4u32, 2112748u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2112748u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203cec));
}
#[inline]
pub fn block_0x00203cec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 92u32, 2112752u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2112756u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2112760u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2112764u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2112768u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2112772u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2112776u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2112780u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2112784u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2112788u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2112792u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2112796u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2112800u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2112804u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112808u32;
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
pub fn block_0x00203d28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2112812u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966444u32, 2112816u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2112820u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2112824u32);
    emu.apc_no_count(1usize, 2112824u32, 40960u32, 2112828u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203d40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2112836u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2112840u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2112844u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2112848u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2112852u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2112856u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2112860u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2112928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203da0));
    } else {
        emu.pc = 2112864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203d60));
    }
}
#[inline(always)]
pub fn block_0x00203d60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2112868u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112872u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2112876u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2112880u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2112884u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2112888u32);
    emu.apc_no_count(1usize, 2112888u32, 8192u32, 2112892u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 3u32, 2112900u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2112900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203d84));
}
#[inline(always)]
pub fn block_0x00203d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2112904u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2112908u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2112912u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2112916u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2112920u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112924u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112928u32;
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
pub fn block_0x00203da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2112932u32)?;
    emu.lbu_no_count(14usize, 10usize, 0u32, 2112936u32);
    emu.lbu_no_count(15usize, 10usize, 1u32, 2112940u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2112944u32);
    emu.lbu_no_count(16usize, 10usize, 2u32, 2112948u32);
    emu.lbu_no_count(17usize, 10usize, 3u32, 2112952u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2112956u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2112960u32);
    emu.adi_no_count(15usize, 10usize, 4u32, 2112964u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2112968u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2112972u32);
    emu.orr_no_count(16usize, 17usize, 16usize, 2112976u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2112980u32);
    emu.sw_no_count(15usize, 11usize, 0u32, 2112984u32)?;
    emu.sw_no_count(13usize, 11usize, 4u32, 2112988u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2113120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e60));
    } else {
        emu.pc = 2112992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203de0));
    }
}
#[inline(always)]
pub fn block_0x00203de0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2112996u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2113076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e34));
    } else {
        emu.pc = 2113000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203de8));
    }
}
#[inline(always)]
pub fn block_0x00203de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 2u32, 2113004u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2113228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ecc));
    } else {
        emu.pc = 2113008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203df0));
    }
}
#[inline(always)]
pub fn block_0x00203df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 3u32, 2113012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2113164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e8c));
    } else {
        emu.pc = 2113016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203df8));
    }
}
#[inline(always)]
pub fn block_0x00203df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2113020u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113024u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2113028u32);
    emu.sw_no_count(11usize, 2usize, 0u32, 2113032u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2113036u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2113040u32);
    emu.apc_no_count(1usize, 2113040u32, 8192u32, 2113044u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113048u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(52u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2113052u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2113056u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2113060u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2113064u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2113068u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113072u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113076u32;
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
pub fn block_0x00203e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2113080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966552u32, 2113084u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2113088u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113092u32);
    emu.apc_no_count(1usize, 2113092u32, 4294959104u32, 2113096u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203e4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113104u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2113108u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2113112u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113116u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113120u32;
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
pub fn block_0x00203e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2113124u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966492u32, 2113128u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2113132u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113136u32);
    emu.apc_no_count(1usize, 2113136u32, 4294959104u32, 2113140u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113144u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203e78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113148u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2113152u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2113156u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113160u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113164u32;
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
pub fn block_0x00203e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 8u32, 2113168u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2113172u32);
    emu.lbu_no_count(14usize, 10usize, 4u32, 2113176u32);
    emu.lbu_no_count(15usize, 10usize, 5u32, 2113180u32);
    emu.lbu_no_count(16usize, 10usize, 6u32, 2113184u32);
    emu.lbu_no_count(10usize, 10usize, 7u32, 2113188u32);
    emu.sw_no_count(13usize, 11usize, 0u32, 2113192u32)?;
    emu.sw_no_count(12usize, 11usize, 4u32, 2113196u32)?;
    emu.adi_no_count(11usize, 0usize, 2u32, 2113200u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2113204u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2113208u32);
    emu.sli_no_count(10usize, 10usize, 24u32, 2113212u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2113216u32);
    emu.orr_no_count(10usize, 10usize, 16usize, 2113220u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2113224u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2113228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112900u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203d84));
}
#[inline]
pub fn block_0x00203ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2113232u32);
    emu.sb_no_count(10usize, 2usize, 0u32, 2113236u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2113240u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2113244u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113248u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966608u32, 2113252u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2113256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966616u32, 2113260u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2113264u32);
    emu.apc_no_count(1usize, 2113264u32, 0u32, 2113268u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2113276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2112896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203d80));
}
#[inline(never)]
pub fn block_0x00203efc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2113280u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2113284u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2113288u32);
    emu.lw_no_count(15usize, 10usize, 0u32, 2113292u32)?;
    emu.adi_no_count(10usize, 15usize, 4u32, 2113296u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2113300u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2113304u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966952u32, 2113308u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2113312u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2113316u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2113324u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2113328u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966977u32, 2113332u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2113336u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966936u32, 2113340u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2113344u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966988u32, 2113348u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2113352u32);
    emu.adi_no_count(14usize, 0usize, 11u32, 2113356u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2113360u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2113364u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2113368u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2113372u32);
    emu.apc_no_count(1usize, 2113372u32, 65536u32, 2113376u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2113384u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2113388u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113392u32;
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
pub fn block_0x00203f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2113396u32)?;
    emu.lbu_no_count(13usize, 12usize, 0u32, 2113400u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2113404u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2113468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203fbc));
    } else {
        emu.pc = 2113408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203f80));
    }
}
#[inline]
pub fn block_0x00203f80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2113412u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2113416u32)?;
    emu.adi_no_count(12usize, 12usize, 1u32, 2113420u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2113424u32)?;
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113428u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966820u32, 2113432u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2113436u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967000u32, 2113440u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2113444u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2113448u32);
    emu.apc_no_count(1usize, 2113448u32, 65536u32, 2113452u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2113460u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2113464u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113468u32;
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
pub fn block_0x00203fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113472u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966840u32, 2113476u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2113480u32);
    emu.apc_no_count(6usize, 2113480u32, 65536u32, 2113484u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113488u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00203fd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2113492u32)?;
    emu.lw_no_count(10usize, 12usize, 4u32, 2113496u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2113500u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2113504u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2113508u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2113512u32);
    emu.apc_no_count(6usize, 2113512u32, 69632u32, 2113516u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113520u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2113524u32)?;
    emu.apc_no_count(6usize, 2113524u32, 36864u32, 2113528u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113532u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2113536u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2113540u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2113544u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020401c));
    } else {
        emu.pc = 2113548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020400c));
    }
}
#[inline(always)]
pub fn block_0x0020400c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2113552u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204024));
    } else {
        emu.pc = 2113556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204014));
    }
}
#[inline(always)]
pub fn block_0x00204014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113556u32, 45056u32, 2113560u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113564u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(328u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020401c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113564u32, 45056u32, 2113568u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113572u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113572u32, 45056u32, 2113576u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113580u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020402c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2113584u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2113588u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2113592u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020404c));
    } else {
        emu.pc = 2113596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020403c));
    }
}
#[inline(always)]
pub fn block_0x0020403c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2113600u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204054));
    } else {
        emu.pc = 2113604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204044));
    }
}
#[inline(always)]
pub fn block_0x00204044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113604u32, 45056u32, 2113608u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113612u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(80u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020404c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113612u32, 45056u32, 2113616u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113620u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113620u32, 45056u32, 2113624u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113628u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x0020405c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2113632u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2113636u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2113640u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2113644u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2113648u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2113652u32);
    emu.apc_no_count(6usize, 2113652u32, 24576u32, 2113656u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113660u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020407c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2113664u32)?;
    emu.apc_no_count(6usize, 2113664u32, 45056u32, 2113668u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113672u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2113676u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2113680u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040a4));
    } else {
        emu.pc = 2113684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204094));
    }
}
#[inline(always)]
pub fn block_0x00204094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2113688u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2113708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040ac));
    } else {
        emu.pc = 2113692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020409c));
    }
}
#[inline(always)]
pub fn block_0x0020409c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113692u32, 45056u32, 2113696u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113700u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113700u32, 45056u32, 2113704u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113708u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2113708u32, 45056u32, 2113712u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113716u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002040b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2113720u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2113724u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2113728u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2113768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040e8));
    } else {
        emu.pc = 2113732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040c4));
    }
}
#[inline(always)]
pub fn block_0x002040c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2113736u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2113740u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2113768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040e8));
    } else {
        emu.pc = 2113744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040d0));
    }
}
#[inline(always)]
pub fn block_0x002040d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2113748u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2113752u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2113756u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2113768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040e8));
    } else {
        emu.pc = 2113760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002040e0));
    }
}
#[inline(always)]
pub fn block_0x002040e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2113764u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2113768u32;
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
pub fn block_0x002040e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113772u32;
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
pub fn block_0x002040ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2113776u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2113780u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2113784u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2113788u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2113792u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2113796u32)?;
    emu.sw_no_count(20usize, 2usize, 8u32, 2113800u32)?;
    emu.sw_no_count(21usize, 2usize, 4u32, 2113804u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2113808u32);
    emu.lw_no_count(11usize, 11usize, 12u32, 2113812u32)?;
    emu.lw_no_count(18usize, 8usize, 20u32, 2113816u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2113820u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2113844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204134));
    } else {
        emu.pc = 2113824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204120));
    }
}
#[inline(always)]
pub fn block_0x00204120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 12u32, 2113828u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113832u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967016u32, 2113836u32);
    emu.apc_no_count(1usize, 2113836u32, 8192u32, 2113840u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113844u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 16u32, 2113848u32)?;
    emu.sli_no_count(11usize, 18usize, 2u32, 2113852u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2113856u32);
    emu.adi_no_count(11usize, 0usize, 192u32, 2113860u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2113864u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2113868u32)?;
    emu.lw_no_count(20usize, 8usize, 8u32, 2113872u32)?;
    emu.adi_no_count(18usize, 18usize, 1u32, 2113876u32);
    emu.sw_no_count(18usize, 8usize, 20u32, 2113880u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2113904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204170));
    } else {
        emu.pc = 2113884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020415c));
    }
}
#[inline(always)]
pub fn block_0x0020415c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113888u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2113892u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113896u32);
    emu.apc_no_count(1usize, 2113896u32, 28672u32, 2113900u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2113908u32)?;
    emu.adi_no_count(18usize, 9usize, 32u32, 2113912u32);
    emu.sli_no_count(11usize, 20usize, 5u32, 2113916u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2113920u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2113924u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2113928u32);
    emu.apc_no_count(1usize, 2113928u32, 4096u32, 2113932u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113936u32;
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
pub fn block_0x00204190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2113940u32)?;
    emu.adi_no_count(21usize, 20usize, 1u32, 2113944u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2113948u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2113972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041b4));
    } else {
        emu.pc = 2113952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041a0));
    }
}
#[inline(always)]
pub fn block_0x002041a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113956u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2113960u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2113964u32);
    emu.apc_no_count(1usize, 2113964u32, 28672u32, 2113968u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2113976u32)?;
    emu.adi_no_count(19usize, 9usize, 64u32, 2113980u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2113984u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2113988u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2113992u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2113996u32);
    emu.apc_no_count(1usize, 2113996u32, 4096u32, 2114000u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114004u32;
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
pub fn block_0x002041d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2114008u32)?;
    emu.adi_no_count(21usize, 20usize, 2u32, 2114012u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2114016u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2114040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041f8));
    } else {
        emu.pc = 2114020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002041e4));
    }
}
#[inline(always)]
pub fn block_0x002041e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2114024u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2114028u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114032u32);
    emu.apc_no_count(1usize, 2114032u32, 28672u32, 2114036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2114044u32)?;
    emu.adi_no_count(18usize, 9usize, 96u32, 2114048u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2114052u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2114056u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2114060u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2114064u32);
    emu.apc_no_count(1usize, 2114064u32, 4096u32, 2114068u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2114076u32)?;
    emu.adi_no_count(21usize, 20usize, 3u32, 2114080u32);
    emu.sw_no_count(21usize, 8usize, 8u32, 2114084u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2114108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020423c));
    } else {
        emu.pc = 2114088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204228));
    }
}
#[inline(always)]
pub fn block_0x00204228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2114092u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2114096u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114100u32);
    emu.apc_no_count(1usize, 2114100u32, 28672u32, 2114104u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020423c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2114112u32)?;
    emu.adi_no_count(19usize, 9usize, 128u32, 2114116u32);
    emu.sli_no_count(21usize, 21usize, 5u32, 2114120u32);
    emu.adr_no_count(10usize, 10usize, 21usize, 2114124u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2114128u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2114132u32);
    emu.apc_no_count(1usize, 2114132u32, 4096u32, 2114136u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114140u32;
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
pub fn block_0x0020425c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2114144u32)?;
    emu.adi_no_count(18usize, 20usize, 4u32, 2114148u32);
    emu.sw_no_count(18usize, 8usize, 8u32, 2114152u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2114176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204280));
    } else {
        emu.pc = 2114156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020426c));
    }
}
#[inline(always)]
pub fn block_0x0020426c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2114160u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2114164u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114168u32);
    emu.apc_no_count(1usize, 2114168u32, 28672u32, 2114172u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2114180u32)?;
    emu.sli_no_count(18usize, 18usize, 5u32, 2114184u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2114188u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2114192u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2114196u32);
    emu.apc_no_count(1usize, 2114196u32, 4096u32, 2114200u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1032u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020429c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2114208u32)?;
    emu.adi_no_count(18usize, 20usize, 5u32, 2114212u32);
    emu.sw_no_count(18usize, 8usize, 8u32, 2114216u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2114240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002042c0));
    } else {
        emu.pc = 2114220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002042ac));
    }
}
#[inline(always)]
pub fn block_0x002042ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2114224u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2114228u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114232u32);
    emu.apc_no_count(1usize, 2114232u32, 28672u32, 2114236u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2114244u32)?;
    emu.adi_no_count(11usize, 9usize, 160u32, 2114248u32);
    emu.sli_no_count(18usize, 18usize, 5u32, 2114252u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2114256u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2114260u32);
    emu.apc_no_count(1usize, 2114260u32, 4096u32, 2114264u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114268u32;
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
pub fn block_0x002042dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 20u32, 2114272u32)?;
    emu.adi_no_count(20usize, 20usize, 6u32, 2114276u32);
    emu.sw_no_count(20usize, 8usize, 8u32, 2114280u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2114292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002042f4));
    } else {
        emu.pc = 2114284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002042ec));
    }
}
#[inline(always)]
pub fn block_0x002042ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2114288u32);
    emu.sw_no_count(10usize, 8usize, 20u32, 2114292u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2114292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002042f4));
}
#[inline]
pub fn block_0x002042f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2114296u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2114300u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2114304u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2114308u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2114312u32)?;
    emu.lw_no_count(20usize, 2usize, 8u32, 2114316u32)?;
    emu.lw_no_count(21usize, 2usize, 4u32, 2114320u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2114324u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114328u32;
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
