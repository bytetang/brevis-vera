pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2110744u32;
pub const PC_MAX: u32 = 2114620u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 125usize] = [
        block_0x00203518,
        block_0x00203558,
        block_0x0020356c,
        block_0x0020357c,
        block_0x0020358c,
        block_0x00203594,
        block_0x002035a8,
        block_0x002035b8,
        block_0x002035c8,
        block_0x002035d0,
        block_0x002035e4,
        block_0x002035f8,
        block_0x00203608,
        block_0x00203610,
        block_0x0020362c,
        block_0x00203640,
        block_0x00203668,
        block_0x00203678,
        block_0x0020368c,
        block_0x002036f4,
        block_0x00203710,
        block_0x00203718,
        block_0x00203734,
        block_0x0020373c,
        block_0x00203758,
        block_0x00203760,
        block_0x00203778,
        block_0x0020378c,
        block_0x00203794,
        block_0x00203798,
        block_0x002037a4,
        block_0x002037e0,
        block_0x002037e8,
        block_0x002037f4,
        block_0x002037f8,
        block_0x00203810,
        block_0x0020382c,
        block_0x00203834,
        block_0x0020383c,
        block_0x00203848,
        block_0x00203850,
        block_0x00203860,
        block_0x00203868,
        block_0x0020387c,
        block_0x00203884,
        block_0x00203890,
        block_0x00203898,
        block_0x002038a8,
        block_0x002038c4,
        block_0x002038cc,
        block_0x002038d0,
        block_0x002038f4,
        block_0x0020390c,
        block_0x00203924,
        block_0x00203948,
        block_0x00203980,
        block_0x002039d4,
        block_0x002039e8,
        block_0x00203a2c,
        block_0x00203a40,
        block_0x00203ab4,
        block_0x00203ac4,
        block_0x00203afc,
        block_0x00203b4c,
        block_0x00203b58,
        block_0x00203b68,
        block_0x00203b78,
        block_0x00203b94,
        block_0x00203bec,
        block_0x00203c04,
        block_0x00203c1c,
        block_0x00203c58,
        block_0x00203c68,
        block_0x00203c74,
        block_0x00203c88,
        block_0x00203c8c,
        block_0x00203c94,
        block_0x00203ca0,
        block_0x00203cb4,
        block_0x00203ccc,
        block_0x00203ce8,
        block_0x00203d00,
        block_0x00203d2c,
        block_0x00203dc8,
        block_0x00203ddc,
        block_0x00203dec,
        block_0x00203dfc,
        block_0x00203e0c,
        block_0x00203e24,
        block_0x00203e38,
        block_0x00203e4c,
        block_0x00203e60,
        block_0x00203e74,
        block_0x00203e8c,
        block_0x00203e9c,
        block_0x00203ea0,
        block_0x00203eac,
        block_0x00203ebc,
        block_0x0020402c,
        block_0x00204064,
        block_0x00204074,
        block_0x002040ac,
        block_0x00204100,
        block_0x00204154,
        block_0x00204168,
        block_0x002041ac,
        block_0x002041c0,
        block_0x00204234,
        block_0x002042c4,
        block_0x002042d4,
        block_0x002042e4,
        block_0x002042fc,
        block_0x00204304,
        block_0x00204318,
        block_0x0020432c,
        block_0x00204340,
        block_0x00204354,
        block_0x0020436c,
        block_0x00204378,
        block_0x00204394,
        block_0x002043f0,
        block_0x002043fc,
        block_0x0020440c,
        block_0x00204410,
        block_0x0020443c,
    ];
    const IDX: [u16; 970usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 22u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 30u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 33u16, 0u16,
        0u16, 34u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 37u16, 0u16, 38u16, 0u16, 39u16, 0u16, 0u16, 40u16, 0u16, 41u16,
        0u16, 0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 44u16, 0u16, 45u16,
        0u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 65u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16,
        77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16,
        0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16,
        95u16, 96u16, 0u16, 0u16, 97u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16,
        0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 113u16, 0u16, 0u16,
        0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16,
        116u16, 0u16, 0u16, 0u16, 0u16, 117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16,
        0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16, 122u16, 0u16, 0u16, 0u16,
        123u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        125u16,
    ];
    if pc < 2110744u32 || pc > 2114620u32 {
        return None;
    }
    let word_offset = ((pc - 2110744u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00203518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2110748u32);
    emu.sw_no_count(1usize, 2usize, 108u32, 2110752u32)?;
    emu.sw_no_count(8usize, 2usize, 104u32, 2110756u32)?;
    emu.sw_no_count(9usize, 2usize, 100u32, 2110760u32)?;
    emu.sw_no_count(18usize, 2usize, 96u32, 2110764u32)?;
    emu.sw_no_count(19usize, 2usize, 92u32, 2110768u32)?;
    emu.sw_no_count(20usize, 2usize, 88u32, 2110772u32)?;
    emu.sw_no_count(21usize, 2usize, 84u32, 2110776u32)?;
    emu.sw_no_count(22usize, 2usize, 80u32, 2110780u32)?;
    emu.sw_no_count(23usize, 2usize, 76u32, 2110784u32)?;
    emu.sw_no_count(24usize, 2usize, 72u32, 2110788u32)?;
    emu.sw_no_count(25usize, 2usize, 68u32, 2110792u32)?;
    emu.sw_no_count(26usize, 2usize, 64u32, 2110796u32)?;
    emu.sw_no_count(27usize, 2usize, 60u32, 2110800u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2110804u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2111220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002036f4));
    } else {
        emu.pc = 2110808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203558));
    }
}
#[inline(always)]
pub fn block_0x00203558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 15usize, 0u32, 2110812u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2110816u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2110820u32);
    emu.apc_no_count(1usize, 2110820u32, 4294963200u32, 2110824u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110828u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020356c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 36u32, 2110832u32)?;
    emu.lw_no_count(9usize, 2usize, 40u32, 2110836u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110840u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2110860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020358c));
    } else {
        emu.pc = 2110844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020357c));
    }
}
#[inline(always)]
pub fn block_0x0020357c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110848u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110852u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2110856u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110860u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037a4));
}
#[inline(always)]
pub fn block_0x0020358c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2110864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2111256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203718));
    } else {
        emu.pc = 2110868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203594));
    }
}
#[inline(always)]
pub fn block_0x00203594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(24usize, 2usize, 44u32, 2110872u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2110876u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2110880u32);
    emu.apc_no_count(1usize, 2110880u32, 4294963200u32, 2110884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 36u32, 2110892u32)?;
    emu.lw_no_count(18usize, 2usize, 40u32, 2110896u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110900u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a != b {
        emu.pc = 2110920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035c8));
    } else {
        emu.pc = 2110904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035b8));
    }
}
#[inline(always)]
pub fn block_0x002035b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110908u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110912u32)?;
    emu.sw_no_count(18usize, 8usize, 4u32, 2110916u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037a4));
}
#[inline(always)]
pub fn block_0x002035c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2110924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2111292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020373c));
    } else {
        emu.pc = 2110928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035d0));
    }
}
#[inline(always)]
pub fn block_0x002035d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 2usize, 44u32, 2110932u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2110936u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2110940u32);
    emu.apc_no_count(1usize, 2110940u32, 4294963200u32, 2110944u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002035e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(27usize, 2usize, 36u32, 2110952u32)?;
    emu.lw_no_count(21usize, 2usize, 40u32, 2110956u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110960u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2110964u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2110984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203608));
    } else {
        emu.pc = 2110968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035f8));
    }
}
#[inline(always)]
pub fn block_0x002035f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2110972u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2110976u32)?;
    emu.sw_no_count(21usize, 8usize, 4u32, 2110980u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2110984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037a4));
}
#[inline(always)]
pub fn block_0x00203608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2110988u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2111328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203760));
    } else {
        emu.pc = 2110992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203610));
    }
}
#[inline(always)]
pub fn block_0x00203610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(22usize, 2usize, 16u32, 2110996u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2111000u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2111004u32)?;
    emu.adi_no_count(10usize, 2usize, 36u32, 2111008u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111012u32);
    emu.apc_no_count(1usize, 2111012u32, 4294963200u32, 2111016u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020362c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 36u32, 2111024u32)?;
    emu.lw_no_count(22usize, 2usize, 40u32, 2111028u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111032u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1u32, 2111036u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2111384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203798));
    } else {
        emu.pc = 2111040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203640));
    }
}
#[inline]
pub fn block_0x00203640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2111044u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2111048u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2111052u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2111056u32)?;
    emu.adi_no_count(14usize, 0usize, 4u32, 2111060u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2111064u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2111068u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2111072u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2111076u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2111352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203778));
    } else {
        emu.pc = 2111080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203668));
    }
}
#[inline(always)]
pub fn block_0x00203668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2111084u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111088u32);
    emu.apc_no_count(1usize, 2111088u32, 4294963200u32, 2111092u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966852u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 36u32, 2111100u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2111104u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111108u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1u32, 2111112u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
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
        emu.pc = 2111116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020368c));
    }
}
#[inline(never)]
pub fn block_0x0020368c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 44u32, 2111120u32)?;
    emu.lw_no_count(13usize, 2usize, 20u32, 2111124u32)?;
    emu.lw_no_count(14usize, 2usize, 24u32, 2111128u32)?;
    emu.lw_no_count(15usize, 2usize, 28u32, 2111132u32)?;
    emu.lw_no_count(16usize, 2usize, 32u32, 2111136u32)?;
    emu.sw_no_count(23usize, 8usize, 0u32, 2111140u32)?;
    emu.sw_no_count(9usize, 8usize, 4u32, 2111144u32)?;
    emu.sw_no_count(24usize, 8usize, 8u32, 2111148u32)?;
    emu.sw_no_count(25usize, 8usize, 12u32, 2111152u32)?;
    emu.sw_no_count(18usize, 8usize, 16u32, 2111156u32)?;
    emu.lw_no_count(17usize, 2usize, 16u32, 2111160u32)?;
    emu.sw_no_count(17usize, 8usize, 20u32, 2111164u32)?;
    emu.sw_no_count(27usize, 8usize, 24u32, 2111168u32)?;
    emu.sw_no_count(21usize, 8usize, 28u32, 2111172u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2111176u32)?;
    emu.sw_no_count(17usize, 8usize, 32u32, 2111180u32)?;
    emu.sw_no_count(26usize, 8usize, 36u32, 2111184u32)?;
    emu.sw_no_count(22usize, 8usize, 40u32, 2111188u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2111192u32)?;
    emu.sw_no_count(14usize, 8usize, 48u32, 2111196u32)?;
    emu.sw_no_count(15usize, 8usize, 52u32, 2111200u32)?;
    emu.sw_no_count(16usize, 8usize, 56u32, 2111204u32)?;
    emu.sw_no_count(11usize, 8usize, 60u32, 2111208u32)?;
    emu.sw_no_count(10usize, 8usize, 64u32, 2111212u32)?;
    emu.sw_no_count(12usize, 8usize, 68u32, 2111216u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2111220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111396u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037a4));
}
#[inline(always)]
pub fn block_0x002036f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111224u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2111228u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111232u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2111236u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2111240u32);
    emu.apc_no_count(1usize, 2111240u32, 4096u32, 2111244u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2111252u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111256u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110844u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020357c));
}
#[inline(always)]
pub fn block_0x00203718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111260u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2111264u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111268u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2111272u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2111276u32);
    emu.apc_no_count(1usize, 2111276u32, 4096u32, 2111280u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203734(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2111288u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110904u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002035b8));
}
#[inline(always)]
pub fn block_0x0020373c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111296u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2111300u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111304u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2111308u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2111312u32);
    emu.apc_no_count(1usize, 2111312u32, 4096u32, 2111316u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111320u32;
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
#[inline(always)]
pub fn block_0x00203758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 10usize, 0u32, 2111324u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111328u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002035f8));
}
#[inline(always)]
pub fn block_0x00203760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111332u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2111336u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111340u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2111344u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2111348u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2111352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111372u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020378c));
}
#[inline(always)]
pub fn block_0x00203778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111356u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966968u32, 2111360u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111364u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966872u32, 2111368u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2111372u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2111372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020378c));
}
#[inline(always)]
pub fn block_0x0020378c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2111372u32, 4096u32, 2111376u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 10usize, 0u32, 2111384u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203798));
}
#[inline(always)]
pub fn block_0x00203798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111388u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2111392u32)?;
    emu.sw_no_count(22usize, 8usize, 4u32, 2111396u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2111396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037a4));
}
#[inline]
pub fn block_0x002037a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 108u32, 2111400u32)?;
    emu.lw_no_count(8usize, 2usize, 104u32, 2111404u32)?;
    emu.lw_no_count(9usize, 2usize, 100u32, 2111408u32)?;
    emu.lw_no_count(18usize, 2usize, 96u32, 2111412u32)?;
    emu.lw_no_count(19usize, 2usize, 92u32, 2111416u32)?;
    emu.lw_no_count(20usize, 2usize, 88u32, 2111420u32)?;
    emu.lw_no_count(21usize, 2usize, 84u32, 2111424u32)?;
    emu.lw_no_count(22usize, 2usize, 80u32, 2111428u32)?;
    emu.lw_no_count(23usize, 2usize, 76u32, 2111432u32)?;
    emu.lw_no_count(24usize, 2usize, 72u32, 2111436u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2111440u32)?;
    emu.lw_no_count(26usize, 2usize, 64u32, 2111444u32)?;
    emu.lw_no_count(27usize, 2usize, 60u32, 2111448u32)?;
    emu.adi_no_count(2usize, 2usize, 112u32, 2111452u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111456u32;
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
pub fn block_0x002037e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2111460u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2111540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203834));
    } else {
        emu.pc = 2111464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002037e8));
    }
}
#[inline(always)]
pub fn block_0x002037e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2111468u32)?;
    emu.adi_no_count(11usize, 14usize, 0u32, 2111472u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2111480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002037f8));
    } else {
        emu.pc = 2111476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002037f4));
    }
}
#[inline(always)]
pub fn block_0x002037f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 13usize, 0u32, 2111480u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002037f8));
}
#[inline(always)]
pub fn block_0x002037f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 10usize, 0u32, 2111484u32)?;
    emu.sbr_no_count(13usize, 13usize, 11usize, 2111488u32);
    emu.adr_no_count(15usize, 17usize, 11usize, 2111492u32);
    emu.sw_no_count(15usize, 10usize, 0u32, 2111496u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2111500u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2111732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038f4));
    } else {
        emu.pc = 2111504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203810));
    }
}
#[inline(always)]
pub fn block_0x00203810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 17usize, 0u32, 2111508u32);
    emu.lw_no_count(14usize, 10usize, 12u32, 2111512u32)?;
    emu.adi_no_count(13usize, 16usize, 4294967231u32, 2111516u32);
    emu.ani_no_count(5usize, 13usize, 255u32, 2111520u32);
    emu.adi_no_count(13usize, 0usize, 6u32, 2111524u32);
    emu.sli_no_count(15usize, 14usize, 1u32, 2111528u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2111548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020383c));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 16usize, 4294967241u32, 2111536u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203860));
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
    emu.adi_no_count(10usize, 0usize, 2u32, 2111544u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111548u32;
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
pub fn block_0x0020383c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 16usize, 4294967199u32, 2111552u32);
    emu.ani_no_count(5usize, 5usize, 255u32, 2111556u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2111568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203850));
    } else {
        emu.pc = 2111560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203848));
    }
}
#[inline(always)]
pub fn block_0x00203848(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 16usize, 4294967209u32, 2111564u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203860));
}
#[inline(always)]
pub fn block_0x00203850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 16usize, 4294967248u32, 2111572u32);
    emu.ani_no_count(5usize, 13usize, 255u32, 2111576u32);
    emu.adi_no_count(6usize, 0usize, 10u32, 2111580u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2111684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c4));
    } else {
        emu.pc = 2111584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203860));
    }
}
#[inline(always)]
pub fn block_0x00203860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 1u32, 2111588u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2111756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020390c));
    } else {
        emu.pc = 2111592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203868));
    }
}
#[inline(always)]
pub fn block_0x00203868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 17usize, 1u32, 2111596u32);
    emu.adi_no_count(11usize, 16usize, 4294967231u32, 2111600u32);
    emu.ani_no_count(17usize, 11usize, 255u32, 2111604u32);
    emu.adi_no_count(11usize, 0usize, 6u32, 2111608u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2111620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203884));
    } else {
        emu.pc = 2111612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020387c));
    }
}
#[inline(always)]
pub fn block_0x0020387c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 16usize, 4294967241u32, 2111616u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111620u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038a8));
}
#[inline(always)]
pub fn block_0x00203884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 4294967199u32, 2111624u32);
    emu.ani_no_count(17usize, 17usize, 255u32, 2111628u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2111640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203898));
    } else {
        emu.pc = 2111632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203890));
    }
}
#[inline(always)]
pub fn block_0x00203890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 16usize, 4294967209u32, 2111636u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111640u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038a8));
}
#[inline(always)]
pub fn block_0x00203898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 16usize, 4294967248u32, 2111644u32);
    emu.ani_no_count(17usize, 11usize, 255u32, 2111648u32);
    emu.adi_no_count(5usize, 0usize, 10u32, 2111652u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2111692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038cc));
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
    emu.sli_no_count(13usize, 13usize, 4u32, 2111660u32);
    emu.orr_no_count(11usize, 11usize, 13usize, 2111664u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2111668u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2111672u32);
    emu.sw_no_count(14usize, 10usize, 12u32, 2111676u32)?;
    emu.adi_no_count(10usize, 13usize, 0u32, 2111680u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111684u32;
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
pub fn block_0x002038c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 15usize, 0u32, 2111688u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2111692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111696u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038d0));
}
#[inline(always)]
pub fn block_0x002038cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ori_no_count(11usize, 15usize, 1u32, 2111696u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038d0));
}
#[inline]
pub fn block_0x002038d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(15usize, 15usize, 4294967040u32, 2111700u32);
    emu.ani_no_count(17usize, 11usize, 255u32, 2111704u32);
    emu.orr_no_count(15usize, 15usize, 17usize, 2111708u32);
    emu.sw_no_count(16usize, 12usize, 0u32, 2111712u32)?;
    emu.sw_no_count(15usize, 12usize, 4u32, 2111716u32)?;
    emu.adi_no_count(14usize, 14usize, 1u32, 2111720u32);
    emu.sw_no_count(14usize, 10usize, 12u32, 2111724u32)?;
    emu.adi_no_count(10usize, 0usize, 0u32, 2111728u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111732u32;
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
pub fn block_0x002038f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111736u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 288u32, 2111740u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2111744u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2111748u32);
    emu.apc_no_count(1usize, 2111748u32, 106496u32, 2111752u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020390c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111760u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 304u32, 2111764u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2111768u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2111772u32);
    emu.apc_no_count(1usize, 2111772u32, 106496u32, 2111776u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2111784u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2111788u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2111792u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2111796u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2111800u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2111804u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2111808u32);
    emu.apc_no_count(1usize, 2111808u32, 57344u32, 2111812u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 92u32, 2111820u32)?;
    emu.sw_no_count(0usize, 2usize, 96u32, 2111824u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2111828u32)?;
    emu.sw_no_count(0usize, 2usize, 104u32, 2111832u32)?;
    emu.lbu_no_count(13usize, 2usize, 40u32, 2111836u32);
    emu.sw_no_count(0usize, 2usize, 76u32, 2111840u32)?;
    emu.sw_no_count(0usize, 2usize, 80u32, 2111844u32)?;
    emu.sw_no_count(0usize, 2usize, 84u32, 2111848u32)?;
    emu.sw_no_count(0usize, 2usize, 88u32, 2111852u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2111856u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2111860u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2111864u32);
    emu.apc_no_count(1usize, 2111864u32, 61440u32, 2111868u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 60u32, 2111876u32)?;
    emu.lw_no_count(11usize, 2usize, 64u32, 2111880u32)?;
    emu.lw_no_count(12usize, 2usize, 68u32, 2111884u32)?;
    emu.lw_no_count(13usize, 2usize, 72u32, 2111888u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2111892u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2111896u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2111900u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2111904u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2111908u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2111912u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2111916u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2111920u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2111924u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2111928u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2111932u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2111936u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2111940u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2111944u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2111948u32);
    emu.apc_no_count(1usize, 2111948u32, 32768u32, 2111952u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111956u32;
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
pub fn block_0x002039d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 308u32, 2111960u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2111964u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2111968u32);
    emu.apc_no_count(1usize, 2111968u32, 32768u32, 2111972u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111976u32;
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
#[inline]
pub fn block_0x002039e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2111980u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2111984u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2111988u32);
    emu.adi_no_count(13usize, 0usize, 4294967295u32, 2111992u32);
    emu.sw_no_count(0usize, 2usize, 192u32, 2111996u32)?;
    emu.sw_no_count(0usize, 2usize, 196u32, 2112000u32)?;
    emu.sw_no_count(11usize, 2usize, 200u32, 2112004u32)?;
    emu.sw_no_count(10usize, 2usize, 204u32, 2112008u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2112012u32)?;
    emu.sw_no_count(13usize, 2usize, 180u32, 2112016u32)?;
    emu.sw_no_count(13usize, 2usize, 184u32, 2112020u32)?;
    emu.sw_no_count(12usize, 2usize, 188u32, 2112024u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2112028u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2112032u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2112036u32);
    emu.apc_no_count(1usize, 2112036u32, 32768u32, 2112040u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 276u32, 2112048u32);
    emu.adi_no_count(11usize, 2usize, 308u32, 2112052u32);
    emu.adi_no_count(12usize, 2usize, 140u32, 2112056u32);
    emu.apc_no_count(1usize, 2112056u32, 28672u32, 2112060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112064u32;
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
#[inline(never)]
pub fn block_0x00203a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112068u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112072u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2112076u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112080u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2112084u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2112088u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2112092u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1565u32, 2112096u32);
    emu.adi_no_count(11usize, 11usize, 4294965300u32, 2112100u32);
    emu.adi_no_count(12usize, 12usize, 171u32, 2112104u32);
    emu.adi_no_count(13usize, 13usize, 4294966998u32, 2112108u32);
    emu.sw_no_count(13usize, 2usize, 192u32, 2112112u32)?;
    emu.sw_no_count(12usize, 2usize, 196u32, 2112116u32)?;
    emu.sw_no_count(11usize, 2usize, 200u32, 2112120u32)?;
    emu.sw_no_count(10usize, 2usize, 204u32, 2112124u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2112128u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 1485u32, 2112132u32);
    emu.adi_no_count(12usize, 15usize, 144u32, 2112136u32);
    emu.adi_no_count(13usize, 16usize, 4294967138u32, 2112140u32);
    emu.adi_no_count(10usize, 10usize, 4294966751u32, 2112144u32);
    emu.sw_no_count(10usize, 2usize, 176u32, 2112148u32)?;
    emu.sw_no_count(13usize, 2usize, 180u32, 2112152u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2112156u32)?;
    emu.sw_no_count(11usize, 2usize, 188u32, 2112160u32)?;
    emu.adi_no_count(10usize, 2usize, 108u32, 2112164u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2112168u32);
    emu.adi_no_count(12usize, 2usize, 176u32, 2112172u32);
    emu.apc_no_count(1usize, 2112172u32, 28672u32, 2112176u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2112184u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2112188u32);
    emu.apc_no_count(1usize, 2112188u32, 61440u32, 2112192u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 292u32, 2112200u32)?;
    emu.sw_no_count(0usize, 2usize, 296u32, 2112204u32)?;
    emu.sw_no_count(0usize, 2usize, 300u32, 2112208u32)?;
    emu.sw_no_count(0usize, 2usize, 304u32, 2112212u32)?;
    emu.lbu_no_count(13usize, 2usize, 172u32, 2112216u32);
    emu.sw_no_count(0usize, 2usize, 276u32, 2112220u32)?;
    emu.sw_no_count(0usize, 2usize, 280u32, 2112224u32)?;
    emu.sw_no_count(0usize, 2usize, 284u32, 2112228u32)?;
    emu.sw_no_count(0usize, 2usize, 288u32, 2112232u32)?;
    emu.adi_no_count(10usize, 2usize, 244u32, 2112236u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2112240u32);
    emu.adi_no_count(12usize, 2usize, 140u32, 2112244u32);
    emu.apc_no_count(1usize, 2112244u32, 61440u32, 2112248u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112252u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 260u32, 2112256u32)?;
    emu.lw_no_count(11usize, 2usize, 264u32, 2112260u32)?;
    emu.lw_no_count(12usize, 2usize, 268u32, 2112264u32)?;
    emu.lw_no_count(13usize, 2usize, 272u32, 2112268u32)?;
    emu.sw_no_count(10usize, 2usize, 192u32, 2112272u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2112276u32)?;
    emu.sw_no_count(12usize, 2usize, 200u32, 2112280u32)?;
    emu.sw_no_count(13usize, 2usize, 204u32, 2112284u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2112288u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2112292u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2112296u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2112300u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2112304u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2112308u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2112312u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2112316u32)?;
    emu.adi_no_count(10usize, 2usize, 308u32, 2112320u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2112324u32);
    emu.apc_no_count(1usize, 2112324u32, 36864u32, 2112328u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203b4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 244u32, 2112336u32);
    emu.apc_no_count(1usize, 2112336u32, 57344u32, 2112340u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112344u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2000u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 10usize, 9usize, 2112348u32);
    emu.ani_no_count(10usize, 10usize, 255u32, 2112352u32);
    emu.apc_no_count(1usize, 2112352u32, 28672u32, 2112356u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112360u32;
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
pub fn block_0x00203b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2112364u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2112368u32);
    emu.apc_no_count(1usize, 2112368u32, 28672u32, 2112372u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112376u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 2usize, 208u32, 2112380u32);
    emu.ani_no_count(13usize, 10usize, 255u32, 2112384u32);
    emu.adi_no_count(11usize, 2usize, 308u32, 2112388u32);
    emu.adi_no_count(12usize, 2usize, 244u32, 2112392u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2112396u32);
    emu.apc_no_count(1usize, 2112396u32, 61440u32, 2112400u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203b94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2112408u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2112412u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2112416u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2112420u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2112424u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2112428u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2112432u32)?;
    emu.lw_no_count(17usize, 2usize, 72u32, 2112436u32)?;
    emu.lbu_no_count(5usize, 2usize, 172u32, 2112440u32);
    emu.sw_no_count(10usize, 2usize, 176u32, 2112444u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2112448u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2112452u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2112456u32)?;
    emu.lbu_no_count(10usize, 2usize, 40u32, 2112460u32);
    emu.sw_no_count(14usize, 2usize, 192u32, 2112464u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2112468u32)?;
    emu.sw_no_count(16usize, 2usize, 200u32, 2112472u32)?;
    emu.sw_no_count(17usize, 2usize, 204u32, 2112476u32)?;
    emu.sb_no_count(0usize, 2usize, 240u32, 2112480u32);
    emu.anr_no_count(10usize, 10usize, 5usize, 2112484u32);
    emu.apc_no_count(1usize, 2112484u32, 28672u32, 2112488u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112492u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203bec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2112496u32);
    emu.adi_no_count(11usize, 2usize, 176u32, 2112500u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2112504u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2112508u32);
    emu.apc_no_count(1usize, 2112508u32, 28672u32, 2112512u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112516u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 68u32, 2112520u32);
    emu.lw_no_count(1usize, 2usize, 348u32, 2112524u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2112528u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2112532u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2112536u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112540u32;
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
pub fn block_0x00203c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966688u32, 2112544u32);
    emu.sw_no_count(1usize, 2usize, 604u32, 2112548u32)?;
    emu.sw_no_count(8usize, 2usize, 600u32, 2112552u32)?;
    emu.sw_no_count(9usize, 2usize, 596u32, 2112556u32)?;
    emu.sw_no_count(18usize, 2usize, 592u32, 2112560u32)?;
    emu.sw_no_count(19usize, 2usize, 588u32, 2112564u32)?;
    emu.sw_no_count(20usize, 2usize, 584u32, 2112568u32)?;
    emu.sw_no_count(21usize, 2usize, 580u32, 2112572u32)?;
    emu.sw_no_count(22usize, 2usize, 576u32, 2112576u32)?;
    emu.sw_no_count(23usize, 2usize, 572u32, 2112580u32)?;
    emu.lbu_no_count(12usize, 11usize, 0u32, 2112584u32);
    emu.adi_no_count(13usize, 12usize, 4294967294u32, 2112588u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2112592u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2112596u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2113180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e9c));
    } else {
        emu.pc = 2112600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c58));
    }
}
#[inline(always)]
pub fn block_0x00203c58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 6u32, 2112604u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2112608u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2112612u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2112652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c8c));
    } else {
        emu.pc = 2112616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c68));
    }
}
#[inline(always)]
pub fn block_0x00203c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 1u32, 2112620u32);
    emu.apc_no_count(1usize, 2112620u32, 28672u32, 2112624u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203c74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2112632u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2112636u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2112640u32);
    emu.apc_no_count(1usize, 2112640u32, 0u32, 2112644u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203c88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2112652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204410));
}
#[inline(always)]
pub fn block_0x00203c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 5u32, 2112656u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2113212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ebc));
    } else {
        emu.pc = 2112660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203c94));
    }
}
#[inline(always)]
pub fn block_0x00203c94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2112664u32);
    emu.apc_no_count(1usize, 2112664u32, 28672u32, 2112668u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(84u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2112676u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2112680u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2112684u32);
    emu.apc_no_count(1usize, 2112684u32, 0u32, 2112688u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112692u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 124u32, 2112700u32);
    emu.adi_no_count(10usize, 2usize, 276u32, 2112704u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2112708u32);
    emu.apc_no_count(1usize, 2112708u32, 28672u32, 2112712u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 72u32, 2112720u32);
    emu.adi_no_count(10usize, 2usize, 212u32, 2112724u32);
    emu.adi_no_count(11usize, 2usize, 276u32, 2112728u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2112732u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2112736u32);
    emu.apc_no_count(1usize, 2112736u32, 61440u32, 2112740u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 308u32, 2112748u32);
    emu.adi_no_count(12usize, 2usize, 36u32, 2112752u32);
    emu.adi_no_count(10usize, 2usize, 244u32, 2112756u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2112760u32);
    emu.apc_no_count(1usize, 2112760u32, 61440u32, 2112764u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 340u32, 2112772u32);
    emu.lbu_no_count(11usize, 2usize, 68u32, 2112776u32);
    emu.sbr_no_count(12usize, 0usize, 9usize, 2112780u32);
    emu.xrr_no_count(11usize, 11usize, 10usize, 2112784u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2112788u32);
    emu.xrr_no_count(19usize, 11usize, 10usize, 2112792u32);
    emu.adi_no_count(10usize, 2usize, 144u32, 2112796u32);
    emu.adi_no_count(11usize, 2usize, 212u32, 2112800u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2112804u32);
    emu.apc_no_count(1usize, 2112804u32, 24576u32, 2112808u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00203d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 2usize, 208u32, 2112816u32);
    emu.lw_no_count(10usize, 2usize, 228u32, 2112820u32)?;
    emu.lw_no_count(11usize, 2usize, 232u32, 2112824u32)?;
    emu.lw_no_count(12usize, 2usize, 236u32, 2112828u32)?;
    emu.lw_no_count(13usize, 2usize, 240u32, 2112832u32)?;
    emu.lw_no_count(14usize, 2usize, 212u32, 2112836u32)?;
    emu.lw_no_count(15usize, 2usize, 216u32, 2112840u32)?;
    emu.lw_no_count(16usize, 2usize, 220u32, 2112844u32)?;
    emu.lw_no_count(17usize, 2usize, 224u32, 2112848u32)?;
    emu.sw_no_count(10usize, 2usize, 360u32, 2112852u32)?;
    emu.sw_no_count(11usize, 2usize, 364u32, 2112856u32)?;
    emu.sw_no_count(12usize, 2usize, 368u32, 2112860u32)?;
    emu.sw_no_count(13usize, 2usize, 372u32, 2112864u32)?;
    emu.adi_no_count(9usize, 2usize, 176u32, 2112868u32);
    emu.sw_no_count(14usize, 2usize, 344u32, 2112872u32)?;
    emu.sw_no_count(15usize, 2usize, 348u32, 2112876u32)?;
    emu.sw_no_count(16usize, 2usize, 352u32, 2112880u32)?;
    emu.sw_no_count(17usize, 2usize, 356u32, 2112884u32)?;
    emu.lw_no_count(10usize, 2usize, 244u32, 2112888u32)?;
    emu.lw_no_count(11usize, 2usize, 248u32, 2112892u32)?;
    emu.lw_no_count(12usize, 2usize, 252u32, 2112896u32)?;
    emu.lw_no_count(13usize, 2usize, 256u32, 2112900u32)?;
    emu.sw_no_count(10usize, 2usize, 540u32, 2112904u32)?;
    emu.sw_no_count(11usize, 2usize, 544u32, 2112908u32)?;
    emu.sw_no_count(12usize, 2usize, 548u32, 2112912u32)?;
    emu.sw_no_count(13usize, 2usize, 552u32, 2112916u32)?;
    emu.lw_no_count(10usize, 2usize, 260u32, 2112920u32)?;
    emu.lw_no_count(11usize, 2usize, 264u32, 2112924u32)?;
    emu.lw_no_count(12usize, 2usize, 268u32, 2112928u32)?;
    emu.lw_no_count(13usize, 2usize, 272u32, 2112932u32)?;
    emu.sw_no_count(10usize, 2usize, 556u32, 2112936u32)?;
    emu.sw_no_count(11usize, 2usize, 560u32, 2112940u32)?;
    emu.sw_no_count(12usize, 2usize, 564u32, 2112944u32)?;
    emu.sw_no_count(13usize, 2usize, 568u32, 2112948u32)?;
    emu.adi_no_count(18usize, 2usize, 376u32, 2112952u32);
    emu.adi_no_count(11usize, 2usize, 540u32, 2112956u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2112960u32);
    emu.apc_no_count(1usize, 2112960u32, 32768u32, 2112964u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(19usize, 2usize, 408u32, 2112972u32);
    emu.adi_no_count(10usize, 2usize, 476u32, 2112976u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2112980u32);
    emu.apc_no_count(1usize, 2112980u32, 57344u32, 2112984u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(12u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 444u32, 2112992u32);
    emu.adi_no_count(11usize, 2usize, 476u32, 2112996u32);
    emu.apc_no_count(1usize, 2112996u32, 61440u32, 2113000u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 76u32, 2113008u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2113012u32);
    emu.apc_no_count(1usize, 2113012u32, 57344u32, 2113016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 508u32, 2113024u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2113028u32);
    emu.apc_no_count(1usize, 2113028u32, 61440u32, 2113032u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113036u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 444u32, 2113040u32)?;
    emu.lw_no_count(13usize, 2usize, 508u32, 2113044u32)?;
    emu.lw_no_count(11usize, 2usize, 512u32, 2113048u32)?;
    emu.lw_no_count(10usize, 2usize, 516u32, 2113052u32)?;
    emu.sltru_no_count(12usize, 13usize, 12usize, 2113056u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2114308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204304));
    } else {
        emu.pc = 2113060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e24));
    }
}
#[inline(always)]
pub fn block_0x00203e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2113064u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2113068u32);
    emu.slti_no_count(12usize, 11usize, 0u32, 2113072u32);
    emu.lw_no_count(11usize, 2usize, 520u32, 2113076u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2114328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204318));
    } else {
        emu.pc = 2113080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e38));
    }
}
#[inline(always)]
pub fn block_0x00203e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 452u32, 2113084u32)?;
    emu.sbr_no_count(10usize, 10usize, 12usize, 2113088u32);
    emu.sltru_no_count(12usize, 10usize, 13usize, 2113092u32);
    emu.lw_no_count(10usize, 2usize, 524u32, 2113096u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2114348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020432c));
    } else {
        emu.pc = 2113100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e4c));
    }
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
    emu.lw_no_count(13usize, 2usize, 456u32, 2113104u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2113108u32);
    emu.sltru_no_count(12usize, 11usize, 13usize, 2113112u32);
    emu.lw_no_count(11usize, 2usize, 528u32, 2113116u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2114368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204340));
    } else {
        emu.pc = 2113120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e60));
    }
}
#[inline(always)]
pub fn block_0x00203e60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 460u32, 2113124u32)?;
    emu.sbr_no_count(10usize, 10usize, 12usize, 2113128u32);
    emu.sltru_no_count(12usize, 10usize, 13usize, 2113132u32);
    emu.lw_no_count(10usize, 2usize, 532u32, 2113136u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2114388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204354));
    } else {
        emu.pc = 2113140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e74));
    }
}
#[inline(always)]
pub fn block_0x00203e74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 464u32, 2113144u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2113148u32);
    emu.sltru_no_count(13usize, 11usize, 13usize, 2113152u32);
    emu.lw_no_count(11usize, 2usize, 536u32, 2113156u32)?;
    emu.lw_no_count(12usize, 2usize, 472u32, 2113160u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2114412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020436c));
    } else {
        emu.pc = 2113164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e8c));
    }
}
#[inline(always)]
pub fn block_0x00203e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 2usize, 468u32, 2113168u32)?;
    emu.sbr_no_count(10usize, 10usize, 13usize, 2113172u32);
    emu.sltru_no_count(10usize, 10usize, 14usize, 2113176u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2113180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204378));
}
#[inline(always)]
pub fn block_0x00203e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2114620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020443c));
    } else {
        emu.pc = 2113184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203ea0));
    }
}
#[inline(always)]
pub fn block_0x00203ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2113188u32);
    emu.apc_no_count(1usize, 2113188u32, 28672u32, 2113192u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2113200u32);
    let a = 0u32.wrapping_add(2260992u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 124u32, 2113208u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2113212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002043fc));
}
#[inline(never)]
pub fn block_0x00203ebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 92u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 11usize, 61u32, 2113216u32);
    emu.lbu_no_count(12usize, 11usize, 62u32, 2113220u32);
    emu.lbu_no_count(13usize, 11usize, 63u32, 2113224u32);
    emu.lbu_no_count(14usize, 11usize, 64u32, 2113228u32);
    emu.lbu_no_count(15usize, 11usize, 57u32, 2113232u32);
    emu.lbu_no_count(16usize, 11usize, 58u32, 2113236u32);
    emu.lbu_no_count(17usize, 11usize, 59u32, 2113240u32);
    emu.lbu_no_count(5usize, 11usize, 60u32, 2113244u32);
    emu.lbu_no_count(6usize, 11usize, 53u32, 2113248u32);
    emu.lbu_no_count(7usize, 11usize, 54u32, 2113252u32);
    emu.lbu_no_count(28usize, 11usize, 55u32, 2113256u32);
    emu.lbu_no_count(29usize, 11usize, 56u32, 2113260u32);
    emu.lbu_no_count(30usize, 11usize, 49u32, 2113264u32);
    emu.lbu_no_count(31usize, 11usize, 50u32, 2113268u32);
    emu.lbu_no_count(18usize, 11usize, 51u32, 2113272u32);
    emu.lbu_no_count(19usize, 11usize, 52u32, 2113276u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2113280u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2113284u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2113288u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2113292u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2113296u32);
    emu.orr_no_count(12usize, 14usize, 13usize, 2113300u32);
    emu.orr_no_count(13usize, 16usize, 15usize, 2113304u32);
    emu.lbu_no_count(20usize, 11usize, 45u32, 2113308u32);
    emu.lbu_no_count(21usize, 11usize, 46u32, 2113312u32);
    emu.lbu_no_count(22usize, 11usize, 47u32, 2113316u32);
    emu.lbu_no_count(23usize, 11usize, 48u32, 2113320u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2113324u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2113328u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2113332u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2113336u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2113340u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2113344u32);
    emu.orr_no_count(14usize, 5usize, 17usize, 2113348u32);
    emu.orr_no_count(15usize, 7usize, 6usize, 2113352u32);
    emu.orr_no_count(16usize, 29usize, 28usize, 2113356u32);
    emu.orr_no_count(17usize, 31usize, 30usize, 2113360u32);
    emu.lbu_no_count(5usize, 11usize, 41u32, 2113364u32);
    emu.lbu_no_count(6usize, 11usize, 42u32, 2113368u32);
    emu.lbu_no_count(7usize, 11usize, 43u32, 2113372u32);
    emu.lbu_no_count(28usize, 11usize, 44u32, 2113376u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2113380u32);
    emu.sli_no_count(19usize, 19usize, 24u32, 2113384u32);
    emu.sli_no_count(21usize, 21usize, 8u32, 2113388u32);
    emu.sli_no_count(22usize, 22usize, 16u32, 2113392u32);
    emu.sli_no_count(23usize, 23usize, 24u32, 2113396u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2113400u32);
    emu.orr_no_count(29usize, 19usize, 18usize, 2113404u32);
    emu.orr_no_count(30usize, 21usize, 20usize, 2113408u32);
    emu.orr_no_count(31usize, 23usize, 22usize, 2113412u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2113416u32);
    emu.lbu_no_count(6usize, 11usize, 37u32, 2113420u32);
    emu.lbu_no_count(18usize, 11usize, 38u32, 2113424u32);
    emu.lbu_no_count(19usize, 11usize, 39u32, 2113428u32);
    emu.lbu_no_count(20usize, 11usize, 40u32, 2113432u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2113436u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2113440u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2113444u32);
    emu.sli_no_count(19usize, 19usize, 16u32, 2113448u32);
    emu.sli_no_count(20usize, 20usize, 24u32, 2113452u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2113456u32);
    emu.orr_no_count(6usize, 18usize, 6usize, 2113460u32);
    emu.lbu_no_count(28usize, 11usize, 33u32, 2113464u32);
    emu.lbu_no_count(18usize, 11usize, 34u32, 2113468u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2113472u32);
    emu.lbu_no_count(20usize, 11usize, 35u32, 2113476u32);
    emu.lbu_no_count(11usize, 11usize, 36u32, 2113480u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2113484u32);
    emu.orr_no_count(28usize, 18usize, 28usize, 2113488u32);
    emu.sli_no_count(20usize, 20usize, 16u32, 2113492u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2113496u32);
    emu.orr_no_count(11usize, 11usize, 20usize, 2113500u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2113504u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2113508u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2113512u32);
    emu.orr_no_count(14usize, 29usize, 17usize, 2113516u32);
    emu.orr_no_count(15usize, 31usize, 30usize, 2113520u32);
    emu.orr_no_count(16usize, 7usize, 5usize, 2113524u32);
    emu.orr_no_count(17usize, 19usize, 6usize, 2113528u32);
    emu.orr_no_count(11usize, 11usize, 28usize, 2113532u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2113536u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2113540u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2113544u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2113548u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2113552u32)?;
    emu.sw_no_count(17usize, 2usize, 8u32, 2113556u32)?;
    emu.sw_no_count(16usize, 2usize, 12u32, 2113560u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2113564u32)?;
    emu.adi_no_count(10usize, 2usize, 276u32, 2113568u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2113572u32);
    emu.apc_no_count(1usize, 2113572u32, 57344u32, 2113576u32);
    emu.add_memory_rw_events(92usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966044u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020402c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 460u32, 2113584u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2113588u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2113592u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2113596u32)?;
    emu.lbu_no_count(13usize, 2usize, 308u32, 2113600u32);
    emu.sw_no_count(0usize, 2usize, 444u32, 2113604u32)?;
    emu.sw_no_count(0usize, 2usize, 448u32, 2113608u32)?;
    emu.sw_no_count(0usize, 2usize, 452u32, 2113612u32)?;
    emu.sw_no_count(0usize, 2usize, 456u32, 2113616u32)?;
    emu.adi_no_count(10usize, 2usize, 412u32, 2113620u32);
    emu.adi_no_count(11usize, 2usize, 444u32, 2113624u32);
    emu.adi_no_count(12usize, 2usize, 276u32, 2113628u32);
    emu.apc_no_count(1usize, 2113628u32, 57344u32, 2113632u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 344u32, 2113640u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2113644u32);
    emu.apc_no_count(1usize, 2113644u32, 57344u32, 2113648u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 524u32, 2113656u32)?;
    emu.sw_no_count(0usize, 2usize, 528u32, 2113660u32)?;
    emu.sw_no_count(0usize, 2usize, 532u32, 2113664u32)?;
    emu.sw_no_count(0usize, 2usize, 536u32, 2113668u32)?;
    emu.lbu_no_count(13usize, 2usize, 376u32, 2113672u32);
    emu.sw_no_count(0usize, 2usize, 508u32, 2113676u32)?;
    emu.sw_no_count(0usize, 2usize, 512u32, 2113680u32)?;
    emu.sw_no_count(0usize, 2usize, 516u32, 2113684u32)?;
    emu.sw_no_count(0usize, 2usize, 520u32, 2113688u32)?;
    emu.adi_no_count(10usize, 2usize, 476u32, 2113692u32);
    emu.adi_no_count(11usize, 2usize, 508u32, 2113696u32);
    emu.adi_no_count(12usize, 2usize, 344u32, 2113700u32);
    emu.apc_no_count(1usize, 2113700u32, 57344u32, 2113704u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113708u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002040ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 428u32, 2113712u32)?;
    emu.lw_no_count(11usize, 2usize, 432u32, 2113716u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2113720u32)?;
    emu.lw_no_count(13usize, 2usize, 440u32, 2113724u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2113728u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2113732u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2113736u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2113740u32)?;
    emu.lw_no_count(10usize, 2usize, 412u32, 2113744u32)?;
    emu.lw_no_count(11usize, 2usize, 416u32, 2113748u32)?;
    emu.lw_no_count(12usize, 2usize, 420u32, 2113752u32)?;
    emu.lw_no_count(13usize, 2usize, 424u32, 2113756u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2113760u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2113764u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2113768u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2113772u32)?;
    emu.adi_no_count(10usize, 2usize, 540u32, 2113776u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2113780u32);
    emu.adi_no_count(12usize, 2usize, 412u32, 2113784u32);
    emu.apc_no_count(1usize, 2113784u32, 28672u32, 2113788u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113792u32;
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
#[inline]
pub fn block_0x00204100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 492u32, 2113796u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2113800u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2113804u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2113808u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2113812u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2113816u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2113820u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2113824u32)?;
    emu.lw_no_count(10usize, 2usize, 476u32, 2113828u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2113832u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2113836u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2113840u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2113844u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2113848u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2113852u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2113856u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2113860u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2113864u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2113868u32);
    emu.apc_no_count(1usize, 2113868u32, 28672u32, 2113872u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113876u32;
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
pub fn block_0x00204154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 76u32, 2113880u32);
    emu.adi_no_count(11usize, 2usize, 144u32, 2113884u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2113888u32);
    emu.apc_no_count(1usize, 2113888u32, 28672u32, 2113892u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113896u32;
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
#[inline]
pub fn block_0x00204168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967292u32, 2113900u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2113904u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2113908u32);
    emu.adi_no_count(13usize, 0usize, 4294967295u32, 2113912u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2113916u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2113920u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2113924u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2113928u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2113932u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2113936u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2113940u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2113944u32)?;
    emu.adi_no_count(10usize, 2usize, 144u32, 2113948u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2113952u32);
    emu.adi_no_count(12usize, 2usize, 476u32, 2113956u32);
    emu.apc_no_count(1usize, 2113956u32, 28672u32, 2113960u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113964u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002041ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 212u32, 2113968u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2113972u32);
    emu.adi_no_count(12usize, 2usize, 144u32, 2113976u32);
    emu.apc_no_count(1usize, 2113976u32, 28672u32, 2113980u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2113984u32;
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
#[inline(never)]
pub fn block_0x002041c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(3694133248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2113988u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(75976704u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2113992u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3852607488u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2113996u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4146147328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2114000u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2901409792u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2114004u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2021928960u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2114008u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3634159616u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2114012u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1565u32, 2114016u32);
    emu.adi_no_count(11usize, 11usize, 4294965300u32, 2114020u32);
    emu.adi_no_count(12usize, 12usize, 171u32, 2114024u32);
    emu.adi_no_count(13usize, 13usize, 4294966998u32, 2114028u32);
    emu.sw_no_count(13usize, 2usize, 20u32, 2114032u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2114036u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2114040u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2114044u32)?;
    let a = 0u32.wrapping_add(700760064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2114048u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 1485u32, 2114052u32);
    emu.adi_no_count(12usize, 15usize, 144u32, 2114056u32);
    emu.adi_no_count(13usize, 16usize, 4294967138u32, 2114060u32);
    emu.adi_no_count(10usize, 10usize, 4294966751u32, 2114064u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2114068u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2114072u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2114076u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2114080u32)?;
    emu.adi_no_count(10usize, 2usize, 76u32, 2114084u32);
    emu.adi_no_count(11usize, 2usize, 212u32, 2114088u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2114092u32);
    emu.apc_no_count(1usize, 2114092u32, 28672u32, 2114096u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966832u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 36u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2114104u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2114108u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2114112u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2114116u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2114120u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2114124u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2114128u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2114132u32)?;
    emu.lw_no_count(10usize, 2usize, 492u32, 2114136u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2114140u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2114144u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2114148u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2114152u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2114156u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2114160u32)?;
    emu.sw_no_count(13usize, 2usize, 32u32, 2114164u32)?;
    emu.lw_no_count(10usize, 2usize, 428u32, 2114168u32)?;
    emu.lw_no_count(11usize, 2usize, 432u32, 2114172u32)?;
    emu.lw_no_count(12usize, 2usize, 436u32, 2114176u32)?;
    emu.lw_no_count(13usize, 2usize, 440u32, 2114180u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2114184u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2114188u32)?;
    emu.sw_no_count(12usize, 2usize, 60u32, 2114192u32)?;
    emu.sw_no_count(13usize, 2usize, 64u32, 2114196u32)?;
    emu.lw_no_count(10usize, 2usize, 412u32, 2114200u32)?;
    emu.lw_no_count(11usize, 2usize, 416u32, 2114204u32)?;
    emu.lw_no_count(12usize, 2usize, 420u32, 2114208u32)?;
    emu.lw_no_count(13usize, 2usize, 424u32, 2114212u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2114216u32)?;
    emu.sw_no_count(11usize, 2usize, 40u32, 2114220u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2114224u32)?;
    emu.sw_no_count(13usize, 2usize, 48u32, 2114228u32)?;
    emu.adi_no_count(10usize, 2usize, 540u32, 2114232u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2114236u32);
    emu.apc_no_count(1usize, 2114236u32, 57344u32, 2114240u32);
    emu.add_memory_rw_events(36usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 376u32, 2114248u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2114252u32);
    emu.apc_no_count(1usize, 2114252u32, 28672u32, 2114256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 308u32, 2114264u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2114268u32);
    emu.apc_no_count(1usize, 2114268u32, 28672u32, 2114272u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2114280u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2114284u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2114288u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114292u32);
    emu.apc_no_count(1usize, 2114292u32, 24576u32, 2114296u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002042fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(0usize, 8usize, 64u32, 2114304u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2114308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2114572u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020440c));
}
#[inline(always)]
pub fn block_0x00204304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 448u32, 2114312u32)?;
    emu.sbr_no_count(11usize, 11usize, 12usize, 2114316u32);
    emu.sltru_no_count(12usize, 11usize, 13usize, 2114320u32);
    emu.lw_no_count(11usize, 2usize, 520u32, 2114324u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2113080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e38));
    } else {
        emu.pc = 2114328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204318));
    }
}
#[inline(always)]
pub fn block_0x00204318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2114332u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2114336u32);
    emu.slti_no_count(12usize, 10usize, 0u32, 2114340u32);
    emu.lw_no_count(10usize, 2usize, 524u32, 2114344u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2113100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e4c));
    } else {
        emu.pc = 2114348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020432c));
    }
}
#[inline(always)]
pub fn block_0x0020432c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2114352u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2114356u32);
    emu.slti_no_count(12usize, 11usize, 0u32, 2114360u32);
    emu.lw_no_count(11usize, 2usize, 528u32, 2114364u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2113120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e60));
    } else {
        emu.pc = 2114368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204340));
    }
}
#[inline(always)]
pub fn block_0x00204340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2114372u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2114376u32);
    emu.slti_no_count(12usize, 10usize, 0u32, 2114380u32);
    emu.lw_no_count(10usize, 2usize, 532u32, 2114384u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2113140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e74));
    } else {
        emu.pc = 2114388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204354));
    }
}
#[inline(always)]
pub fn block_0x00204354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 11usize, 12usize, 2114392u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2114396u32);
    emu.slti_no_count(13usize, 11usize, 0u32, 2114400u32);
    emu.lw_no_count(11usize, 2usize, 536u32, 2114404u32)?;
    emu.lw_no_count(12usize, 2usize, 472u32, 2114408u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2113164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203e8c));
    } else {
        emu.pc = 2114412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020436c));
    }
}
#[inline(always)]
pub fn block_0x0020436c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 13usize, 2114416u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2114420u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2114424u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2114424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204378));
}
#[inline(always)]
pub fn block_0x00204378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(13usize, 11usize, 12usize, 2114428u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2114432u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2114436u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2114440u32);
    emu.sbr_no_count(10usize, 11usize, 10usize, 2114444u32);
    emu.apc_no_count(1usize, 2114444u32, 81920u32, 2114448u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114452u32;
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
#[inline]
pub fn block_0x00204394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 212u32, 2114456u32)?;
    emu.lw_no_count(12usize, 2usize, 216u32, 2114460u32)?;
    emu.lw_no_count(13usize, 2usize, 220u32, 2114464u32)?;
    emu.lw_no_count(14usize, 2usize, 224u32, 2114468u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2114472u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2114476u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2114480u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2114484u32)?;
    emu.lw_no_count(11usize, 2usize, 228u32, 2114488u32)?;
    emu.lw_no_count(12usize, 2usize, 232u32, 2114492u32)?;
    emu.lw_no_count(13usize, 2usize, 236u32, 2114496u32)?;
    emu.lw_no_count(14usize, 2usize, 240u32, 2114500u32)?;
    emu.sw_no_count(11usize, 2usize, 92u32, 2114504u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2114508u32)?;
    emu.sw_no_count(13usize, 2usize, 100u32, 2114512u32)?;
    emu.sw_no_count(14usize, 2usize, 104u32, 2114516u32)?;
    emu.adi_no_count(11usize, 2usize, 108u32, 2114520u32);
    emu.ani_no_count(13usize, 10usize, 255u32, 2114524u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2114528u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2114532u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2114536u32);
    emu.apc_no_count(1usize, 2114536u32, 57344u32, 2114540u32);
    emu.add_memory_rw_events(23usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114544u32;
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
pub fn block_0x002043f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 72u32, 2114548u32);
    emu.sb_no_count(19usize, 2usize, 140u32, 2114552u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2114556u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2114556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002043fc));
}
#[inline(always)]
pub fn block_0x002043fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 68u32, 2114560u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2114564u32);
    emu.apc_no_count(1usize, 2114564u32, 24576u32, 2114568u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020440c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 68u32, 2114576u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2114576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00204410));
}
#[inline]
pub fn block_0x00204410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 604u32, 2114580u32)?;
    emu.lw_no_count(8usize, 2usize, 600u32, 2114584u32)?;
    emu.lw_no_count(9usize, 2usize, 596u32, 2114588u32)?;
    emu.lw_no_count(18usize, 2usize, 592u32, 2114592u32)?;
    emu.lw_no_count(19usize, 2usize, 588u32, 2114596u32)?;
    emu.lw_no_count(20usize, 2usize, 584u32, 2114600u32)?;
    emu.lw_no_count(21usize, 2usize, 580u32, 2114604u32)?;
    emu.lw_no_count(22usize, 2usize, 576u32, 2114608u32)?;
    emu.lw_no_count(23usize, 2usize, 572u32, 2114612u32)?;
    emu.adi_no_count(2usize, 2usize, 608u32, 2114616u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114620u32;
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
pub fn block_0x0020443c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 7u32, 2114624u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2114628u32)?;
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2114632u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966240u32, 2114636u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2114640u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966224u32, 2114644u32);
    let a = 0u32.wrapping_add(2265088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2114648u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966252u32, 2114652u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2114656u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2114660u32);
    emu.apc_no_count(1usize, 2114660u32, 118784u32, 2114664u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2114668u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
